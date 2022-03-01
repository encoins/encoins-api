use std::fmt::{Display, Formatter};
use crate::base_types::UserId;
use crate::transfer::Transfer;
use serde::Serialize;


#[derive(Clone,Serialize,Debug)]
pub enum Instruction {
    SignedTransfer
    {
        transfer : Transfer,
        signature : Vec<u8> // vec of (signature .to_byte (easier to serialize))
    },

    Balance{user: UserId}
}



impl Display for Instruction
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self
        {
            Instruction::Balance {user} => { write!(f, " Balances of {}", user.to_string()) }
            Instruction::SignedTransfer {transfer, signature:_} => { write!(f, "New transfer : (sender : {}, recipient :{}, amount {})", transfer.sender , transfer.recipient, transfer.amount) }

        }
    }
}
