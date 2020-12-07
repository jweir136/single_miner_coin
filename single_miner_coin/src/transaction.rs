use block_cryptography_rust::hashing::sha256_hash;
use block_cryptography_rust::signing::verify_data;
use ring::signature::Signature;
use ring::digest::Digest;

pub type Username = String;
pub type PublicKey = [u8; 32];
pub type Hash = Digest;
pub type Seal = Signature;

pub struct Transaction {
    to: Username,
    from: Username,
    amount: f32,
    sender_seal: Signature,
    reciever_seal: Signature
}

impl Transaction {
    pub fn new(to: Username, from: Username, amount: f32, sender_seal: Seal, reciever_seal: Seal) -> Self {
        Transaction {
            to: to,
            from: from,
            amount: amount,
            sender_seal: sender_seal,
            reciever_seal: reciever_seal
        }
    }

    pub fn to(&self) -> &Username {
        &self.to
    }

    pub fn from(&self) -> &Username {
        &self.from
    }

    pub fn amount(&self) -> &f32 {
        &self.amount
    }

    pub fn sender_seal(&self) -> &Seal {
        &self.sender_seal
    }

    pub fn receiver_seal(&self) -> &Seal {
        &self.reciever_seal
    }

    pub fn hash(&self) -> Hash {
        sha256_hash(format!("{}{}{}", &self.to, &self.from, &self.amount).as_bytes())
    }

    pub fn check(&self, sender_key: &PublicKey, reciever_key: &PublicKey) -> bool {
        verify_data(sender_key, format!("{:?}", self.hash()).as_bytes(), self.sender_seal) &&  verify_data(reciever_key, format!("{:?}", self.hash()).as_bytes(), self.reciever_seal) 
    }
}