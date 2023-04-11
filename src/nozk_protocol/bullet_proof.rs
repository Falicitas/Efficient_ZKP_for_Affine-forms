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
}
