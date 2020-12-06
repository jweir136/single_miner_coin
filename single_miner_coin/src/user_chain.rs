use ring::digest::Digest;
use block_cryptography_rust::hashing::sha256_hash;

type Username = String;
type Hash = Digest;
type PublicKey = &[u8];

struct UserBlock {
    username: Username,
    key: PublicKey,
    last: Hash,
    proof: Hash
}

impl UserBlock {
    pub fn hash(&self) -> Hash {
        sha256_hash(format!("{}{}", self.username, self.key))
    }
}