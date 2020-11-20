use mockall;

pub mod json;

pub enum AlicaMessageValidationError {
    InvalidFormat(String),
    MissingField(String)
}

impl Into<String> for AlicaMessageValidationError {
    fn into(self) -> String {
        match self {
            AlicaMessageValidationError::InvalidFormat(message) => message,
            AlicaMessageValidationError::MissingField(field) => format!("Required field missing: {}", field)
        }
    }
}

pub type AlicaMessageValidationResult = Result<(), AlicaMessageValidationError>;

#[mockall::automock]
pub trait AlicaMessageJsonValidator {
    fn validate(&self, message: &[u8]) -> AlicaMessageValidationResult;
}