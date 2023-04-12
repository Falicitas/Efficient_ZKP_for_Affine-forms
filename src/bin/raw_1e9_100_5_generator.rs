use rand::rngs::OsRng;
use rand::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use std::fs;
use Efficient_ZKP_for_Affine_forms::curve25519::scalar::Scalar;
use Efficient_ZKP_for_Affine_forms::curve25519::scalar_math;
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

fn write_to_file<T: Serialize>(path: String, object: T) {
    fs::write(path, serde_json::to_string(&object).unwrap()).unwrap();
}

pub fn generator(suffix_path: String) {
    let infix_path = "./random_data/1e9_100_5".to_owned();
    // Scalar range in [0,1e9), and n range in {10},s range in {5}

    let raw_file_path = infix_path.to_owned() + &"/raw_".to_owned() + &suffix_path;
    let private_file_path = infix_path.to_owned() + &"/private_".to_owned() + &suffix_path;
    //? rust . + & + & + &

    let mut csprng: OsRng = OsRng;

    let n = 100;
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

    write_to_file(
        raw_file_path,
        Raw {
            m_matric: m_matric.clone(),
            b_vec: b_vec.clone(),
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
        generator(i.to_string() + &".in".to_owned());
    }
}
