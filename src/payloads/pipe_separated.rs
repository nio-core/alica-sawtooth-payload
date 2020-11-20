use crate::payloads::{Error, ParsingResult, TransactionPayload, SerializationResult};
use crate::payloads;

pub struct Format {}

impl Format {
    pub fn new() -> Self {
        Format {}
    }
}

impl Default for Format {
    fn default() -> Self {
        Format {}
    }
}

impl payloads::Format for Format {
    fn serialize(&self, payload: &TransactionPayload) -> SerializationResult {
        let message = String::from_utf8(payload.message_bytes.clone())
            .map_err(|_| Error::InvalidPayload("Message is not a UTF8 String".to_string()))?;
        let output = format!("{}|{}|{}|{}", payload.agent_id.clone(), payload.message_type.clone(), message, &payload.timestamp).as_bytes().to_vec();
        Ok(output)
    }

    fn deserialize(&self, bytes: &[u8]) -> ParsingResult {
        let payload = String::from_utf8(bytes.to_vec())
            .map_err(|_| Error::InvalidPayload("Payload is not a string".to_string()))?;

        let mut content = payload.split("|");
        let agent_id = content.next()
            .ok_or_else(|| Error::InvalidPayload("Payload contains no agent id".to_string()))?;
        let message_type = content.next()
            .ok_or_else(|| Error::InvalidPayload("Payload contains no message type".to_string()))?;
        let message_bytes = content.next()
            .and_then(|message| Some(message.as_bytes()))
            .ok_or_else(|| Error::InvalidPayload("Payload contains no message".to_string()))?;
        let timestamp = content.next()
            .ok_or_else(|| Error::InvalidPayload("Payload contains no timestamp".to_string()))?
            .parse::<u64>()
            .map_err(|_| Error::InvalidTimestamp)?;

        Ok(TransactionPayload::new(
            agent_id,
            message_type,
            message_bytes,
            timestamp,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::payloads::{TransactionPayload, pipe_separated, Format};

    mod parsing {
        use crate::payloads::{pipe_separated, Format};

        #[test]
        fn the_payload_is_valid_if_it_is_structured_properly() {
            let id = "id";
            let message_type = "type";
            let message_text = "msg";
            let timestamp = 684948894984u64;

            let payload_bytes = format!("{}|{}|{}|{}", id, message_type, message_text, timestamp)
                .as_bytes()
                .to_vec();

            let payload = pipe_separated::Format::default().deserialize(&payload_bytes).expect("Error parsing payload");

            assert_eq!(payload.agent_id, id);
            assert_eq!(payload.message_type, message_type);
            assert_eq!(payload.message_bytes, message_text.as_bytes().to_vec());
            assert_eq!(payload.timestamp, timestamp);
        }

        #[test]
        fn the_payload_is_not_valid_if_the_timestamp_is_missing() {
            let id = "id";
            let message_type = "type";
            let message_text = "msg";
            let payload_bytes = format!("{}|{}|{}", id, message_type, message_text)
                .as_bytes()
                .to_vec();

            let result = pipe_separated::Format::default().deserialize(&payload_bytes);

            assert!(result.is_err())
        }

        #[test]
        fn the_payload_is_not_valid_if_the_message_is_missing() {
            let id = "id";
            let message_type = "type";
            let timestamp = 6849849849u64;
            let payload_bytes = format!("{}|{}|{}", id, message_type, timestamp)
                .as_bytes()
                .to_vec();

            let result = pipe_separated::Format::default().deserialize(&payload_bytes);

            assert!(result.is_err());
        }

        #[test]
        fn the_payload_is_not_valid_if_the_message_type_is_missing() {
            let id = "id";
            let message = "message";
            let timestamp = 9819849484984u64;
            let payload_bytes = format!("{}|{}|{}", id, message, timestamp)
                .as_bytes()
                .to_vec();

            let result = pipe_separated::Format::default().deserialize(&payload_bytes);

            assert!(result.is_err());
        }

        #[test]
        fn the_payload_is_valid_if_the_agent_id_is_missing() {
            let message_type = "type";
            let message_text = "msg";
            let timestamp = 649494894984u64;
            let payload_bytes = format!("{}|{}|{}", message_type, message_text, timestamp)
                .as_bytes()
                .to_vec();

            let result = pipe_separated::Format::default().deserialize(&payload_bytes);

            assert!(result.is_err());
        }

        #[test]
        fn empty_message_is_not_parsed() {
            let payload_bytes = "".as_bytes();

            let result = pipe_separated::Format::default().deserialize(&payload_bytes);

            assert!(result.is_err());
        }
    }

    pub mod serialization {
        use crate::payloads::{pipe_separated, TransactionPayload, Format};

        #[test]
        fn it_serializes_valid_transaction_payloads() {
            let transaction_payload = TransactionPayload::default();

            let result = pipe_separated::Format::default().serialize(&transaction_payload);

            assert!(result.is_ok());
        }

        #[test]
        fn serialized_payloads_are_utf8_strings() {
            let transaction_payload = TransactionPayload::default();

            let result = pipe_separated::Format::default().serialize(&transaction_payload).unwrap();

            assert!(String::from_utf8(result).is_ok());
        }

        #[test]
        fn serialized_messages_have_four_parts() {
            let transaction_payload = TransactionPayload::default();

            let result = pipe_separated::Format::default().serialize(&transaction_payload).unwrap();
            let result_text = String::from_utf8(result).unwrap();
            let result_parts = result_text.split("|");

            assert_eq!(result_parts.count(), 4)
        }

        #[test]
        fn serialized_messages_contain_the_agent_id() {
            let agent_id = "id";
            let mut transaction_payload = TransactionPayload::default();
            transaction_payload.agent_id = agent_id.to_string();

            let result = pipe_separated::Format::default().serialize(&transaction_payload).unwrap();

            let result_text = String::from_utf8(result).unwrap();
            let mut result_parts = result_text.split("|");
            let id = result_parts.next().expect("Missing agent id");
            assert_eq!(id, agent_id)
        }

        #[test]
        fn serialized_messages_contain_the_message_type() {
            let message_type = "type";
            let mut transaction_payload = TransactionPayload::default();
            transaction_payload.message_type = message_type.to_string();

            let result = pipe_separated::Format::default().serialize(&transaction_payload).unwrap();

            let result_text = String::from_utf8(result).unwrap();
            let mut result_parts = result_text.split("|");
            result_parts.next();
            let msg_type = result_parts.next().expect("Missing message type");

            assert_eq!(msg_type, message_type)
        }

        #[test]
        fn serialized_messages_contain_the_message() {
            let message = "msg";
            let mut transaction_payload = TransactionPayload::default();
            transaction_payload.message_bytes = message.as_bytes().to_vec();

            let result = pipe_separated::Format::default().serialize(&transaction_payload).unwrap();

            let result_text = String::from_utf8(result).unwrap();
            let mut result_parts = result_text.split("|");
            result_parts.next();
            result_parts.next();
            let msg = result_parts.next().expect("Missing message");

            assert_eq!(msg, message)
        }

        #[test]
        fn serialized_messages_contain_the_timestamp() {
            let timestamp = 1;
            let mut transaction_payload = TransactionPayload::default();
            transaction_payload.timestamp = timestamp;

            let result = pipe_separated::Format::default().serialize(&transaction_payload).unwrap();

            let result_text = String::from_utf8(result).unwrap();
            let mut result_parts = result_text.split("|");
            result_parts.next();
            result_parts.next();
            result_parts.next();
            let ts = result_parts.next().expect("Missing message");

            assert_eq!(ts, timestamp.to_string())
        }
    }

    #[test]
    fn serialized_messages_can_be_read_by_parser() {
        let transaction_payload = TransactionPayload::default();

        let serialized_message = pipe_separated::Format::default().serialize(&transaction_payload)
            .expect("Could not serialize payload");
        let result = pipe_separated::Format::default().deserialize(&serialized_message)
            .expect("Could not parse payload");

        assert_eq!(result, transaction_payload)
    }
}
