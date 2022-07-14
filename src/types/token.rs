use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Serialize, Deserialize)]
pub struct Token {
    token_type: String,
    project_id: String,
    private_key_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_x509_cert_url: String,
}

impl Token {
    pub fn from_json(path: &str) -> Result<Token> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        Token::from_str(data.as_str())
    }

    pub fn from_str(data: &str) -> Result<Token> {
        let token: Token = serde_json::from_str(data)?;
        Ok(token)
    }
}
