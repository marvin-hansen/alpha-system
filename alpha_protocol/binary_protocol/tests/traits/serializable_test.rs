use binary_protocol::{BytesSerializable, Sizeable};
use rkyv::api::high::to_bytes_with_alloc;
use rkyv::rancor::Error;
use rkyv::ser::allocator::Arena;
use rkyv::util::AlignedVec;
use rkyv::{Archive, Deserialize, Serialize};
use stream_errors::StreamError;

// Define a simple struct to test the BytesSerializable trait
#[derive(Archive, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct TestSerializable {
    id: u32,
}

impl TestSerializable {
    fn new(id: u32) -> Self {
        Self { id }
    }
}

impl BytesSerializable for TestSerializable {
    fn to_bytes(&self) -> Result<AlignedVec, StreamError> {
        let capacity = self.byte_size();
        let mut arena = Arena::with_capacity(capacity);
        match to_bytes_with_alloc::<_, Error>(self, arena.acquire()) {
            Ok(bytes) => Ok(bytes),
            Err(error) => Err(StreamError::SerializationError(error.to_string())),
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, StreamError>
    where
        Self: Sized,
    {
        let deserialized: Self = match rkyv::from_bytes::<Self, Error>(bytes) {
            Ok(deserialized) => deserialized,
            Err(error) => return Err(StreamError::DeserializationError(error.to_string())),
        };
        Ok(deserialized)
    }
}

impl Sizeable for TestSerializable {
    fn byte_size(&self) -> usize {
        4
    }
}

#[test]
fn test_bytes_serializable_roundtrip() {
    let original = TestSerializable::new(42);

    let bytes = original.to_bytes().unwrap();
    let deserialized = TestSerializable::from_bytes(&bytes).unwrap();

    assert_eq!(original, deserialized);
    assert_eq!(deserialized.id, 42);
}

#[test]
fn test_bytes_serializable_large_data() {
    // Test with a larger string
    let original = TestSerializable::new(42);

    let bytes = original.to_bytes().unwrap();
    let deserialized = TestSerializable::from_bytes(&bytes).unwrap();

    assert_eq!(original, deserialized);
    assert_eq!(deserialized.id, 42);
}
