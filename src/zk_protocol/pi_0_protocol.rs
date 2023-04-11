use super::super::commitments::{Commitments, MultiCommitGens};
use super::super::random::RandomTape;
use super::super::transcript::{AppendToTranscript, ProofTranscript};
use super::sigma_phase;
use crate::curve25519::errors::ProofVerifyError;
use crate::curve25519::group::{CompressedGroup, CompressedGroupExt};
use crate::curve25519::scalar::Scalar;
use crate::curve25519::scalar_math;
use crate::transcript;
use merlin::Transcript;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pi_0_Proof {
    pub A: CompressedGroup,
    pub t: Scalar,
}

impl Pi_0_Proof {
    fn protocol_name() -> &'static [u8] {
        b"pi_0_proof"
    }

    pub fn mod_prove(
        transcript: &mut Transcript,
        prover_random_tape: &mut RandomTape,
        gens_n: &MultiCommitGens,
        gamma: &Scalar,
        l_form_vec: &[Scalar],
        y: &Scalar,
        x_vec: &[Scalar],
    ) -> (Pi_0_Proof, CompressedGroup, Vec<Scalar>, Scalar) {
        let P = x_vec.commit(&gamma, gens_n).compress();

        transcript.append_protocol_name(Pi_0_Proof::protocol_name());
        P.append_to_transcript(b"P", transcript);
        y.append_to_transcript(b"y", transcript);

        let (A, t, r_vec, rho) =
            sigma_phase::commit_phase(transcript, prover_random_tape, gens_n, l_form_vec);
        //TODO: order of transcript?
        let c_0 = sigma_phase::challenge_phase(transcript);

        let (z, phi) = sigma_phase::response_phase(&c_0, x_vec, &r_vec, gamma, &rho);

        (Pi_0_Proof { A, t }, P, z, phi)
    }

    pub fn mod_verify(
        &self,
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
        a: &[Scalar],
        P: &CompressedGroup,
        y: &Scalar,
    ) -> Scalar {
        assert_eq!(gens_n.n, a.len());

        transcript.append_protocol_name(Pi_0_Proof::protocol_name());
        P.append_to_transcript(b"P", transcript);
        y.append_to_transcript(b"y", transcript);
        self.A.append_to_transcript(b"A", transcript);
        self.t.append_to_transcript(b"t", transcript);

        let c = transcript.challenge_scalar(b"c");
        //?为什么要返回c
        c
    }
}
