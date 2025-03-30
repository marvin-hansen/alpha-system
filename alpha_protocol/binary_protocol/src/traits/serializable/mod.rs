use rkyv::util::AlignedVec;
use stream_errors::StreamError;

/// Traits for serializing and deserializing types to and from bytes.
///
/// The BytesSerialization macro automatically implements
/// the BytesSerializable trait below. See delta_macros/byte_serialization_macro
pub trait BytesSerializable {
    fn to_bytes(&self) -> Result<AlignedVec, StreamError>;

    fn from_bytes(bytes: &[u8]) -> Result<Self, StreamError>
    where
        Self: Sized;
}
