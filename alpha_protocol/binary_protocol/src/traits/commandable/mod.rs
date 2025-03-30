use crate::{BytesSerializable, Validatable};
use std::fmt::Debug;
use stream_errors::ValidationError;

pub trait Commandable:
    BytesSerializable + Validatable<ValidationError> + Send + Sync + Debug
{
    fn command_code(&self) -> u16;
}
