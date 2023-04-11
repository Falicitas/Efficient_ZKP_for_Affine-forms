use crate::curve25519::errors::ProofVerifyError;
use crate::curve25519::group::{
    CompressedGroup, CompressedGroupExt, GroupElement, VartimeMultiscalarMul,
    GROUP_BASEPOINT_COMPRESSED,
};
use crate::curve25519::scalar::Scalar;
use crate::curve25519::scalar_math;
use digest::{ExtendableOutput, Input};
use lazy_static;
use merlin::Transcript;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use sha3::Shake256;
use std::fs;
//TODO: rewrite as input_trait
#[derive(Debug, Serialize, Deserialize)]
pub struct Input_ {
    n: usize,
    s: usize,
    M_matric: Vec<Vec<Scalar>>,
    b_vec: Vec<Scalar>,
    x_vec: Vec<Scalar>,
}

//no need for change
fn file_input() -> Input_ {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <target input file>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];
    let prover_input: Input_ = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();

    input_check(&prover_input);

    prover_input
}

fn input_check(prover_input: &Input_) {
    let s_ = prover_input.M_matric.len();
    assert!(s_ > 0);
    for i in 0..s_ - 1 {
        assert_eq!(
            prover_input.M_matric[i].len(),
            prover_input.M_matric[i + 1].len()
        );
    }

    let n_ = prover_input.M_matric[0].len();
    assert_eq!(prover_input.n, n_);
    assert_eq!(prover_input.s, s_);
    assert_eq!(prover_input.b_vec.len(), s_);
    assert_eq!(prover_input.x_vec.len(), n_);
}

fn input_init(input: &Input_) -> (usize, usize, Vec<Vec<Scalar>>, Vec<Scalar>, Vec<Scalar>) {
    (
        input.n,
        input.s,
        input.M_matric.clone(), //TODO: 二维 clone 会出问题吗
        input.b_vec.clone(),
        input.x_vec.clone(),
    )
}

pub fn new_gens() {
    let mut shake = Shake256::default();
    shake.input("label");
    shake.input(GROUP_BASEPOINT_COMPRESSED.as_bytes());

    let mut reader = shake.xof_result();
    let mut gens: Vec<GroupElement> = Vec::new();
    let mut uniform_bytes = [0u8; 64];
    for _ in 0..256 {
        gens.push(GroupElement::from_uniform_bytes(&uniform_bytes));
    }
}

pub mod Pub_Param_Affine_forms {
    use crate::curve25519::scalar::Scalar;
    #[macro_use]
    lazy_static::lazy_static! {

        pub static ref INPUT: super::Input_ = super::file_input();

        pub static ref INIT: (usize, usize, Vec<Vec<Scalar>>, Vec<Scalar>, Vec<Scalar>) = super::input_init(&INPUT);
        // pub static ref gens:
    }
}
