use sha2::{Digest, Sha256};

pub mod bytes_serde_format {
    use bytes::Bytes;
    use serde::Serializer;
    pub fn serialize<S>(data: &Bytes, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let res = std::str::from_utf8(data).unwrap();
        serializer.serialize_str(res)
    }
}
pub mod vec_bytes_serde_format {
    use bytes::Bytes;
    use serde::Serializer;
    pub fn serialize<S>(data: &Vec<Bytes>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // let res = std::str::from_utf8(data).unwrap();
        // serializer.serialize_str(res)
        serializer.collect_seq(data.iter().map(|s| std::str::from_utf8(s).unwrap()))
    }
}

pub fn sha256_encrypt(data: String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
