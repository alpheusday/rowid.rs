/// Errors that may occur during the process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowIDError {
    SystemTimeBeforeEpoch,
    EncodedLength,
    InvalidEncoded,
    CharListLength,
}

impl RowIDError {
    /// Get the error message as `&str`.
    pub fn as_str(&self) -> &str {
        match self {
            | Self::SystemTimeBeforeEpoch => {
                "System time must not before the Unix epoch"
            },
            | Self::EncodedLength => "Encoded is not long enough to be decoded",
            | Self::InvalidEncoded => "Encoded is not valid",
            | Self::CharListLength => {
                "The length of char_list must be longer or equal to 28"
            },
        }
    }
}

impl std::fmt::Display for RowIDError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
