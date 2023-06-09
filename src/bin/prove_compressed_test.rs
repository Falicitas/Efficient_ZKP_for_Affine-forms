#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use merlin::Transcript;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use Efficient_ZKP_for_Affine_forms::{
    commitments::{Commitments, DotProductProofGens},
    curve25519::{group::CompressedGroup, scalar::Scalar},
    random::RandomTape,
    runtime,
    transcript::AppendToTranscript,
    zk_protocol::pi_a_protocol::Pi_Affine_Proof,
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

impl Proof_and_Commitments {
    pub fn siz(&self) -> usize {
        self.proof.siz()
    }
}

fn singleton_test(suffix_path: String) {
    //TODO: Compress FS using the seed of transcript
    use std::time::Instant;
    let mut now = Instant::now();

    let infix_path = "./random_data/1e9_10000_5000/".to_owned();

    let raw_path = infix_path.to_owned() + &"raw_".to_owned() + &suffix_path;
    let private_path = infix_path.to_owned() + &"private_".to_owned() + &suffix_path;
    let proof_path = infix_path.to_owned() + &"proof_".to_owned() + &suffix_path;

    let disk_raw: Raw = serde_json::from_str(&fs::read_to_string(raw_path).unwrap()).unwrap();
    let disk_secret: X = serde_json::from_str(&fs::read_to_string(private_path).unwrap()).unwrap();

    runtime::print_runtime(&mut now, "", "input/parse");

    // std output unit for speed test

    let (mut m_matric, b_vec) = (disk_raw.m_matric.clone(), disk_raw.b_vec.clone());
    let mut x_vec = disk_secret.x_vec.clone();

    runtime::print_runtime(&mut now, "", "clone");

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

    let mut csprng: OsRng = OsRng;

    let gens = DotProductProofGens::new(n, b"gens");

    // Setup stage complete.

    let gamma = Scalar::random(&mut csprng);

    runtime::print_runtime(&mut now, "", "gens generating");

    let mut prover_random_tape = RandomTape::new(b"proof");
    let mut prover_transcript = Transcript::new(b"Kinesis's protocol");

    let P_secure = x_vec.commit(&gamma, &gens.gens_n).compress();
    P_secure.append_to_transcript(b"P_secure", &mut prover_transcript);

    let (proof, P, P_hat, y) = Pi_Affine_Proof::prove(
        &gens,
        &mut prover_transcript,
        &mut prover_random_tape,
        &x_vec,
        &gamma,
        &m_matric,
    );

    runtime::print_runtime(&mut now, "", "proof running");

    println!("proof size: {} bits", proof.siz() * 8);

    fs::write(
        proof_path,
        serde_json::to_string(&Proof_and_Commitments { proof, P, P_hat, y }).unwrap(),
    )
    .unwrap();
}

fn main() {
    for i in 0..1 {
        singleton_test(i.to_string() + &".in".to_owned());
    }
}
