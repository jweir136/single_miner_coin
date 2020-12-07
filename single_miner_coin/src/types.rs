use ring::digest::Digest;
use ring::signature::Signature;

pub type Hash = Digest;
pub type Seal = Signature;
pub type Username = String;
pub type PublicKey = [u8; 32];