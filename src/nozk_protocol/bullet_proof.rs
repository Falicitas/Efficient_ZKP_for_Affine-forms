#![allow(non_snake_case)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
use super::super::math::Math;
use super::super::transcript::ProofTranscript;
use crate::curve25519::errors::ProofVerifyError;
use crate::curve25519::group::{CompressedGroup, GroupElement, VartimeMultiscalarMul};
use crate::curve25519::scalar::Scalar;
use crate::curve25519::scalar_math::inner_product;
use crate::transcript;
use core::iter;
use curve25519_dalek::ristretto::CompressedRistretto;
use merlin::Transcript;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BulletReductionProof {
    A_vec: Vec<CompressedGroup>,
    B_vec: Vec<CompressedGroup>,
    z: Vec<Scalar>,
}

impl BulletReductionProof {
    pub fn prove(
        transcript: &mut Transcript,
        k: &GroupElement,
        G_vec: &[GroupElement],
        z_vec: &[Scalar],
        L_vec: &[Scalar],
    ) -> BulletReductionProof {
        let mut G = &mut G_vec.to_owned()[..];
        let mut z = &mut z_vec.to_owned()[..];
        let mut L = &mut L_vec.to_owned()[..];
        //TODO: [..] ?

        let mut n = G.len();
        assert!(n.is_power_of_two());
        //TODO: is_power_of two
        let lg_n = n.log2() - 1;

        let G_factors: Vec<Scalar> = iter::repeat(Scalar::one()).take(n).collect();

        assert_eq!(G.len(), n);
        assert_eq!(z.len(), n);
        assert_eq!(L.len(), n);
        assert_eq!(G_factors.len(), n);

        let mut A_vec: Vec<CompressedRistretto> = Vec::with_capacity(lg_n);
        let mut B_vec: Vec<CompressedRistretto> = Vec::with_capacity(lg_n);

        while n != 2 {
            n /= 2;
            let (z_L, z_R) = z.split_at_mut(n);
            let (L_L, L_R) = L.split_at_mut(n);
            let (G_L, G_R) = G.split_at_mut(n);

            let c_L = inner_product(&z_L, &L_R);
            let c_R = inner_product(&z_R, &L_L);

            let A = GroupElement::vartime_multiscalar_mul(
                z_L.iter().chain(iter::once(&c_L)),
                G_R.iter().chain(iter::once(k)),
            );
            //TODO: 隔离群运算！

            let B = GroupElement::vartime_multiscalar_mul(
                z_R.iter().chain(iter::once(&c_R)),
                G_L.iter().chain(iter::once(k)),
            );

            transcript.append_point(b"L", &A.compress());
            transcript.append_point(b"R", &B.compress());

            let c = transcript.challenge_scalar(b"c");

            for i in 0..n {
                z_L[i] = z_L[i] + c * z_R[i];
                L_L[i] = L_L[i] * c + L_R[i];
                G_L[i] =
                    GroupElement::vartime_multiscalar_mul(&[c, Scalar::one()], &[G_L[i], G_R[i]]);
            }

            A_vec.push(A.compress());
            B_vec.push(B.compress());

            z = z_L;
            L = L_L;
            G = G_L;
        }

        assert_eq!(z.len(), 2);

        BulletReductionProof {
            A_vec,
            B_vec,
            z: z.to_vec(),
        }
    }

    fn verification_scalars(
        &self,
        n: usize,
        transcript: &mut Transcript,
    ) -> Result<(Vec<Scalar>, Vec<Scalar>, Vec<Scalar>), ProofVerifyError> {
        let lg_n = self.A_vec.len();
        if lg_n >= 32 {
            // 4 billion multiplications should be enough for anyone
            // and this check prevents overflow in 1<<lg_n below.
            return Err(ProofVerifyError::InternalError);
        }

        if n / 2 != (1 << lg_n) {
            return Err(ProofVerifyError::InternalError);
        }

        // 1. Recompute x_k,...,x_1 based on the proof transcript
        let mut challenges = Vec::with_capacity(lg_n);
        for (L, R) in self.A_vec.iter().zip(self.B_vec.iter()) {
            transcript.append_point(b"L", L);
            transcript.append_point(b"R", R);
            challenges.push(transcript.challenge_scalar(b"c"));
        }

        // 2. Compute the ML_n Distribution mentioned in
        // <Updatable Inner Product Argument with Logarithmic Verifier and Applications>
        // as (1,u_1,u_2,u_1u_2,u_3,u_1u_3,u_2u_3,\cdots,u_1u_2\cdots u_k)
        let mut ml_challenges: Vec<Scalar> = Vec::with_capacity(lg_n);
        ml_challenges.push(Scalar::one());
        for i in (0..challenges.len()).rev() {
            let len_ml = ml_challenges.len();
            for j in 0..len_ml {
                ml_challenges.push(challenges[i] * ml_challenges[j]);
            }
        }
        // 3. Reverse of ml_challenges
        let mut rev_ml_challenges: Vec<Scalar> = Vec::with_capacity(lg_n);
        for i in (0..ml_challenges.len()).rev() {
            rev_ml_challenges.push(ml_challenges[i]);
        }

        Ok((ml_challenges, rev_ml_challenges, challenges))
    }

    pub fn verify(
        &self,
        n: usize,
        b: &[Scalar],
        transcript: &mut Transcript,
        Gamma: &GroupElement,
        Q: &GroupElement,
        G: &[GroupElement],
    ) -> Result<(), ProofVerifyError> {
        assert_eq!(n, b.len());
        let (_ml_challenges, rev_ml_challenges, challenges) =
            self.verification_scalars(n, transcript)?;

        let Ls = self
            .A_vec
            .iter()
            .map(|p| p.decompress().ok_or(ProofVerifyError::InternalError))
            .collect::<Result<Vec<_>, _>>()?;

        let Rs = self
            .B_vec
            .iter()
            .map(|p| p.decompress().ok_or(ProofVerifyError::InternalError))
            .collect::<Result<Vec<_>, _>>()?;

        let mut b_odd: Vec<Scalar> = Vec::with_capacity(n / 2);
        let mut b_even: Vec<Scalar> = Vec::with_capacity(n / 2);
        for i in 0..b.len() {
            if i % 2 == 0 {
                b_odd.push(b[i]);
            } else {
                b_even.push(b[i]);
            }
        }
        let b_L_hat = inner_product(&b_odd, &rev_ml_challenges);
        let b_R_hat = inner_product(&b_even, &rev_ml_challenges);
        /*assert_eq!(self.b[0], b_L_hat);
        assert_eq!(self.b[1], b_R_hat);*/

        let mut G_odd: Vec<GroupElement> = Vec::with_capacity(n / 2);
        let mut G_even: Vec<GroupElement> = Vec::with_capacity(n / 2);
        for i in 0..G.len() {
            if i % 2 == 0 {
                G_odd.push(G[i]);
            } else {
                G_even.push(G[i]);
            }
        }
        let G_L_hat = GroupElement::vartime_multiscalar_mul(rev_ml_challenges.iter(), G_odd.iter());
        let G_R_hat =
            GroupElement::vartime_multiscalar_mul(rev_ml_challenges.iter(), G_even.iter());
        /*assert_eq!(self.G[0], G_L_hat);
        assert_eq!(self.G[1], G_R_hat);*/

        let lg_n = self.A_vec.len();
        let mut Gamma_hat = *Gamma;
        for i in 0..lg_n {
            Gamma_hat = GroupElement::vartime_multiscalar_mul(
                &[Scalar::one(), challenges[i], challenges[i].square()],
                &[Ls[i], Gamma_hat, Rs[i]],
            );
        }

        let c = inner_product(&self.z, &[b_L_hat, b_R_hat]);

        assert_eq!(
            Gamma_hat,
            &GroupElement::vartime_multiscalar_mul(&self.z, &[G_L_hat, G_R_hat]) + c * Q
        );
        if Gamma_hat == &GroupElement::vartime_multiscalar_mul(&self.z, &[G_L_hat, G_R_hat]) + c * Q
        {
            Ok(())
        } else {
            Err(ProofVerifyError::InternalError)
        }
    }
}
