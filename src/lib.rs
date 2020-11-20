use crate::payloads::TransactionPayload;

pub mod messages;
pub mod payloads;
pub mod helper;

pub struct TransactionFamily {
    name: String,
    versions: Vec<String>
}

impl TransactionFamily {
    pub fn new(name: &str, versions: &[String]) -> Self {
        TransactionFamily {
            name: name.to_string(),
            versions: versions.to_vec()
        }
    }

    pub fn calculate_namespace(&self) -> String {
        let namespace_part = helper::calculate_checksum(&self.name);
        namespace_part[..6].to_string()
    }

    pub fn calculate_state_address_for(&self, message: &TransactionPayload) -> String {
        let payload_part = helper::calculate_checksum(
            &format!("{}{}{}", &message.agent_id, &message.message_type, &message.timestamp));
        let namespace_part = self.calculate_namespace();
        format!("{}{}", &namespace_part[..6], &payload_part[..64])
    }

    pub fn latest_version(&self) -> String {
        self.versions.last()
            .expect(&format!("There are no versions for transaction family {} configured", &self.name))
            .clone()
    }
}

impl Default for TransactionFamily {
    fn default() -> Self {
        TransactionFamily {
            name: "".to_string(),
            versions: Vec::new()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::TransactionFamily;
    use crate::payloads::TransactionPayload;

    #[test]
    fn a_namespace_is_6_bytes_in_size() {
        let family = TransactionFamily::default();

        let namespace = family.calculate_namespace();

        assert_eq!(namespace.as_bytes().len(), 6)
    }

    #[test]
    fn a_state_address_is_70_bytes_in_size() {
        let payload = TransactionPayload::default();
        let family = TransactionFamily::default();

        let address = family.calculate_state_address_for(&payload);

        assert_eq!(address.as_bytes().len(), 70)
    }

    #[test]
    fn a_state_address_starts_with_the_namespace() {
        let payload = TransactionPayload::default();
        let family = TransactionFamily::default();
        let namespace = family.calculate_namespace();

        let address = family.calculate_state_address_for(&payload);

        assert!(address.starts_with(&namespace))
    }

    #[test]
    fn the_latest_version_is_the_one_with_the_highest_index() {
        let version1 = "0.1.0";
        let version2 = "0.2.0";
        let family = TransactionFamily::new("", &vec![version1.to_string(), version2.to_string()]);

        let version = family.latest_version();

        assert_eq!(version, version2)
    }
}
