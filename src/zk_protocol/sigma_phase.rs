use super::super::commitments::{Commitments, MultiCommitGens};
use crate::curve25519::group::CompressedGroup;
use crate::curve25519::scalar::Scalar;
use crate::curve25519::scalar_math;
use crate::random::RandomTape;
use crate::transcript::{AppendToTranscript, ProofTranscript};
use merlin::Transcript;

pub fn commit_phase(
    transcript: &mut Transcript,
    prover_random_tape: &mut RandomTape,
    gens_n: &MultiCommitGens,
    l_form_vec: &[Scalar],
) -> (CompressedGroup, Scalar, Vec<Scalar>, Scalar) {
    let n = l_form_vec.len();
    assert_eq!(gens_n.n, n);

    // produce r, rho as random factor
    let r_vec = prover_random_tape.random_vector(b"r_vec", n);
    let rho = prover_random_tape.random_scalar(b"rho");
    let A = r_vec.commit(&rho, gens_n).compress();
    //TODO: check out if random_tape and commit can be optimized.
    let t = scalar_math::compute_linearform(&l_form_vec, &r_vec);

    A.append_to_transcript(b"A", transcript);
    t.append_to_transcript(b"t", transcript);

    (A, t, r_vec, rho)
}

pub fn challenge_phase(transcript: &mut Transcript) -> Scalar {
    let c = transcript.challenge_scalar(b"c");
    //TODO: internal implication need for check.
    c
}

pub fn response_phase(
    c: &Scalar,
    x_vec: &[Scalar],
    r_vec: &[Scalar],
    gamma: &Scalar,
    rho: &Scalar,
) -> (Vec<Scalar>, Scalar) {
    // assert_eq!();
    let n = x_vec.len();
    let z = (0..n)
        .map(|i| c * x_vec[i] + r_vec[i])
        .collect::<Vec<Scalar>>();
    let phi = c * gamma + rho;

    (z, phi)
}
