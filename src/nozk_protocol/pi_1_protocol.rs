use super::super::commitments::MultiCommitGens;
use crate::curve25519::group::{CompressedGroup, GroupElement, VartimeMultiscalarMul};
use crate::curve25519::scalar::Scalar;
use crate::curve25519::scalar_math;
use crate::transcript::{AppendToTranscript, ProofTranscript};
use merlin::Transcript;
use serde::{Deserialize, Serialize};
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Pi_1_Proof {}

impl Pi_1_Proof {
    pub fn siz(&self) -> usize {
        0
    }
    fn protocol_name() -> &'static [u8] {
        b"pi_1_proof"
    }

    pub fn mod_prove(
        transcript: &mut Transcript,
        gens_n: &MultiCommitGens,
        z_vec: &[Scalar],
        phi: &Scalar,
        l_form_vec: &[Scalar],
    ) -> (
        Pi_1_Proof,
        CompressedGroup,
        Scalar,
        Vec<Scalar>,
        Vec<Scalar>,
        Vec<GroupElement>,
    ) {
        transcript.append_protocol_name(Pi_1_Proof::protocol_name());

        let mut G_hat = gens_n.G.clone();
        G_hat.push(gens_n.h);

        let mut z_hat = z_vec.clone().to_vec();
        z_hat.push(*phi);

        let mut L_hat = l_form_vec.clone().to_vec();
        L_hat.push(Scalar::zero());

        let y_hat = scalar_math::compute_linearform(&L_hat, &z_hat);

        assert_eq!(z_hat.len(), G_hat.len());

        let P_hat = GroupElement::vartime_multiscalar_mul(&z_hat, &G_hat).compress();

        P_hat.append_to_transcript(b"P_hat", transcript);
        y_hat.append_to_transcript(b"y_hat", transcript);

        let c_1 = transcript.challenge_scalar(b"c_1");

        let mut L_tilde: Vec<Scalar> = Vec::new();
        for i in 0..L_hat.len() {
            L_tilde.push(c_1 * L_hat[i]);
        }

        (Pi_1_Proof {}, P_hat, y_hat, L_tilde, z_hat, G_hat)
    }

    pub fn mod_verify(
        &self,
        transcript: &mut Transcript,
        P_hat: &CompressedGroup,
        y_hat: &Scalar,
    ) -> Scalar {
        transcript.append_protocol_name(Pi_1_Proof::protocol_name());
        P_hat.append_to_transcript(b"P_hat", transcript);
        y_hat.append_to_transcript(b"y_hat", transcript);

        let c_1 = transcript.challenge_scalar(b"c_1");

        c_1
    }
}
