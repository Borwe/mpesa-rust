use std::collections::HashMap;
use std::error::Error;
use reqwest::blocking::Client;
use openssl::x509::X509;
use openssl::rsa::Padding;

use super::utils::extract_auth_token;
use super::environment::Environment;

/// Mpesa client that will facilitate communication with the Safaricom API
#[derive(Debug)]
pub struct Mpesa {
    client_key: String,
    client_secret: String,
    environment: Environment,
}

impl Mpesa {
    /// Constructs a new `Mpesa` instance. 
    pub fn new(client_key: String, client_secret: String, environment: Environment) -> Mpesa {
        Mpesa {
            client_key,
            client_secret,
            environment,
        }
    }

    /// Sends `GET` request to Safaricom oauth to acquire token for authentication
    pub fn auth(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/oauth/v1/generate?grant_type=client_credentials", self.environment.base_url());

        let resp: HashMap<String, String> = Client::new().get(&url)
            .basic_auth(&self.client_key, Some(&self.client_secret))
            .send()?
            .json()?;
        
        Ok(extract_auth_token(&resp)?)
    }

    /// Generates security credentials
    /// M-Pesa Core authenticates a transaction by decrypting the security credentials.
    /// Security credentials are generated by encrypting the base64 encoded initiator password with M-Pesa’s public key, a X509 certificate.
    pub fn get_security_credentials(&self) -> Result<String, Box<dyn Error>> {
        let pem = self.environment.get_certificate().as_bytes();
        let cert = X509::from_pem(pem).expect("error extracting X509 from pem");
        // getting the public and rsa keys
        let pub_key = cert.public_key().expect("error getting public key");
        let rsa_key = pub_key.rsa().expect("error getting rsa key from pub_key");
        // configuring the buffer
        let buf_len = pub_key.size();
        let mut buffer = vec![0; buf_len];

        match rsa_key.public_encrypt(
            self.client_secret.as_bytes(),
            &mut buffer,
            Padding::PKCS1,
        ) {
            Ok(_) => Ok(base64::encode(buffer)),
            Err(_) => unimplemented!(),
        }
    }
}