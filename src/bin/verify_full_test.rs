use merlin::Transcript;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use Efficient_ZKP_for_Affine_forms::{
    commitments::DotProductProofGens,
    curve25519::{group::CompressedGroup, scalar::Scalar},
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

fn singleton_test(suffix_path: String) {
    use std::time::Instant;

    let now = Instant::now();

    let infix_path = "./random_data/1e9_1000_5".to_owned();

    let raw_path = infix_path.to_owned() + &"/raw_".to_owned() + &suffix_path;

    let proof_path = infix_path.to_owned() + &"/proof_".to_owned() + &suffix_path;

    let disk_raw: Raw = serde_json::from_str(&fs::read_to_string(raw_path).unwrap()).unwrap();
    let disk_proof: Proof_and_Commitments =
        serde_json::from_str(&fs::read_to_string(proof_path).unwrap()).unwrap();

    //? >>>in-------------------------------test running time-----------------------------------------------------
    let duration = now.elapsed();
    let s = duration.as_secs();
    let mut ms = duration.as_millis();
    let mut us = duration.as_micros();
    us -= ms * 1000;
    ms -= s as u128 * 1000;
    println!("Hi! input running time: {} s {} ms {} us", s, ms, us);
    let now = Instant::now();
    //? <<out-------------------------------test running time-----------------------------------------------------

    let (mut m_matric, b_vec) = (disk_raw.m_matric, disk_raw.b_vec.clone());
    let (proof, P, P_hat, y) = (
        disk_proof.proof,
        disk_proof.P,
        disk_proof.P_hat,
        disk_proof.y,
    );

    assert!(m_matric.len() > 0);
    assert!(m_matric[0].len() > 0);

    let s = b_vec.len();

    assert_eq!(m_matric.len(), s);

    for i in 0..s {
        m_matric[i].push(b_vec[i]);
    }

    let n = m_matric[0].len();

    // FS for once-and-for-all proof

    let gens = DotProductProofGens::new(n, b"gens");

    let mut verifier_transcript = Transcript::new(b"Kinesis's protocol");

    let P_secure = P;
    P_secure.append_to_transcript(b"P_secure", &mut verifier_transcript);

    assert!(proof
        .verify(
            n,
            &gens,
            &mut verifier_transcript,
            &m_matric,
            &P,
            &y,
            &P_hat
        )
        .is_ok());

    //? >>>in-------------------------------test running time-----------------------------------------------------
    let duration = now.elapsed();
    let s = duration.as_secs();
    let mut ms = duration.as_millis();
    let mut us = duration.as_micros();
    us -= ms * 1000;
    ms -= s as u128 * 1000;
    println!("Hi! verify running time: {} s {} ms {} us", s, ms, us);
    //? <<out-------------------------------test running time-----------------------------------------------------
}

fn main() {
    for i in 0..10 {
        println!("----------------------------------------singleton_test----------------------------------------");
        singleton_test(i.to_string() + &".in".to_owned());
        println!("test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured.");
    }
}
