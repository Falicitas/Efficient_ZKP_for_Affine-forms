# Efficient ZKP for Affine-forms

Practical work for the implementation of *[Compressed sigma-Protocol Theory and Practical Application to Plug & Play Secure Algorithmics](https://eprint.iacr.org/2020/152.pdf)*, which is truncated in ZKPoK of linear/affine form.

Authors of the paper reconcile Bulletproofs with the theory of $\Sigma$-Protocols such that (a) applications can follow (established) cryptographic protocol theory, thereby dispensing with the need for "reinventing" it, while (b) enjoying exactly the same communication reduction. They do this by giving a precise perspective on BPs as a significant strengthening of the power of $\Sigma$-protocols, resulting in $\Pi_c$ protocol. By using $\Pi_c$ protocol as a black box, efficient zero-knowledge arguments for arbitrary arithmetic circuits materialize. 

The purpose of this library is to solve such problems in a uniform and standardized way: for arbitrary linear equations of the form Y = AX, with X private to P, the matrix A and vector Y is public (vector Y has lower dimension than vector X), where P proves that X satisfies the affine property without revealing any knowledge about X. 

Here, $\Pi_c$ protocol is sufficient to fit the purpose. The security assumption of the protocol is $\mathcal{D}_n-\text { Find - Rep }^{[1]}$. See section 4.3 for detail of the zero-knowledge property of the protocol.

Why write it:

- The existing open-source projects with related implementations suffer from the Weak Fiat-Shamir Transformation problem.
- No specific practice of this scenario could be found on the web.
- The paper only describes the core algorithm logic and does not give design recommendations for other engineering-time practices and extensions such as the initial setup of the Setup phase FS.
- Zero-knowledge proofs of various equations or relations are used to support the protocol Multiparty Confidential Computing (MPC), which is practically used in various fields such as big data analysis, privacy machine learning, etc. The protocol is one of the fundamental sub-protocols of MPC.

## Reference in engineering

Discrete Logarithmic assumption uses [curve25519-dalek]([curve25519-dalek/src at main · dalek-cryptography/curve25519-dalek (github.com)](https://github.com/dalek-cryptography/curve25519-dalek/tree/main/src)).

NIZK、Scalar implementation refers to [Spartan]([microsoft/Spartan: Spartan: High-speed zkSNARKs without trusted setup (github.com)](https://github.com/microsoft/Spartan)) repo. Use [ristretto255]([Spartan/ristretto255.rs at master · microsoft/Spartan (github.com)](https://github.com/microsoft/Spartan/blob/master/src/scalar/ristretto255.rs)) to avoid small subgroup attacks caused by the EC group of curve25519 not being with prime order.

[Compressed_sigma-protocol](https://github.com/3for/Compressed_sigma-protocol) for reference of the modular design of the protocol.

## Usage

build environment: Windows/Linux, rustc 1.63.0+, cargo 1.63.0+

There are three binary crates under [Efficient_ZKP_for_Affine-forms/src/bin]([Efficient_ZKP_for_Affine-forms/src/bin at master · Falicitas/Efficient_ZKP_for_Affine-forms (github.com)](https://github.com/Falicitas/Efficient_ZKP_for_Affine-forms/tree/master/src/bin)), which can be considered as Prover-end interface, Verifier-end interface, and randomly generated verification data interface respectively.

run the bash command:

```bash
cargo run --package Efficient_ZKP_for_Affine-forms --bin verify_full_test

cargo run --package Efficient_ZKP_for_Affine-forms --bin prove_full_test

cargo run --package Efficient_ZKP_for_Affine-forms --bin raw_-_-_-_generator
```

> The three bars of raw - are the three parameters of the data. The data range can be adjusted according to the actual scenario.
>
> The data description is mentioned in the "Data Format" below.

### prove_full_test

prove_full_test (hereinafter referred to as the Prover) receives the common input $L,b_{vec}$ from the "raw.in" file, where $L$ corresponds to the $A$ matrix of the scenario $AX=Y$ and $b_{vec}$ corresponds to the $Y$ vector. the data of the raw file is defined in the "Data Format".

Prover gets the private vector $x_{vec}$ from the "private.in" file, which corresponds to the $X$ vector for the scenario $AX=Y$. private data is defined in the same place.

**The Setup stage** yields the elliptic curve group vector $\mathbf g$ of the same order as the private vector, the blind group constant $h$, and the group constant $k$ for the bulletproofs equation fusion.

**The Prove stage** generates "proof.in" file. "proof.in" contains the compacted proof. Proof data definition is the same. See [merlin](https://doc-internal.dalek.rs/merlin/index.html) for details on implementing transcript.

The theoretical cryptographic size of proof: $2\lceil \log_2(n+1)\rceil$ elements of $\mathbb G$ and 4 elements of $\mathbb Z_q$。

The actual size of the proof is shown in "Data Analysis".

Running prover will result in a console output of the project running each module speed test, like the following:

```text
Hi! input/parse running time: 97 s 141 ms 617 us
Hi! clone running time: 0 s 796 ms 579 us
Hi! gens generating running time: 1 s 372 ms 775 us
>> Hi! generate challenge rho running time: 0 s 0 ms 16 us
>> Hi! rho vec running time: 0 s 1 ms 318 us
>> Hi! transpose running time: 10 s 661 ms 626 us
>> Hi! M * rho_vec running time: 28 s 82 ms 893 us
>> Hi! compressed L_vec running time: 0 s 5 ms 56 us
Hi! proof running time: 47 s 270 ms 964 us
```

> You can see that IO/parse is the bottleneck. Improvements are mentioned at the end of "Data Format".

### verify_full_test

verify_full_test (hereinafter referred to as Verifier) receives public input $L,b_{vec}$ from the "raw.in" file.

Verifier gets the proof from the "proof.in" file.

**The Setup stage** yields the elliptic curve group vector $\mathbf g$ of the same order as the private vector, the blind group constant $h$, and the group constant $k$ for the bulletproofs equation fusion.

**The verify stage**: simulate the protocol step and determine the Open result as Rejected/Accepted based on the given proof.

## Data Format

Use [serde]([serde 1.0.160 - Docs.rs](https://docs.rs/crate/serde/1.0.160)) + json to serialize data.

### raw.in file

Data structure (Rust):

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Raw {
    m_matric: Vec<Vec<Scalar>>,
    b_vec: Vec<Scalar>,
}
```

> Scalar data structure:
>
> ```rust
> #[derive(Clone, Copy, Eq, Serialize, Deserialize)]
> pub struct Scalar(pub(crate) [u64; 4]);
> ```
>
> The selected curve is curve25519 (a high-speed elliptic curve). For details, see [curve25519_dalek]([curve25519_dalek - Rust](https://doc-internal.dalek.rs/curve25519_dalek/index.html)).
>
> More specifically, use [ristretto255]([Spartan/ristretto255.rs at master · microsoft/Spartan (github.com)](https://github.com/microsoft/Spartan/blob/master/src/scalar/ristretto255.rs)), a prime-order group abstraction atop `curve25519`.

Example (a $M_{10\times 5}(\mathbb Z_q)$ matrix with elements in the range $[0,1e9]$; a vector of $\mathbf b \in \mathbb Z_q^5$):

```json
{
    "m_matric":[
        [
            [13276908015437741373,3592806579707016811,18446744072625377718,1152921504606846975],
            [12614958237812813229,3030987718033332194,18446744073116338532,1152921504606846975],
            [9365260290691321405,5067376357125077850,18446744072909150961,1152921504606846975],
            [7664012236552416637,8410637746888738917,18446744073323616054,1152921504606846975],
            [15774217627742101565,958557695338336328,18446744073306388416,1152921504606846975],
            [5826072014865981885,2181825295617706142,18446744073025290929,1152921504606846975],
            [8141829510864244973,2177128762238151152,18446744072843458224,1152921504606846975],
            [6380862624016234141,17168035538327950704,18446744072700977883,1152921504606846975],
            [3309586274979345197,17142985049237517935,18446744072597444298,1152921504606846975],
            [2451010686357911165,5284116648411382727,18446744073702473400,1152921504606846975],
            [0,0,0,0],
            [0,0,0,0],
            [0,0,0,0],
            [0,0,0,0]
        ],
        [
            [6704455190936064685,10341137113960808256,18446744073602603304,1152921504606846975],
            [12779849998191621261,3114502591235457963,18446744073223558837,1152921504606846975],
            [12138448063643584909,18261121437134717547,18446744073631483224,1152921504606846975],
            [15233815262005558125,16071056226817278419,18446744072988468023,1152921504606846975],
            [6559416847632926829,11342628997071964814,18446744073563896547,1152921504606846975],
            [3560177459904650285,2622382674915976073,18446744072890510146,1152921504606846975],
            [11637864778482141789,10870533681176228915,18446744072940181142,1152921504606846975],
            [3395466689902326925,6090976866353146670,18446744073491769556,1152921504606846975],
            [1695598268450460349,15471921048224802153,18446744072721751554,1152921504606846975],
            [4681344253222265853,11147320607836298652,18446744073614183131,1152921504606846975],
            [0,0,0,0],
            [0,0,0,0],
            [0,0,0,0],
            [0,0,0,0]
        ],
        ...
    ],
    "b_vec":[
        [16165370562432757293,2472276347282455938,14658254701303188733,1152921504606846975],
        [3769179692919829437,4295222006105858819,16125493125238889864,1152921504606846975],
        [5092172213295258077,15136393472795811008,15315067540909944876,1152921504606846975],
        [3646469370550633117,10445048618549962130,15688014505047568228,1152921504606846975],
        [7392665778416128445,12943298557239208650,13883433267589129859,1152921504606846975]
    ]
}
```

> In dalek, there are many engineering optimizations for curve25519. The matrix values seem to be disordered in the $[0,2^{64})$ range, but they are actually transformations of Montgomery and other coordinate systems for optimization usage, whose optimization techniques are demonstrated in curve25519 specification.

### private.in file

Data structure (Rust):

```rust
#[derive(Debug, Serialize, Deserialize)]
struct X {
    x_vec: Vec<Scalar>,
}
```

Example (a vector of $\mathbf x \in \mathbb Z_q^{10}$ with elements in the range $[0,1e9]$).

```rust
{
    "x_vec":[
        [5251319160379093949,8799768076557345214,18446744072823710335,1152921504606846975],
        [17512364419966843869,16977316210074015753,18446744073595419838,1152921504606846975],
        [12385927357735259277,18306212413368797835,18446744072424889273,1152921504606846975],
        [2564821099300706749,90727270523353831,18446744073477281825,1152921504606846975],
        [7262948673690039725,17549384933758080969,18446744073123044801,1152921504606846975],
        [11825195126148417117,16986426204587851471,18446744072676061389,1152921504606846975],
        [4451603737918421245,14264338277772149537,18446744072846965351,1152921504606846975],
        [15945819925830226429,12136244039198684177,18446744073701768659,1152921504606846975],
        [13942212278420420813,14081750297034536888,18446744072725597752,1152921504606846975],
        [16213490331297649229,16618084983927167250,18446744073282253364,1152921504606846975],
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0]
    ]
}
```

### proof.in file

Generated by the Prover.

Data structure (Rust):

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Proof_and_Commitments {
    proof: Pi_Affine_Proof,
    P: CompressedGroup,
    P_hat: CompressedGroup,
    y: Scalar,
}
```

> The `Pi_Affine_Proof` data structure is shown in [Efficient_ZKP_for_Affine-forms/src/zk_protocol]([Efficient_ZKP_for_Affine-forms/src/zk_protocol at master · Falicitas/Efficient_ZKP_for_Affine-forms (github.com)](https://github.com/Falicitas/Efficient_ZKP_for_Affine-forms/tree/master/src/zk_protocol))

Example (the proof based on a $M_{10\times 5}(\mathbb Z_q)$ matrix with elements in the range $[0,1e9]$, a $\mathbf b \in \mathbb Z_q^5$ vector; a $\mathbf x \in \mathbb Z_q^{10}$ vector with elements in the range $[0, 1e9]$).

```rust
{
    "proof":{
        "proof":{
            "proof_0":{
                "A":[152,236,155,149,17,254,84,5,22,151,230,53,228,74,189,180,46,224,39,169,60,13,141,255,197,200,146,134,162,28,107,103],
                "t":[2897318199261404492,16115809634034243861,12550015130646925735,618649744646671615]
            },
            "proof_1":{},
            "proof_2":{
                "bullet_reduction_proof":{
                    "A_vec":[
                        [154,155,151,100,162,167,204,6,197,153,215,27,15,58,115,139,170,219,19,87,16,9,209,102,175,37,6,13,142,198,103,49],
                        [254,26,40,123,234,132,152,140,137,17,80,98,193,175,96,61,75,245,105,107,60,235,160,88,19,21,54,187,65,227,177,48],
                        [152,121,25,246,105,75,96,168,190,176,227,162,40,151,239,48,190,200,199,223,35,106,188,31,177,245,154,0,101,205,34,1]
                    ],
                    "B_vec":[
                        [174,189,159,116,180,59,192,131,158,17,34,174,112,91,216,132,188,63,219,115,28,71,100,100,139,217,179,9,34,100,70,58],
                        [166,3,155,242,3,102,222,18,4,199,5,100,58,220,78,167,70,246,221,3,15,43,136,178,134,65,177,68,191,165,133,22],
                        [126,152,118,202,147,90,195,31,237,167,184,131,87,112,20,31,164,8,61,104,9,134,72,253,114,175,130,239,21,225,107,83]
                    ],
                    "z":[
                        [1283086968465217592,4948381693011550388,14530263635831889355,302277948331831675],
                        [1679898278614087945,9241039853980972493,7762127129313567107,314404131039404944]
                    ]
                }
            }
        }
    },
    "P":[102,213,178,195,37,172,30,11,173,136,11,29,37,241,62,139,252,4,71,190,89,18,196,57,26,132,30,148,66,207,237,80],
    "P_hat":[10,255,192,121,55,166,61,24,159,100,180,95,156,210,108,100,76,91,156,118,58,68,211,105,81,103,245,230,195,81,19,10],
    "y":[0,0,0,0]
}
```

### Data range

In principle, since the data range has no reference meaning under the modular operation and, since ristretto255 is equivalent to encapsulating all the points to ensure that the points provided to the outside world are in prime order and there is no small subgroup attack.

The actual interface only provides integer support for $[0,2^{512})$:

```rust
Scalar::from_u512(limbs: [u64; 8]) -> Scalar
```

In addition, the length $n$ of the privacy vector $\mathbf x$ will be filled with a certain number of 0 to the length of $2^t-2$ ($t>1$), which is used to adapt the halved recursion of BP. The paper proposes that adding 0 elements to $\mathbf x$ and $L$ in the protocol does not affect the zero-knowledge property of the protocol, and it is easy to prove that filling 0 in the data outside the protocol is equivalent to the previous operation.

### Data optimization

In fact, json can be replaced by protobuf, which can increase the speed by 10~20 times in the IO/parse stage. Google Protocol Buffer (protobuf for short) is Google's internal mixed-language data standard. Protocol Buffers is a lightweight and efficient structured data storage format that can be used for structured data serialization, or serialization. It is very suitable for data storage or RPC data exchange format. A language-independent, platform-independent, and extensible serialized structured data format that can be used in instant messaging, data storage, and other fields. For detailed documentation, see [protocolbuffers/protobuf]([protocolbuffers/protobuf: Protocol Buffers - Google's data interchange format (github.com)](https://github.com/protocolbuffers/protobuf)).

But dealing with IO/parse is not the focus of this project.

## Reference

[1]: DAZA V, RÀFOLS C, ZACHARAKIS A. Updateable inner product argument with logarithmic verifier and applications[C]. In: Public-Key Cryptography—PKC 2020, Part I. Springer Cham, 2020: 527–557. [DOI: 10.1007/978- 3-030-45374-9_18]






