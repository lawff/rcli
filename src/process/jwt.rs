use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::get_content;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: String,
    exp: usize,
}

pub fn process_jwt_sign(
    sub: String,
    exp: usize,
    aud: String,
    alg: Algorithm,
) -> anyhow::Result<String> {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: sub.to_string(),
        aud: aud.to_string(),
        exp: now + exp,
    };
    let key = get_content("fixtures/jwt_secret.key")?;
    let key = key.as_slice();

    let header = Header {
        alg,
        ..Default::default()
    };

    let token = encode(&header, &claims, &EncodingKey::from_secret(key))?;
    Ok(token)
}

pub fn process_jwt_verify(token: String, aud: String, alg: Algorithm) -> anyhow::Result<bool> {
    let key = get_content("fixtures/jwt_secret.key")?;
    let key = key.as_slice();

    let mut validation = Validation::new(alg);
    validation.set_audience(&[aud]);
    validation.set_required_spec_claims(&["aud"]);

    let decoded = decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation)?;

    println!("{:?}", decoded.claims);
    println!("{:?}", decoded.header);

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_jwt_sign_verify() -> anyhow::Result<()> {
        let sub = "test".to_string();
        let exp = 3600;
        let aud = "rcli".to_string();
        let alg = Algorithm::HS256;
        let token = process_jwt_sign(sub, exp, aud.clone(), alg)?;
        process_jwt_verify(token, aud, alg)?;
        Ok(())
    }
}
