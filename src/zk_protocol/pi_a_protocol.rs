use super::super::random::RandomTape;
use super::super::transcript::ProofTranscript;
use crate::commitments::DotProductProofGens;
use crate::curve25519::errors::ProofVerifyError;
use crate::curve25519::group::CompressedGroup;
use crate::curve25519::scalar::Scalar;
use crate::curve25519::scalar_math;
use crate::zk_protocol::pi_c_protocol::Pi_c_Proof;
use merlin::Transcript;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Pi_Affine_Proof {
    proof: Pi_c_Proof,
}
// different linear forms, same $\vec{x}$
impl Pi_Affine_Proof {
    fn protocol_name() -> &'static [u8] {
        b"affine form proof"
    }

    pub fn prove(
        gens: &DotProductProofGens, //? 外部
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
        x_vec: &[Scalar],
        gamma: &Scalar,
        l_matrix: &Vec<Vec<Scalar>>,
    ) -> (Pi_Affine_Proof, CompressedGroup, CompressedGroup, Scalar) {
        transcript.append_protocol_name(Pi_Affine_Proof::protocol_name());

        let n = x_vec.len();
        let s = l_matrix.len();
        assert_eq!(gens.gens_n.n, n);

        //? ???????
        let rho = transcript.challenge_scalar(b"rho");
        let rho_vec = scalar_math::vandemonde_challenge_one(rho, s);
        let l_matrix_t = scalar_math::matrix_transpose(&l_matrix);
        let l_vec = scalar_math::matrix_vector_mul(&l_matrix_t, &rho_vec);

        let y = scalar_math::compute_linearform(&l_vec, &x_vec);

        let (proof, P, P_hat) =
            Pi_c_Proof::prove(&gens, transcript, random_tape, x_vec, gamma, &l_vec, &y);

        (Pi_Affine_Proof { proof }, P, P_hat, y)
    }
}
