//! Definition of global types for encoins

use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter};

pub type ComprPubKey = [u8; 32];

#[derive(Clone,Copy,PartialEq,Debug,Serialize,Deserialize,Eq,Hash)]
pub struct UserId
{
    pub id : ComprPubKey
}

/// For the moment, the currency is encoded in a 32-bit integer. Defining how to deal with currency is still to be determined
pub type Currency = u32;

/// For the moment, the sequence id of a transaction is a 32-bit integer. Maybe a specific type for big numbers should be implemented to avoid future problems
pub type SeqId = u32;

/// A transaction is an exchange of money between two accounts
#[derive(Clone, PartialEq,Debug,Serialize,Deserialize)]
pub struct Transaction
{
    /// seq_id is the id of the transaction. For a transaction t, seq_id will be the number of validated transfers outgoing form the sender +1.
    pub seq_id: SeqId,
    /// the user id of the transaction's sender
    pub sender_id: UserId,
    /// the user id of the transaction's receiver
    pub receiver_id: UserId,
    /// the currency exchanged
    pub amount: Currency
}


impl UserId
{
    /// Builds a UserId from a String
    pub fn from_string(string : &String) -> Result<UserId,String>
    {
        let mut result : [u8;32] = [0;32];
        for i in 0..32
        {
            let el1 = match string.as_bytes().get(2*i) {
                Some(e) => { let ba = b"a".get(0).unwrap();
                    if e < ba {return Err(String::from("Invalid character")) }
                    let el = e - ba;
                    if el > 15 { return Err(String::from("Invalid character")) }
                    el
                }
                None => {return Err(String::from("The key is too short"))}
            };
            let el2 = match string.as_bytes().get(2*i + 1) {
                Some(e) => { let ba = b"a".get(0).unwrap();
                    if e < ba {return Err(String::from("Invalid character")) }
                    let el = e - ba;
                    if el > 15 { return Err(String::from("Invalid character")) }
                    el
                }
                None => {return Err(String::from("The key is too short"))}
            };
            result[i] = el1 + (el2 << 4);
        }

        Ok(UserId{id : result})
    }

    pub fn from_bytes(bytes: [u8;32]) -> UserId
    {
        UserId{id : bytes}
    }

    /// Transforms a String into a UserId
    pub fn to_string(&self) -> String
    {
        let mut result = String::new();
        let pub_key = self.id;
        for el in pub_key
        {
            let el1 : u8 = (el << 4) >> 4;
            let el2 : u8 = el >> 4;
            result.push((el1 + b"a".get(0).unwrap()) as char);
            result.push((el2 + b"a".get(0).unwrap()) as char);
        }
        result
    }
}

impl Transaction
{
    pub fn from(seq_id : SeqId, sender_id : UserId, receiver_id : UserId, amount : Currency) -> Transaction
    {
        Transaction
        {
            seq_id,
            sender_id,
            receiver_id,
            amount
        }
    }

}

impl Display for UserId
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.to_string())
    }
}

impl Display for Transaction
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "(Sender : {}, Receiver : {}, Sender's seq id : {}, Amount : {})", self.sender_id, self.receiver_id, self.seq_id, self.amount)
    }
}

