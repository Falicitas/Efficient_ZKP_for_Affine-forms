use super::super::commitments::{Commitments, DotProductProofGens, MultiCommitGens};
use super::super::random::RandomTape;
use super::super::transcript::{AppendToTranscript, ProofTranscript};
use crate::curve25519::errors::ProofVerifyError;
use crate::curve25519::group::{
    CompressedGroup, CompressedGroupExt, GroupElement, VartimeMultiscalarMul,
};
use crate::curve25519::scalar::Scalar;
use crate::curve25519::scalar_math;
use crate::math::Math;
use crate::nozk_protocol::bullet_proof::BulletReductionProof;
use crate::transcript;
use merlin::Transcript;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pi_2_Proof {
    bullet_reduction_proof: BulletReductionProof,
}

impl Pi_2_Proof {
    fn protocol_name() -> &'static [u8] {
        b"pi_2 proof"
    }

    pub fn mod_prove(
        // gens_n: &MultiCommitGens,
        // gens_1: &MultiCommitGens,
        transcript: &mut Transcript,
        L_tilde: &[Scalar],
        z_hat: &[Scalar],
    ) -> (Pi_2_Proof, CompressedGroup) {
        transcript.append_protocol_name(Pi_2_Proof::protocol_name());

        let n = z_hat.len();
        assert_eq!(L_tilde.len(), n);

        let k = &gens_1.G[0];
        let Q = ((GroupElement::vartime_multiscalar_mul(z_hat, &gens_n.G))
            + k * scalar_math::compute_linearform(L_tilde, z_hat))
        .compress();

        Q.append_to_transcript(b"Q", transcript);

        (
            Pi_2_Proof {
                bullet_reduction_proof: BulletReductionProof::prove(
                    transcript, k, &gens_n.G, z_hat, L_tilde,
                ),
            },
            Q,
        )
    }
}
