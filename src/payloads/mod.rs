pub mod pipe_separated;

use std::fmt::{Debug, Display, Formatter, Result};
use mockall;

pub type ParsingResult = std::result::Result<TransactionPayload, Error>;
pub type SerializationResult = std::result::Result<Vec<u8>, Error>;

#[mockall::automock]
pub trait Parser {
    fn parse(&self, bytes: &[u8]) -> ParsingResult;
}

#[mockall::automock]
pub trait Serializer {
    fn serialize(&self, payload: &TransactionPayload) -> SerializationResult;
}

#[derive(Debug)]
pub enum Error {
    InvalidPayload(String),
    InvalidTimestamp,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let message = match self {
            Error::InvalidPayload(message) => message,
            Error::InvalidTimestamp => "Payload contains invalid timestamp",
        };

        write!(formatter, "{}", message)
    }
}

#[derive(PartialEq, Debug)]
pub struct TransactionPayload {
    pub agent_id: String,
    pub message_type: String,
    pub message_bytes: Vec<u8>,
    pub timestamp: u64,
}

impl TransactionPayload {
    pub fn new(agent_id: &str, message_type: &str, message_bytes: &[u8], timestamp: u64) -> Self {
        TransactionPayload {
            agent_id: agent_id.to_string(),
            message_type: message_type.to_string(),
            message_bytes: message_bytes.to_vec(),
            timestamp,
        }
    }
}

impl Default for TransactionPayload {
    fn default() -> Self {
        TransactionPayload {
            agent_id: "".to_string(),
            message_type: "".to_string(),
            message_bytes: "".as_bytes().to_vec(),
            timestamp: 0
        }
    }
}
