pub mod hammster;
pub mod merkle;

#[cfg(feature = "wasm")]
pub mod wasm;

use std::io::BufReader;

use halo2_proofs::{
    halo2curves::bn256::{Bn256, Fr},
    poly::{commitment::Params, kzg::commitment::ParamsKZG},
};
use halo2curves::ff::{Field, PrimeField};

use crate::merkle::{
    create_circuit, empty_circuit, generate_keys, generate_proof,
    generate_setup_params,
};

pub fn setup_params(k: u32) -> Vec<u8> {
    // Generate setup params
    let params = generate_setup_params(k);
    let mut buf = vec![];
    params.write(&mut buf).expect("Can write params");
    buf
}

pub fn proof_generate(a: &[u8], b: &[u8], params_bytes: &[u8]) -> Vec<u8> {
    let params = ParamsKZG::<Bn256>::read(&mut BufReader::new(params_bytes))
        .expect("params should not fail to read");

    let (pk, _vk) = generate_keys(&params, &empty_circuit());

    let root = Fr::from_str_vartime(
                "21070819810761031412485887399825884674609810661645526274842754985158693294840",
            )
            .unwrap();
    let leaf = Fr::ONE;
    let path = [Fr::ONE; 20];
    let path_shape = [Fr::ONE; 20];

    // Generate proof
    let test_circuit = create_circuit(root, leaf, path, path_shape);
    generate_proof(&params, &pk, test_circuit, &vec![])
}

// TODO: Consider rewriting:
// #[wasm_bindgen]
// pub fn proof_verify(
//     params_bytes: &[u8],
//     hamming_dist: u32,
//     proof: &[u8]
// ) -> bool {
//     log("verifying...");
//
//     let params = Params::<EqAffine>::read(&mut BufReader::new(params_bytes)).expect("params should not fail to read");
//
//     // Generate verifying key
//     let empty_circuit = empty_circuit();
//     let vk = keygen_vk(&params, &empty_circuit).expect("vk should not fail to generate");
//
//     // Transform params for verify function
//     let hamming_dist_fp = vec![Fp::from(hamming_dist as u64)];
//     let proof_vec = proof.to_vec();
//
//     // Verify the proof and public input
//     let ret_val = verify(&params, &vk, &hamming_dist_fp, proof_vec);
//     match ret_val {
//         Err(_) => false,
//         _ => true,
//     }
// }
