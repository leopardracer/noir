use crate::{
    AbiType,
    input_parser::{InputTypecheckingError, InputValue},
};
use acvm::{AcirField, FieldElement, acir::native_types::Witness};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputParserError {
    #[error("input file is badly formed, could not parse, {0}")]
    ParseInputMap(String),
    #[error(
        "The value passed for parameter `{arg_name}` is invalid:\nExpected witness values to be integers, but `{value}` failed with `{error}`"
    )]
    ParseStr { arg_name: String, value: String, error: String },
    #[error(
        "The value passed for parameter `{arg_name}` is invalid:\nValue {value} is less than minimum allowed value of {min}"
    )]
    InputUnderflowsMinimum { arg_name: String, value: String, min: String },
    #[error(
        "The value passed for parameter `{arg_name}` is invalid:\nValue {value} exceeds maximum allowed value of {max}"
    )]
    InputOverflowsMaximum { arg_name: String, value: String, max: String },
    #[error(
        "The value passed for parameter `{arg_name}` is invalid:\nValue {value} exceeds field modulus. Values must fall within [0, {})",
        FieldElement::modulus()
    )]
    InputExceedsFieldModulus { arg_name: String, value: String },
    #[error("cannot parse value `{0}` into {1:?}")]
    AbiTypeMismatch(String, AbiType),
    #[error("Expected argument `{0}`, but none was found")]
    MissingArgument(String),
}

impl From<toml::ser::Error> for InputParserError {
    fn from(err: toml::ser::Error) -> Self {
        Self::ParseInputMap(err.to_string())
    }
}

impl From<toml::de::Error> for InputParserError {
    fn from(err: toml::de::Error) -> Self {
        Self::ParseInputMap(err.to_string())
    }
}

impl From<serde_json::Error> for InputParserError {
    fn from(err: serde_json::Error) -> Self {
        Self::ParseInputMap(err.to_string())
    }
}

#[derive(Debug, Error)]
pub enum AbiError {
    #[error("Received parameters not expected by ABI: {0:?}")]
    UnexpectedParams(Vec<String>),
    #[error("The value passed for parameter `{}` does not match the specified type:\n{0}", .0.path())]
    TypeMismatch(#[from] InputTypecheckingError),
    #[error("ABI expects the parameter `{0}`, but this was not found")]
    MissingParam(String),
    #[error(
        "Could not read witness value at index {witness_index:?} (required for parameter \"{name}\")"
    )]
    MissingParamWitnessValue { name: String, witness_index: Witness },
    #[error(
        "Attempted to write to witness index {0:?} but it is already initialized to a different value"
    )]
    InconsistentWitnessAssignment(Witness),
    #[error(
        "The return value is expected to be a {return_type:?} but found incompatible value {value:?}"
    )]
    ReturnTypeMismatch { return_type: AbiType, value: InputValue },
    #[error("No return value is expected but received {0:?}")]
    UnexpectedReturnValue(InputValue),
}
