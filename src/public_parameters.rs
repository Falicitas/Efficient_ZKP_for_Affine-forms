use crate::curve25519::errors::ProofVerifyError;
use crate::curve25519::group::{
    CompressedGroup, CompressedGroupExt, GroupElement, VartimeMultiscalarMul,
};
use crate::curve25519::scalar::Scalar;
use crate::curve25519::scalar_math;
use merlin::Transcript;
use serde::{Deserialize, Serialize};

use lazy_static;

pub fn yyy() -> String {
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    line
}

pub mod Pub_Param_Affine_forms {
    #[macro_use]
    lazy_static::lazy_static! {
    // #[derive(Debug)]
            pub static ref DT: String = super::yyy();
        }
}
