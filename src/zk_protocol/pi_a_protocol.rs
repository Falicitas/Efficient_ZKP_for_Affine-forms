use super::super::random::RandomTape;
use super::super::runtime;
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
pub struct Pi_Affine_Proof {
    proof: Pi_c_Proof,
}

impl Pi_Affine_Proof {
    pub fn siz(&self) -> usize {
        self.proof.siz()
    }

    fn protocol_name() -> &'static [u8] {
        b"zk pi_affine proof"
    }

    pub fn prove(
        gens: &DotProductProofGens,
        transcript: &mut Transcript,
        prover_random_tape: &mut RandomTape,
        x_vec: &[Scalar],
        gamma: &Scalar,
        l_matric: &Vec<Vec<Scalar>>,
    ) -> (Pi_Affine_Proof, CompressedGroup, CompressedGroup, Scalar) {
        use std::time::Instant;
        let mut now = Instant::now();

        transcript.append_protocol_name(Pi_Affine_Proof::protocol_name());

        let n = x_vec.len();
        let s = l_matric.len();
        assert_eq!(gens.gens_n.n, n);

        let rho = transcript.challenge_scalar(b"rho");

        runtime::print_runtime(&mut now, ">> ", "generate challenge rho");

        let rho_vec = scalar_math::vandemonde_challenge_one(rho, s);

        runtime::print_runtime(&mut now, ">> ", "generate rho_vec");

        let l_matric_t = scalar_math::matrix_transpose(&l_matric);

        runtime::print_runtime(&mut now, ">> ", "transpose");

        let l_vec = scalar_math::matrix_vector_mul(&l_matric_t, &rho_vec);

        runtime::print_runtime(&mut now, ">> ", "M * rho_vec");

        let y = scalar_math::compute_linearform(&l_vec, &x_vec);

        runtime::print_runtime(&mut now, ">> ", "compressed L_vec");

        let (proof, P, P_hat) = Pi_c_Proof::prove(
            &gens,
            transcript,
            prover_random_tape,
            x_vec,
            gamma,
            &l_vec,
            &y,
        );

        (Pi_Affine_Proof { proof }, P, P_hat, y)
    }

    pub fn verify(
        &self,
        n: usize,
        gens: &DotProductProofGens,
        transcript: &mut Transcript,
        l_matric: &Vec<Vec<Scalar>>,
        P: &CompressedGroup,
        y: &Scalar,
        P_hat: &CompressedGroup,
    ) -> Result<(), ProofVerifyError> {
        assert!(gens.gens_n.n >= n);

        transcript.append_protocol_name(Pi_Affine_Proof::protocol_name());

        let s = l_matric.len();
        let rho = transcript.challenge_scalar(b"rho");
        let rho_vec = scalar_math::vandemonde_challenge_one(rho, s);
        let l_matrix_t = scalar_math::matrix_transpose(&l_matric);
        let l_vec = scalar_math::matrix_vector_mul(&l_matrix_t, &rho_vec);

        return self
            .proof
            .verify(n, &gens, transcript, &l_vec, &P, &y, &P_hat);
    }
}
