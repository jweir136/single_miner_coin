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

    pub fn new(username: String, key: PublicKey, last_hash: Hash) -> Self {
        UserBlock {
            proof: UserBlock::calculate_proof_of_work(sha256_hash(format!("{}{:?}{:?}", &username, &key, &last_hash).as_bytes())),
            username: username,
            key: key,
            last: last_hash,
        }
    }

    fn calculate_proof_of_work(hash: Hash) -> Hash {
        let mut proof: u32 = 0;

        loop {
            let proof_hash: Hash = sha256_hash(format!("{}", proof).as_bytes());
            let combined_hash = &format!("{:?}", sha256_hash(format!("{:?}{:?}", hash, proof_hash).as_bytes())).to_string()[7..];

            if &combined_hash[..12] == "000000000000" {
                return proof_hash;
            } else {
                proof += 1;
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

    #[test]
    fn hash_pow_test() {
        let username = "Jake".to_string();
        let key = [0 as u8; 32];
        let last = sha256_hash("START".as_bytes());

        let block = UserBlock {
            proof: UserBlock::calculate_proof_of_work(
                sha256_hash(format!("{}{:?}{:?}", &username, &key, &last).as_bytes())
            ),
            username: username,
            key: key,
            last: last
        };
    }   
}