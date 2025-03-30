use binary_protocol::{BytesSerializable, binary_utils};
use rkyv::util::AlignedVec;
use stream_errors::StreamError;

// A simple mock struct that implements BytesSerializable for testing
struct TestPayload {
    value: u32,
}

impl BytesSerializable for TestPayload {
    fn to_bytes(&self) -> Result<AlignedVec, StreamError> {
        let mut bytes = AlignedVec::with_capacity(4); // 4 bytes for u32
        bytes.extend_from_slice(&self.value.to_le_bytes());
        Ok(bytes)
    }

    fn from_bytes(_bytes: &[u8]) -> Result<Self, StreamError> {
        unimplemented!("Not needed for this test")
    }
}

#[test]
fn test_build_raw_bytes_empty_payload() {
    // Test with a payload that serializes to an empty byte array
    struct EmptyPayload;

    impl BytesSerializable for EmptyPayload {
        fn to_bytes(&self) -> Result<AlignedVec, StreamError> {
            Ok(AlignedVec::new())
        }

        fn from_bytes(_bytes: &[u8]) -> Result<Self, StreamError> {
            unimplemented!("Not needed for this test")
        }
    }

    let command_code: u16 = 0x1234u16;
    let payload = EmptyPayload;

    let result = binary_utils::pack_raw_bytes(command_code, &payload);

    // First 4 bytes: total length (8) in little-endian
    // Next 4 bytes: command code (0x1234) as u32 in little-endian
    assert_eq!(result.len(), 8);
    assert_eq!(&result[0..4], &[8, 0, 0, 0]); //  4 bytes
    assert_eq!(&result[4..8], &[52, 18, 0, 0]); // command code: 0x1234 as u16 (LE)
}

#[test]
fn test_build_raw_bytes_with_payload() {
    // Test with a payload containing a u32 value
    let command_code: u16 = 0x5678u16;
    let payload = TestPayload { value: 0x12345678 };

    let result = binary_utils::pack_raw_bytes(command_code, &payload);

    // Expected: [12, 0, 0, 0, 120, 86, 0, 0, 120, 86, 52, 18]
    // First 4 bytes: total length (12) in little-endian
    // Next 4 bytes: command code (0x5678) as u32 in little-endian
    // Last 4 bytes: payload value (0x12345678) in little-endian
    assert_eq!(result.len(), 12);
    assert_eq!(&result[0..4], &[12, 0, 0, 0]); //  4 bytes
    assert_eq!(&result[4..8], &[120, 86, 0, 0]); // command code: 0x1234 as u32 (LE)
    assert_eq!(&result[8..12], &[120, 86, 52, 18]); // payload value: 0x12345678 (LE)
}

#[test]
fn test_build_raw_bytes_multiple_values() {
    // Test with a complex payload containing multiple values
    struct ComplexPayload {
        values: Vec<u16>,
    }

    impl BytesSerializable for ComplexPayload {
        fn to_bytes(&self) -> Result<AlignedVec, StreamError> {
            let mut bytes = AlignedVec::new();
            // First add the count of values
            bytes.extend_from_slice(&(self.values.len() as u32).to_le_bytes());
            // Then add each value
            for val in &self.values {
                bytes.extend_from_slice(&val.to_le_bytes());
            }
            Ok(bytes)
        }

        fn from_bytes(_bytes: &[u8]) -> Result<Self, StreamError> {
            unimplemented!("Not needed for this test")
        }
    }

    let command_code: u16 = 0xABCDu16;
    let payload = ComplexPayload {
        values: vec![0x1122, 0x3344, 0x5566],
    };

    let result = binary_utils::pack_raw_bytes(command_code, &payload);

    // Expected total length: 4 (length) + 4 (command) + 4 (count) + 6 (values) = 18 bytes
    assert_eq!(result.len(), 18);
    assert_eq!(&result[0..4], &[18, 0, 0, 0]); // total length: 18 bytes
    assert_eq!(&result[4..8], &[205, 171, 0, 0]); // command code: 0xABCD as u32 (LE)
    assert_eq!(&result[8..12], &[3, 0, 0, 0]); // count: 3 as u32 (LE)
    assert_eq!(&result[12..14], &[34, 17]); // 0x1122 (LE)
    assert_eq!(&result[14..16], &[68, 51]); // 0x3344 (LE)
    assert_eq!(&result[16..18], &[102, 85]); // 0x5566 (LE)
}

#[test]
fn test_build_raw_bytes_large_payload() {
    // Test with a larger payload
    struct LargePayload {
        data: Vec<u8>,
    }

    impl BytesSerializable for LargePayload {
        fn to_bytes(&self) -> Result<AlignedVec, StreamError> {
            let mut bytes = AlignedVec::new();
            bytes.extend_from_slice(&self.data);
            Ok(bytes)
        }

        fn from_bytes(_bytes: &[u8]) -> Result<Self, StreamError> {
            unimplemented!("Not needed for this test")
        }
    }

    let command_code: u16 = 0x9999u16;
    let data = vec![0xAA; 1000]; // 1000 bytes all set to 0xAA
    let payload = LargePayload { data };

    let result = binary_utils::pack_raw_bytes(command_code, &payload);

    // Expected total length: 4 (length) + 4 (command) + 1000 (data) = 1008 bytes
    assert_eq!(result.len(), 1008);
    assert_eq!(&result[0..4], &[240, 3, 0, 0]); // Check length prefix
    assert_eq!(&result[4..8], &[153, 153, 0, 0]); // command code: 0x9999 as u32 (LE)

    // Check that all payload bytes are 0xAA
    for i in 8..1008 {
        assert_eq!(result[i], 0xAA);
    }
}
