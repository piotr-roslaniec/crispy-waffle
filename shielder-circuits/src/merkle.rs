use std::marker::PhantomData;

use halo2_poseidon::poseidon::{
    primitives::{generate_constants, ConstantLength, Mds, Spec},
    Hash, Pow5Chip, Pow5Config,
};
use halo2_proofs::{
    arithmetic::Field,
    circuit::{AssignedCell, Chip, Layouter, Region, Value},
    dev::MockProver,
    halo2curves::bn256::{Bn256, Fr, G1Affine},
    plonk::{
        create_proof, keygen_pk, keygen_vk, Advice, Circuit, Column,
        ConstraintSystem, Error, Expression, ProvingKey, Selector,
        VerifyingKey,
    },
    poly::{
        commitment::ParamsProver,
        kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG},
            multiopen::ProverSHPLONK,
        },
        Rotation,
    },
    standard_plonk::StandardPlonk,
    transcript::{Blake2bWrite, Challenge255, TranscriptWriterBuffer},
};
use halo2curves::ff::PrimeField;
use rand_core::OsRng;

#[derive(Debug, Clone, Copy)]
pub struct PoseidonSpec<const WIDTH: usize, const RATE: usize>;

impl<const WIDTH: usize, const RATE: usize> Spec<Fr, WIDTH, RATE>
    for PoseidonSpec<WIDTH, RATE>
{
    fn full_rounds() -> usize {
        8
    }

    fn partial_rounds() -> usize {
        56
    }

    fn sbox(val: Fr) -> Fr {
        val.pow_vartime([5])
    }

    fn secure_mds() -> usize {
        0
    }

    fn constants() -> (Vec<[Fr; WIDTH]>, Mds<Fr, WIDTH>, Mds<Fr, WIDTH>) {
        generate_constants::<_, Self, WIDTH, RATE>()
    }
}

pub struct SelectChip<F: Field> {
    config: SelectConfig<F>,
}

#[derive(Clone, Debug)]
pub struct SelectConfig<F: Field> {
    pub advices: [Column<Advice>; 4],
    pub selector: Selector,
    _field: PhantomData<F>,
}

impl<F: Field> Chip<F> for SelectChip<F> {
    type Config = SelectConfig<F>;

    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: Field> SelectChip<F> {
    pub fn new(config: SelectConfig<F>) -> Self {
        Self { config }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        advices: [Column<Advice>; 4],
    ) -> <Self as Chip<F>>::Config {
        for advice in &advices {
            meta.enable_equality(*advice);
        }
        let selector = meta.selector();

        meta.create_gate("select", |meta| {
            let a = meta.query_advice(advices[0], Rotation::cur());
            let b = meta.query_advice(advices[1], Rotation::cur());
            let out = meta.query_advice(advices[2], Rotation::cur());
            let condition = meta.query_advice(advices[3], Rotation::cur());

            let one = Expression::Constant(F::ONE);
            let selector = meta.query_selector(selector);

            vec![
                selector.clone()
                    * (condition.clone() * (one - condition.clone())),
                selector * ((a - b.clone()) * condition + b - out),
            ]
        });
        SelectConfig {
            advices,
            selector,
            _field: PhantomData,
        }
    }

    pub fn synthesize(
        &self,
        layouter: &mut impl Layouter<F>,
        condition: AssignedCell<F, F>,
        a: AssignedCell<F, F>,
        b: AssignedCell<F, F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        let config = self.config();

        layouter.assign_region(
            || "ternary operator",
            |mut region: Region<'_, F>| {
                config.selector.enable(&mut region, 0)?;

                a.copy_advice(|| "copy a", &mut region, config.advices[0], 0)?;
                b.copy_advice(|| "copy b", &mut region, config.advices[1], 0)?;

                let condition = condition.copy_advice(
                    || "copy condition",
                    &mut region,
                    config.advices[3],
                    0,
                )?;

                region.assign_advice(
                    || "select option",
                    config.advices[2],
                    0,
                    || {
                        condition
                            .value()
                            .copied()
                            .to_field()
                            .zip(a.value())
                            .zip(b.value())
                            .map(
                                |((condition, a), b)| {
                                    if condition == F::ONE.into() {
                                        a
                                    } else {
                                        b
                                    }
                                },
                            )
                            .copied()
                    },
                )
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct MerkleProofChip<F: PrimeField, const TREE_HEIGHT: usize> {
    config: MerkleProofConfig<F, TREE_HEIGHT>,
    _hasher: PhantomData<F>,
}

#[derive(Clone, Debug)]
pub struct MerkleProofConfig<F: PrimeField, const TREE_HEIGHT: usize> {
    pub advices: [Column<Advice>; TREE_HEIGHT],
    pub selector: Selector,
    pub select_config: SelectConfig<F>,
    pub poseidon_config: Pow5Config<F, 3, 2>,
}

impl<F: PrimeField, const TREE_HEIGHT: usize> Chip<F>
    for MerkleProofChip<F, TREE_HEIGHT>
{
    type Config = MerkleProofConfig<F, TREE_HEIGHT>;

    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<const TREE_HEIGHT: usize> MerkleProofChip<Fr, TREE_HEIGHT> {
    fn new(config: MerkleProofConfig<Fr, TREE_HEIGHT>) -> Self {
        Self {
            config,
            _hasher: PhantomData,
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<Fr>,
        advices: [Column<Advice>; TREE_HEIGHT],
        poseidon_config: Pow5Config<Fr, 3, 2>,
        select_config: SelectConfig<Fr>,
    ) -> <Self as Chip<Fr>>::Config {
        for advice in &advices {
            meta.enable_equality(*advice);
        }
        let selector = meta.selector();

        MerkleProofConfig {
            advices,
            selector,
            select_config,
            poseidon_config,
        }
    }

    fn synthesize(
        &self,
        layouter: &mut impl Layouter<Fr>,
        leaf: AssignedCell<Fr, Fr>,
        path: [AssignedCell<Fr, Fr>; HEIGHT],
        path_shape: [AssignedCell<Fr, Fr>; HEIGHT],
        root: AssignedCell<Fr, Fr>,
    ) -> Result<(), Error> {
        let config = self.config();

        let mut note = leaf;

        let select_chip = SelectChip::new(config.select_config.clone());

        for (sibling, is_left) in path.into_iter().zip(path_shape.into_iter()) {
            let left = select_chip.synthesize(
                layouter,
                is_left.clone(),
                note.clone(),
                sibling.clone(),
            )?;
            let right =
                select_chip.synthesize(layouter, is_left, sibling, note)?;

            let poseidon_chip =
                Pow5Chip::construct(config.poseidon_config.clone());
            let hasher: Hash<
                Fr,
                Pow5Chip<Fr, 3, 2>,
                PoseidonSpec<3, 2>,
                ConstantLength<2>,
                3,
                2,
            > = Hash::init(poseidon_chip, layouter.namespace(|| "Hash init"))?;
            note = hasher.hash(layouter.namespace(|| "Hash"), [left, right])?;
        }

        println!("Note: {:?}", note);
        println!("Root: {:?}", root);
        layouter.assign_region(
            || "Proof verification",
            |mut region: Region<'_, Fr>| {
                region.constrain_equal(note.cell(), root.cell())
            },
        )
    }
}

const HEIGHT: usize = 20;

#[derive(Clone, Debug)]
pub struct TestCircuit {
    root: Value<Fr>,
    path: [Value<Fr>; HEIGHT],
    path_shape: [Value<Fr>; HEIGHT],
    leaf: Value<Fr>,
}

impl Circuit<Fr> for TestCircuit {
    type Config = MerkleProofConfig<Fr, 22>;
    type FloorPlanner = <StandardPlonk as Circuit<Fr>>::FloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            root: Value::unknown(),
            path: [Value::unknown(); HEIGHT],
            path_shape: [Value::unknown(); HEIGHT],
            leaf: Value::unknown(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        let advices = [(); HEIGHT + 2].map(|_| meta.advice_column());
        let poseidon_advices = [(); 4].map(|_| meta.advice_column());
        let select_advices = [(); 4].map(|_| meta.advice_column());

        for advice in &advices {
            meta.enable_equality(*advice);
        }

        for advice in &poseidon_advices {
            meta.enable_equality(*advice);
        }

        for advice in &select_advices {
            meta.enable_equality(*advice);
        }

        let col_const = meta.fixed_column();
        meta.enable_constant(col_const);

        let rc_a = [(); 3].map(|_| meta.fixed_column());
        let rc_b = [(); 3].map(|_| meta.fixed_column());

        let poseidon_config = Pow5Chip::configure::<PoseidonSpec<3, 2>>(
            meta,
            poseidon_advices[1..4].try_into().unwrap(),
            poseidon_advices[0],
            rc_a,
            rc_b,
        );

        let select_config = SelectChip::configure(meta, select_advices);

        MerkleProofChip::<Fr, 22>::configure(
            meta,
            advices,
            poseidon_config,
            select_config,
        )
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fr>,
    ) -> Result<(), Error> {
        // Initialize the Path chip
        let path_chip = MerkleProofChip::new(config.clone());

        // Witness values
        let (root, path, path_shape, leaf) = layouter.assign_region(
            || "witness",
            |mut region| {
                let root = region.assign_advice(
                    || "witness root",
                    config.advices[20],
                    0,
                    || self.root,
                )?;

                let path: [AssignedCell<Fr, Fr>; HEIGHT] = (0..HEIGHT)
                    .map(|i| {
                        region
                            .assign_advice(
                                || "witness root",
                                config.advices[i],
                                0,
                                || self.path[i],
                            )
                            .unwrap()
                    })
                    .collect::<Vec<AssignedCell<Fr, Fr>>>()
                    .try_into()
                    .unwrap();

                let path_shape: [AssignedCell<Fr, Fr>; HEIGHT] = (0..HEIGHT)
                    .map(|i| {
                        region
                            .assign_advice(
                                || "witness root",
                                config.advices[i],
                                1,
                                || self.path_shape[i],
                            )
                            .unwrap()
                    })
                    .collect::<Vec<AssignedCell<Fr, Fr>>>()
                    .try_into()
                    .unwrap();

                let leaf = region.assign_advice(
                    || "witness leaf",
                    config.advices[21],
                    0,
                    || self.leaf,
                )?;

                Ok((root, path, path_shape, leaf))
            },
        )?;

        path_chip.synthesize(&mut layouter, leaf, path, path_shape, root)?;

        Ok(())
    }
}

// Generates an empty circuit. Useful for generating the proving/verifying keys.
pub fn empty_circuit() -> TestCircuit {
    TestCircuit {
        root: Value::unknown(),
        path: [Value::unknown(); HEIGHT],
        path_shape: [Value::unknown(); HEIGHT],
        leaf: Value::unknown(),
    }
}

// Creates a circuit from two vector inputs
pub fn create_circuit(
    root: Fr,
    leaf: Fr,
    path: [Fr; HEIGHT],
    path_shape: [Fr; HEIGHT],
) -> TestCircuit {
    // Put inputs into circuit-friendly form
    let root = Value::known(root);
    let leaf = Value::known(leaf);

    let path = path
        .into_iter()
        .map(Value::known)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let path_shape = path_shape
        .into_iter()
        .map(Value::known)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    // Create circuit from inputs
    TestCircuit {
        root,
        path,
        path_shape,
        leaf,
    }
}

// Generates setup parameters using k, which is the number of rows of the circuit
// can fit in and must be a power of two
pub fn generate_setup_params(k: u32) -> ParamsKZG<Bn256> {
    ParamsKZG::<Bn256>::new(k)
}

// Generates the verifying and proving keys. We can pass in an empty circuit to generate these
pub fn generate_keys(
    params: &ParamsKZG<Bn256>,
    circuit: &TestCircuit,
) -> (ProvingKey<G1Affine>, VerifyingKey<G1Affine>) {
    // just to emphasize that for vk, pk we don't need to know the value of `x`
    let vk = keygen_vk(params, circuit).expect("vk should not fail");
    let pk =
        keygen_pk(params, vk.clone(), circuit).expect("pk should not fail");
    (pk, vk)
}

// Runs the mock prover and prints any errors
pub fn run_mock_prover(k: u32, circuit: &TestCircuit, _pub_input: &[Fr]) {
    let prover =
        MockProver::run(k, circuit, vec![]).expect("Mock prover should run");
    let res = prover.verify();
    match res {
        Ok(()) => println!("MockProver OK"),
        Err(e) => println!("err {:#?}", e),
    }
}

// Generates a proof
pub fn generate_proof(
    params: &ParamsKZG<Bn256>,
    pk: &ProvingKey<G1Affine>,
    circuit: TestCircuit,
    _pub_input: &[Fr],
) -> Vec<u8> {
    println!("Generating proof...");
    let mut transcript: Blake2bWrite<_, _, _> =
        Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    println!(
        "{:?}",
        create_proof::<
            KZGCommitmentScheme<Bn256>,
            ProverSHPLONK<Bn256>,
            _,
            _,
            _,
            _,
        >(params, pk, &[circuit], &[&[]], OsRng, &mut transcript,)
    );

    transcript.finalize()
}

// Verifies the proof
// pub fn verify(
//     params: &Params<EqAffine>,
//     vk: &VerifyingKey<EqAffine>,
//     pub_input: &Vec<Fp>,
//     proof: Vec<u8>,
// ) -> Result<(), Error> {
//     println!("Verifying proof...");
//     let strategy = SingleVerifier::new(&params);
//     let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
//     verify_proof(
//         params,
//         vk,
//         strategy,
//         &[&[pub_input]],
//         &mut transcript,
//     )
// }
