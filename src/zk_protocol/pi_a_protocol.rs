use super::super::random::RandomTape;
use super::super::transcript::ProofTranscript;
use crate::commitments::{Commitments, DotProductProofGens};
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
        transcript.append_protocol_name(Pi_Affine_Proof::protocol_name());

        use std::time::Instant;
        let now = Instant::now();

        let n = x_vec.len();
        let s = l_matric.len();
        assert_eq!(gens.gens_n.n, n);

        let rho = transcript.challenge_scalar(b"rho");
        let rho_vec = scalar_math::vandemonde_challenge_one(rho, s);
        let l_matric_t = scalar_math::matrix_transpose(&l_matric);
        let l_vec = scalar_math::matrix_vector_mul(&l_matric_t, &rho_vec);

        let y = scalar_math::compute_linearform(&l_vec, &x_vec);

        //? >>>in-------------------------------test running time-----------------------------------------------------
        let duration = now.elapsed();
        let (s, mut ms, mut us) = (
            duration.as_secs(),
            duration.as_millis(),
            duration.as_micros(),
        );
        us -= ms * 1000;
        ms -= s as u128 * 1000;
        println!(
            "Hi! compressed L_vec running time: {} s {} ms {} us",
            s, ms, us
        );
        //? <<out-------------------------------test running time-----------------------------------------------------

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
