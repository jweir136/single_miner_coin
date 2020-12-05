use ring::digest::Digest;
use block_cryptography_rust::hashing::sha256_hash;

type Username = String;
type Hash = Digest;
type PublicKey = &[u8];