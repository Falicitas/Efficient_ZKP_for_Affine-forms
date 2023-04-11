use crate::transcript::ProofTranscript;
use curve25519_dalek::ristretto::CompressedRistretto;
use merlin::Transcript;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use Efficient_ZKP_for_Affine_forms::{
    commitments::{DotProductProofGens, MultiCommitGens},
    curve25519::{
        errors::ProofVerifyError,
        group::{CompressedGroup, CompressedGroupExt, GROUP_BASEPOINT},
        scalar::Scalar,
        scalar_math,
    },
    nozk_protocol::{pi_1_protocol::Pi_1_Proof, pi_2_protocol::Pi_2_Proof},
    random::RandomTape,
    transcript,
    zk_protocol::{pi_0_protocol::Pi_0_Proof, pi_a_protocol::Pi_Affine_Proof},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Raw {
    m_matric: Vec<Vec<Scalar>>,
    b_vec: Vec<Scalar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct X {
    x_vec: Vec<Scalar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof_and_Commitments {
    proof: Pi_Affine_Proof,
    P: CompressedGroup,
    P_hat: CompressedGroup,
    y: Scalar,
}

fn singleton_test(suffix_path: String) {
    let raw_path = "./random_data/raw_".to_owned() + &suffix_path;
    let secret_path = "./random_data/private_".to_owned() + &suffix_path;
    let proof_path = "./random_data/proof_".to_owned() + &suffix_path;

    let disk_raw: Raw = serde_json::from_str(&fs::read_to_string(raw_path).unwrap()).unwrap();
    let disk_secret: X = serde_json::from_str(&fs::read_to_string(secret_path).unwrap()).unwrap();

    let (mut m_matric, b_vec) = (disk_raw.m_matric.clone(), disk_raw.b_vec.clone());
    let mut x_vec = disk_secret.x_vec.clone();

    assert!(m_matric.len() > 0);
    assert!(m_matric[0].len() > 0);

    let s = b_vec.len();

    assert_eq!(m_matric.len(), s);
    assert_eq!(x_vec.len(), m_matric[0].len());

    x_vec.push(-Scalar::one());

    for i in 0..s {
        m_matric[i].push(b_vec[i]);
    }

    let n = x_vec.len(); // 2^t - 1

    // FS for once-and-for-all proof

    let mut csprng: OsRng = OsRng;

    let gamma = Scalar::random(&mut csprng);

    let gens = DotProductProofGens::new(n, b"gens");

    let mut prover_random_tape = RandomTape::new(b"proof");
    let mut prover_transcript = Transcript::new(b"Kinesis's protocol");

    let (proof, P, P_hat, y) = Pi_Affine_Proof::prove(
        &gens,
        &mut prover_transcript,
        &mut prover_random_tape,
        &x_vec,
        &gamma,
        &m_matric,
    );

    fs::write(
        proof_path,
        serde_json::to_string(&Proof_and_Commitments { proof, P, P_hat, y }).unwrap(),
    )
    .unwrap();
}

fn main() {
    for i in 0..10 {
        singleton_test("1e9_10_".to_owned() + &i.to_string());
    }
}
