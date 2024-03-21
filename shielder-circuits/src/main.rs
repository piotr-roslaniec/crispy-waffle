use halo2_proofs::{
    halo2curves::bn256::{Bn256, Fr, G1Affine},
    plonk::verify_proof,
    poly::{
        commitment::Verifier,
        kzg::{
            commitment::KZGCommitmentScheme,
            multiopen::{ProverGWC, ProverSHPLONK, VerifierSHPLONK},
            strategy::SingleStrategy,
        },
        VerificationStrategy,
    },
    transcript::{Blake2bRead, Challenge255, TranscriptReadBuffer},
};
use halo2curves::ff::{Field, PrimeField};

fn main() {
    use shielder_circuits::merkle::{
        create_circuit, empty_circuit, generate_keys, generate_proof,
        generate_setup_params, run_mock_prover,
    };

    // Size of the circuit. Circuit must fit within 2^k rows.
    let k = 13;
    let root = Fr::from_str_vartime(
                "21070819810761031412485887399825884674609810661645526274842754985158693294840",
            )
            .unwrap();
    let leaf = Fr::ONE;
    let path = [Fr::ONE; 20];
    let path_shape = [Fr::ONE; 20];
    // Create circuit
    let test_circuit = create_circuit(root, leaf, path, path_shape);

    // Items that are useful for debugging issues
    run_mock_prover(k, &test_circuit, &vec![]);

    // Generate setup params
    let params = generate_setup_params(k);

    // Generate proving and verifying keys
    let empty_circuit = empty_circuit();
    let (pk, vk) = generate_keys(&params, &empty_circuit);

    // Generate proof
    let proof = generate_proof(&params, &pk, test_circuit, &vec![]);
    let strategy = SingleStrategy::new(&params);
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
    println!(
        "{:?}",
        verify_proof::<
            KZGCommitmentScheme<Bn256>,
            VerifierSHPLONK<Bn256>,
            _,
            _,
            _,
        >(&params, &vk, strategy, &[&[]], &mut transcript,)
    );
}
