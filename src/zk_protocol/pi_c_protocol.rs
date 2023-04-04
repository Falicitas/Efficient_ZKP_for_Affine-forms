use super::super::commitments::{DotProductProofGens, MultiCommitGens};
use super::super::random::RandomTape;
use super::super::transcript::ProofTranscript;
use crate::curve25519::errors::ProofVerifyError;
use crate::curve25519::group::{CompressedGroup, CompressedGroupExt, GROUP_BASEPOINT};
use crate::curve25519::scalar::Scalar;
use crate::nozk_protocol::pi_1_protocol::Pi_1_Proof;
use crate::nozk_protocol::pi_2_protocol::Pi_2_Proof;
use crate::transcript;
use crate::zk_protocol::pi_0_protocol::Pi_0_Proof;
use merlin::Transcript;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pi_c_Proof {
    proof_0: Pi_0_Proof,
    proof_1: Pi_1_Proof,
    proof_2: Pi_2_Proof,
}

impl Pi_c_Proof {
    fn protocol_name() -> &'static [u8] {
        b"zk compressed pi_c proof"
    }

    pub fn prove(
        gens: &DotProductProofGens,
        transcript: &mut Transcript,
        prover_random_tape: &mut RandomTape,
        x_vec: &[Scalar],
        gamma: &Scalar,
        //? x' = (x, gamma)
        l_vec: &[Scalar],
        //? L
        y: &Scalar,
        //? L(y)
    ) -> (Pi_c_Proof, CompressedGroup, CompressedGroup) {
        transcript.append_protocol_name(Pi_c_Proof::protocol_name());

        let n = x_vec.len();
        assert_eq!(l_vec.len(), n);
        assert_eq!(gens.gens_n.n, n);

        let (proof_0, P, z_vec, phi) = Pi_0_Proof::mod_prove(
            transcript,
            prover_random_tape,
            &gens.gens_n,
            gamma,
            &l_vec,
            &y,
            &x_vec,
        );
        //? Pi_0{A, t}, P, z, phi <----- gens_n(g, h), gamma, l_vec, y, x_vec

        let (proof_1, P_hat, _y_hat, L_tilde, z_hat, G_hat_vec) =
            Pi_1_Proof::mod_prove(transcript, &gens.gens_n, &z_vec, &phi, &l_vec);

        //? Pi_1{}, P_hat, y_hat, L_tilde, z_hat, G_hat <------- gens_n, z_vec, phi, l_form_vec

        let gens_hat = MultiCommitGens {
            n: n + 1,
            G: G_hat_vec,
            h: GROUP_BASEPOINT,
        };
        //TODO: GROUP_BASEPOINT safe, probably.

        let (proof_2, _Q) =
            Pi_2_Proof::mod_prove(&gens_hat, &gens.gens_1, transcript, &L_tilde, &z_hat);
        (
            Pi_c_Proof {
                proof_0,
                proof_1,
                proof_2,
            },
            P,
            P_hat,
        )
    }
}
