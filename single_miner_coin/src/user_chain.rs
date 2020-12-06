use ring::digest::Digest;
use block_cryptography_rust::hashing::sha256_hash;

type Username = String;
type Hash = Digest;
type PublicKey = [u8; 32];

struct UserBlock {
    username: Username,
    key: PublicKey,
    last: Hash,
    proof: Hash
}

impl UserBlock {
    
    pub fn hash(&self) -> Hash {
        sha256_hash(format!("{}{:?}", self.username, self.key).as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        let block = UserBlock {
            username: "test".to_string(),
            key: [0 as u8; 32],
            last: sha256_hash(format!("test").as_bytes()),
            proof: sha256_hash(format!("test").as_bytes())
        };

        block.hash();
    }
}