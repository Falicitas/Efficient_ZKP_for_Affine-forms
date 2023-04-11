use rand::rngs::OsRng;
use rand::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use Efficient_ZKP_for_Affine_forms::curve25519::scalar::Scalar;
use Efficient_ZKP_for_Affine_forms::curve25519::scalar_math;
// use Efficient_ZKP_for_Affine_forms::zk_protocol::pi_c_protocol;
use merlin::Transcript;
use Efficient_ZKP_for_Affine_forms::transcript::{AppendToTranscript, ProofTranscript};
const MOD: u64 = 1_000_000_000;

fn random_in_1e9<Rng: RngCore + CryptoRng>(rng: &mut Rng) -> u64 {
    rng.next_u64() % MOD
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Raw {
    m_matric: Vec<Vec<Scalar>>,
    b_vec: Vec<Scalar>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Dispose {
    l_affine_vec: Vec<Scalar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct X {
    x_vec: Vec<Scalar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {}
//TODO: add Pi_c_proof

fn write_to_file<T: Serialize>(path: String, object: T) {
    fs::write(path, serde_json::to_string(&object).unwrap()).unwrap();
}

pub fn generator(suffix_path: String) {
    let infix_path = "1e9_10_".to_owned();

    let infix_suffix = infix_path + &suffix_path;
    let raw_file_path = "./random_data/raw_".to_owned() + &infix_suffix;
    let dispose_file_path = "./random_data/dispose_".to_owned() + &infix_suffix;
    let private_file_path = "./random_data/private_".to_owned() + &infix_suffix;
    //? rust . + & + & + &

    let mut fs_transcript = Transcript::new(b"Kinesis's protocol");

    let mut csprng: OsRng = OsRng;

    let n = 10;
    let s = 5;

    let mut m_matric: Vec<Vec<Scalar>> = Vec::new();
    let mut m_hat_matric: Vec<Vec<Scalar>> = Vec::new();
    let mut b_vec: Vec<Scalar> = Vec::new();
    let mut x_hat_vec: Vec<Scalar> = Vec::new();

    let mut lim = 1;

    while lim < n + 2 {
        lim = lim * 2;
    }

    for _ in 0..n {
        x_hat_vec.push(Scalar::from(random_in_1e9(&mut csprng)));
    }

    for _ in n..lim - 2 {
        x_hat_vec.push(Scalar::zero());
    }

    assert_eq!(x_hat_vec.len(), lim - 2);

    let x_vec = x_hat_vec.clone();
    x_hat_vec.push(-Scalar::one());

    for _ in 0..s {
        let mut tmp: Vec<Scalar> = (0..n)
            .map(|_| Scalar::from(random_in_1e9(&mut csprng)))
            .collect();

        for _ in n..lim - 2 {
            tmp.push(Scalar::zero());
        }

        assert_eq!(tmp.len(), lim - 2);

        m_matric.push(tmp.clone());

        //make $<L_i, \vec{x}>=0$
        let y = scalar_math::compute_linearform(&tmp, &x_vec);

        b_vec.push(y);
        tmp.push(y);

        m_hat_matric.push(tmp.clone());

        assert_eq!(
            scalar_math::compute_linearform(&tmp, &x_hat_vec),
            Scalar::zero()
        );
    }

    let rho = fs_transcript.challenge_scalar(b"rho");
    let rho_vec = scalar_math::vandemonde_challenge_one(rho, s);
    let l_hat_matrix_t = scalar_math::matrix_transpose(&m_hat_matric);
    let l_vec = scalar_math::matrix_vector_mul(&l_hat_matrix_t, &rho_vec);

    let y = scalar_math::compute_linearform(&l_vec, &x_hat_vec);
    assert_eq!(y, Scalar::zero());

    write_to_file(
        raw_file_path,
        Raw {
            m_matric: m_matric.clone(),
            b_vec: b_vec.clone(),
        },
    );

    write_to_file(
        dispose_file_path,
        Dispose {
            l_affine_vec: l_vec.clone(),
        },
    );

    write_to_file(
        private_file_path,
        X {
            x_vec: x_vec.clone(),
        },
    );
}

fn main() {
    for i in 0..10 {
        generator(i.to_string());
    }
}