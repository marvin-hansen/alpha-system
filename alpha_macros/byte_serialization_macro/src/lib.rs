use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// This macro implements the `BytesSerialization` trait for a given struct.
///
/// # Example
///
/// ```text
/// use byte_serialization_macro::BytesSerialization;
/// use rkyv::{Archive, Deserialize, Serialize};
///
/// #[derive(BytesSerialization)]
/// #[derive(Archive, Deserialize, Serialize, Clone, Debug, PartialEq)]
/// [rkyv(compare(PartialEq), derive(Debug))]
/// struct MyStruct {
///     field1: u32,
/// }
///
///
/// let my_struct = MyStruct { field1: 42 };
/// let bytes = my_struct.to_bytes().unwrap();
/// let deserialized_struct = MyStruct::from_bytes(&bytes).unwrap();
/// assert_eq!(deserialized_struct.field1, 42);
/// ```
///
///
#[proc_macro_derive(BytesSerialization)]
pub fn derive_bytes_serialization(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        use rkyv::api::high::to_bytes_with_alloc;
        use rkyv::rancor::Error;
        use rkyv::ser::allocator::Arena;
        use rkyv::util::AlignedVec;
        use stream_errors::StreamError;

        impl BytesSerializable for #name {
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
                Self: Sized{
                let deserialized: Self = match rkyv::from_bytes::<Self, Error>(&bytes) {
                    Ok(deserialized) => deserialized,
                    Err(error) => return Err(StreamError::DeserializationError(error.to_string())),
                };
                Ok(deserialized)
            }
        }
    };

    TokenStream::from(expanded)
}
