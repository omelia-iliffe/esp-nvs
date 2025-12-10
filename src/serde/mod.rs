use alloc::string::{String, ToString};
use core::fmt::Display;

mod ser;
mod de;

impl<P: crate::Platform> crate::Nvs<P> {
    /// Use serde to write a struct to nvs
    ///
    /// Supported types:
    /// NewTypes, Option<_>,
    /// char, String,
    /// Enums (unit varients only),
    /// bool, unsigned and signed intergers (up to 64 bit)
    pub fn serialize<T: serde::Serialize>(
        &mut self,
        namespace: &crate::Key,
        input: T,
    ) -> Result<(), SerdeError> {
        ser::write(self, namespace, input)
    }

    /// Use serde to read a struct from nvs
    ///
    /// Supported types:
    /// NewTypes, Option<_>,
    /// char, String,
    /// Enums (unit varients only),
    /// bool, unsigned and signed intergers (up to 64 bit)
    pub fn deserialize<'de, T: serde::Deserialize<'de>>(
        &'de mut self,
        namespace: &'de crate::Key,
    ) -> Result<T, SerdeError> {
        de::read(self, namespace)
    }
}

#[derive(Debug, strum::Display)]
pub enum SerdeError {
    NvsError(crate::error::Error),
    Custom(String),
    WontImplement(String),
    NotYetImplemented(String),
}

impl From<crate::error::Error> for SerdeError {
    fn from(value: crate::error::Error) -> Self {
        Self::NvsError(value)
    }
}
impl core::error::Error for SerdeError {}
impl serde::ser::Error for SerdeError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl serde::de::Error for SerdeError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl SerdeError {
    fn top_level_ser() -> Self {
        let msg = "top-level serializer supports only structs";
        SerdeError::Custom(msg.into())
    }

    fn no_key() -> Self {
        let msg = "tried to serialize a value before serializing key";
        SerdeError::Custom(msg.into())
    }

    fn wont_impl(msg: impl core::fmt::Display) -> Self {
        Self::WontImplement(msg.to_string())
    }
    fn not_yet_impl(msg: impl Display) -> Self {
        Self::NotYetImplemented(msg.to_string())
    }
}
