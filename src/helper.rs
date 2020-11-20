use sha2::Digest;

pub fn calculate_checksum<T>(data: &T) -> String
    where T: AsRef<[u8]> {
    let mut hasher = sha2::Sha512::new();
    hasher.update(data);
    data_encoding::HEXLOWER.encode(&hasher.finalize()[..])
}