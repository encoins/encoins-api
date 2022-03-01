use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use crate::base_types::{ComprPubKey, Currency, UserId};
use serde::{Serialize,Deserialize};
use crate::instruction::Instruction;

/// A transfer is a transaction request by a user
#[derive(Clone,Copy,Serialize,Deserialize,Debug)]
pub struct Transfer
{
    pub sender : UserId,
    pub recipient : UserId,
    pub amount : Currency
}


impl Transfer
{

    /// Signs a transfer and transforms it into an instruction to be sent to the network
    pub fn sign(self, secret_key : &Keypair) -> Instruction
    {
        let transfer : &[u8] = &(bincode::serialize(&self).unwrap()[..]);
        let signature = secret_key.sign(transfer).to_bytes().to_vec();
        Instruction::SignedTransfer {
            transfer : self,
            signature
        }
    }

    pub fn verif_signature_transfer(&self, pub_key : ComprPubKey, signature : Vec<u8>) -> bool
    {
        let public_key = PublicKey::from_bytes(&pub_key[..])
            .expect("Problem with the conversion from signature to bytes");
        let transfer = &(bincode::serialize(&self)
            .expect("Problem with the deserialization of a transfer message")[..]);
        public_key.verify(transfer, &Signature::from_bytes(signature.as_slice())
            .expect("Problem with the conversion from signature to bytes")).is_ok()
    }
}
