use codee::{binary::BincodeSerdeCodec, HybridDecoder, HybridEncoder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use server_fn::{
    codec::{Patch, Post, Put},
    Bytes, ContentType, Decodes, Encodes, Format, FormatType,
};
use thiserror::Error;

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub enum BinaryError {
    #[error("Failed to encode: {0}")]
    Encoding(String),

    #[error("Failed to decode: {0}")]
    Decode(String),
}

pub struct BinaryEncoding;

impl ContentType for BinaryEncoding {
    const CONTENT_TYPE: &'static str = "application/x-bytes";
}

impl FormatType for BinaryEncoding {
    const FORMAT_TYPE: server_fn::Format = Format::Binary;
}

impl<T> Decodes<T> for BinaryEncoding
where
    T: DeserializeOwned,
{
    type Error = BinaryError;

    fn decode(bytes: server_fn::Bytes) -> Result<T, Self::Error> {
        BincodeSerdeCodec::decode_bin(&bytes).map_err(|e| BinaryError::Decode(e.to_string()))
    }
}

impl<T> Encodes<T> for BinaryEncoding
where
    T: Serialize,
{
    type Error = BinaryError;

    fn encode(output: &T) -> Result<server_fn::Bytes, Self::Error> {
        BincodeSerdeCodec::encode_bin(output)
            .map(Bytes::from)
            .map_err(|e| BinaryError::Encoding(e.to_string()))
    }
}

pub type Binary = Post<BinaryEncoding>;
pub type PutBinary = Put<BinaryEncoding>;
pub type PatchBinary = Patch<BinaryEncoding>;
