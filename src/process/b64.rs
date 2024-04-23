use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::io::Read;

use crate::Base64Format;

pub fn process_encode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encode)
}

pub fn process_decode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    Ok(String::from_utf8(decoded)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_reader;

    #[test]
    fn test_process_encode() -> Result<()> {
        let input = "Cargo.toml";
        let mut reader = get_reader(input)?;
        let format = Base64Format::Standard;
        assert!(process_encode(&mut reader, format).is_ok());
        Ok(())
    }

    #[test]
    fn test_process_decode() -> Result<()> {
        let input = "fixtures/b64.txt";
        let mut reader = get_reader(input)?;
        let format = Base64Format::UrlSafe;
        process_decode(&mut reader, format)?;
        Ok(())
    }
}
