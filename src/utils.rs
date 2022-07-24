use bytes::Bytes;
use sha2::{Digest, Sha256};

pub mod bytes_serde_format {
    use bytes::Bytes;
    use serde::Serializer;
    pub fn serialize<S>(data: &Bytes, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let res = std::str::from_utf8(&data).unwrap();
        serializer.serialize_str(&res)
    }
}

pub fn sha256_encrypt(data: String) -> Bytes {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hex_hash = hex::encode(hasher.finalize());
    Bytes::from(hex_hash)
}
