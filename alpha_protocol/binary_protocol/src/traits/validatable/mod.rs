pub trait Validatable<E>
where
    E: std::error::Error,
{
    fn validate(&self) -> Result<(), E>;
}
