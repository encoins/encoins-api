use ed25519_dalek::{Keypair, Signer};
use crate::base_types::{Currency, UserId};
use serde::Serialize;
use crate::instruction::Instruction;

/// A transfer is a transaction request by a user
#[derive(Clone, Serialize,Debug)]
pub struct Transfer
{
    pub sender : UserId,
    pub recipient : UserId,
    pub amount : Currency
}


impl Transfer
{

    /// Signs a transfer and transforms it into an instruction to be sent to the network
    pub fn sign(self, secret_key : &Keypair) -> Instruction {
        let transfer : &[u8] = &(bincode::serialize(&self).unwrap()[..]);
        let signature = secret_key.sign(transfer).to_bytes().to_vec();
        Instruction::SignedTransfer {
            transfer : self,
            signature
        }
    }
}