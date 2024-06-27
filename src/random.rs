use bc_crypto::{ hash::{ sha256, hkdf_hmac_sha256 }, SHA256_SIZE };
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DeterministicRandomNumberGenerator {
    seed: [u8; SHA256_SIZE],
    salt: u64,
}

impl DeterministicRandomNumberGenerator {
    pub fn new(seed: [u8; SHA256_SIZE]) -> Self {
        Self { seed, salt: 0 }
    }

    pub fn new_with_seed(seed: &str) -> Self {
        let seed = sha256(seed.as_bytes());
        Self::new(seed)
    }

    pub fn deterministic_random_data(&mut self, size: usize) -> Vec<u8> {
        self.salt += 1;
        let mut salt_bytes = [0u8; 8];
        salt_bytes.copy_from_slice(&self.salt.to_le_bytes());
        hkdf_hmac_sha256(self.seed, salt_bytes, size)
    }
}

pub fn sha256_deterministic_random(entropy: &[u8], n: usize) -> Result<Vec<u8>> {
    let seed = sha256(entropy);
    if n <= seed.len() {
        Ok(seed[..n].to_vec())
    } else {
        Err(anyhow::anyhow!("Random number generator limits reached."))
    }
}

pub fn sha256_deterministic_random_string(string: &str, n: usize) -> Result<Vec<u8>> {
    let entropy = string.as_bytes();
    sha256_deterministic_random(entropy, n)
}

pub fn deterministic_random(entropy: &[u8], n: usize) -> Vec<u8> {
    let seed = sha256(entropy);
    hkdf_hmac_sha256(seed, [], n)
}
