use crate::parsers::ArgType;
use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum PrintSyscallError {
    #[fail(display = "Syntax error near {:?}", near)]
    FormatSyntaxError { near: String },

    #[fail(display = "Unknown syntax error")]
    UnknownFormatSyntaxError,

    #[fail(
        display = "Length can't be specified for argument with type '{:?}'",
        for_type
    )]
    UnacceptableLength { for_type: ArgType },

    #[fail(
        display = "Wrong arguments count: expected {}, got {}",
        expected,
        got
    )]
    WrongArgumentsCount { expected: usize, got: usize },
}
