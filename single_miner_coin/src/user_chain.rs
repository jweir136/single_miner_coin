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

    fn calculate_proof_of_work(hash: Hash) -> Hash {
        let mut proof: u32 = 0;

        loop {
            let proof_hash: Hash = sha256_hash(format!("{}", proof).as_bytes());
            let combined_hash = format!("{:?}", sha256_hash(format!("{:?}{:?}", hash, proof_hash).as_bytes())).to_string();

            if &combined_hash[..6] == "000000" {
                return proof_hash;
            }
        }
    }
    
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