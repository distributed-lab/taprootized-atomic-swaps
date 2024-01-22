use ark_bn254::{Bn254, Fr, G1Affine, G2Affine};
use ark_crypto_primitives::snark::SNARK;
use ark_ec::pairing::Pairing;
use ark_ff::QuadExtField;
use ark_groth16::{Groth16, Proof, VerifyingKey};
use ark_relations::r1cs::SynthesisError;
use num::bigint::ParseBigIntError;
use num::BigUint;
use std::str::FromStr;

pub type PublicInputs = Vec<<Bn254 as Pairing>::ScalarField>;
pub type ZkProof = Proof<Bn254>;
pub type VerificationKey = VerifyingKey<Bn254>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to deserialize from json: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("failed to parse point from string: {0}")]
    ParsePoints(#[from] ParseBigIntError),
    #[error("invalid verification key or proof")]
    VerifyProof(#[from] SynthesisError),
}

pub fn groth16_verifier(
    verification_key_json: &[u8],
    proof_json: &[u8],
    public_inputs_json: &[u8],
) -> Result<bool, Error> {
    let proof = ZkProof::from_json(proof_json).unwrap();
    let verification_key: VerificationKey = VerificationKey::from_json(verification_key_json)?;
    let public_inputs = PublicInputs::from_json(public_inputs_json)?;

    let pvk = Groth16::<Bn254>::process_vk(&verification_key)?;

    let result = Groth16::<Bn254>::verify_with_processed_vk(&pvk, &public_inputs as &[Fr], &proof)?;

    Ok(result)
}

pub trait FromJson {
    type Output;

    fn from_json(src: &[u8]) -> Result<Self::Output, Error>;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ZkProofIntermidiateRepr {
    pub pi_a: [String; 3],
    pub pi_b: [[String; 2]; 3],
    pub pi_c: [String; 3],
}

impl FromJson for ZkProof {
    type Output = ZkProof;

    fn from_json(src: &[u8]) -> Result<Self::Output, Error> {
        let interm_repr: ZkProofIntermidiateRepr = serde_json::from_slice(src)?;

        let pi_a = G1Affine::new(
            <Bn254 as Pairing>::BaseField::from(BigUint::from_str(interm_repr.pi_a[0].as_str())?),
            <Bn254 as Pairing>::BaseField::from(BigUint::from_str(interm_repr.pi_a[1].as_str())?),
        );

        let pi_b = G2Affine::new(
            QuadExtField {
                c0: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.pi_b[0][0].as_str(),
                )?),
                c1: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.pi_b[0][1].as_str(),
                )?),
            },
            QuadExtField {
                c0: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.pi_b[1][0].as_str(),
                )?),
                c1: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.pi_b[1][1].as_str(),
                )?),
            },
        );

        let pi_c = G1Affine::new(
            <Bn254 as Pairing>::BaseField::from(BigUint::from_str(interm_repr.pi_c[0].as_str())?),
            <Bn254 as Pairing>::BaseField::from(BigUint::from_str(interm_repr.pi_c[1].as_str())?),
        );

        Ok(Self {
            a: pi_a,
            b: pi_b,
            c: pi_c,
        })
    }
}

impl FromJson for PublicInputs {
    type Output = PublicInputs;

    fn from_json(src: &[u8]) -> Result<Self::Output, Error> {
        let interm_repr: Vec<String> = serde_json::from_slice(src)?;

        let mut result = Vec::new();
        for value in interm_repr {
            let scalar_bigint = BigUint::from_str(value.as_str())?;

            result.push(<Bn254 as Pairing>::ScalarField::from(scalar_bigint))
        }

        Ok(result)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerificationKeyIntermidiateRepr {
    pub vk_alpha_1: [String; 3],
    pub vk_beta_2: [[String; 2]; 3],
    pub vk_gamma_2: [[String; 2]; 3],
    pub vk_delta_2: [[String; 2]; 3],
    #[serde(rename = "IC")]
    pub ic: Vec<[String; 3]>,
}

impl FromJson for VerificationKey {
    type Output = VerificationKey;

    fn from_json(src: &[u8]) -> Result<Self::Output, Error> {
        let interm_repr: VerificationKeyIntermidiateRepr = serde_json::from_slice(src)?;

        let vk_alpha_1 = G1Affine::new(
            <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                interm_repr.vk_alpha_1[0].as_str(),
            )?),
            <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                interm_repr.vk_alpha_1[1].as_str(),
            )?),
        );

        let vk_beta_2 = G2Affine::new(
            QuadExtField {
                c0: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_beta_2[0][0].as_str(),
                )?),
                c1: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_beta_2[0][1].as_str(),
                )?),
            },
            QuadExtField {
                c0: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_beta_2[1][0].as_str(),
                )?),
                c1: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_beta_2[1][1].as_str(),
                )?),
            },
        );

        let vk_gamma_2 = G2Affine::new(
            QuadExtField {
                c0: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_gamma_2[0][0].as_str(),
                )?),
                c1: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_gamma_2[0][1].as_str(),
                )?),
            },
            QuadExtField {
                c0: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_gamma_2[1][0].as_str(),
                )?),
                c1: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_gamma_2[1][1].as_str(),
                )?),
            },
        );

        let vk_delta_2 = G2Affine::new(
            QuadExtField {
                c0: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_delta_2[0][0].as_str(),
                )?),
                c1: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_delta_2[0][1].as_str(),
                )?),
            },
            QuadExtField {
                c0: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_delta_2[1][0].as_str(),
                )?),
                c1: <Bn254 as Pairing>::BaseField::from(BigUint::from_str(
                    interm_repr.vk_delta_2[1][1].as_str(),
                )?),
            },
        );

        let mut gamma_abc_g1 = Vec::new();
        for val in interm_repr.ic.iter() {
            let ic = G1Affine::new(
                <Bn254 as Pairing>::BaseField::from(BigUint::from_str(val[0].as_str())?),
                <Bn254 as Pairing>::BaseField::from(BigUint::from_str(val[1].as_str())?),
            );

            gamma_abc_g1.push(ic);
        }

        Ok(VerificationKey {
            alpha_g1: vk_alpha_1,
            beta_g2: vk_beta_2,
            gamma_g2: vk_gamma_2,
            delta_g2: vk_delta_2,
            gamma_abc_g1,
        })
    }
}
