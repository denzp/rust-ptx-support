use nom::types::CompleteStr;
use nom::*;

use crate::error::PrintSyscallError;

pub fn parse_format(format: &str) -> Result<Vec<Arg>, PrintSyscallError> {
    arg_list(CompleteStr(format))
        .map(|res| res.1)
        .map_err(|err| match err {
            Err::Error(Context::Code(CompleteStr(rest), _)) => {
                let is_head = format.len() == rest.len();
                let is_tail = rest.len() <= 5;

                PrintSyscallError::FormatSyntaxError {
                    near: format!(
                        "{}{}{}",
                        if is_head { "" } else { "..." },
                        rest.chars().take(5).collect::<String>(),
                        if is_tail { "" } else { "..." }
                    ),
                }
            }

            _ => PrintSyscallError::UnknownFormatSyntaxError,
        })
}

// Parsing based on:
// https://en.wikipedia.org/wiki/Printf_format_string

named!(arg_list <CompleteStr, Vec<Arg>>, do_parse!(
          many0!(none_of!("%")) >>
    args: many_till!(terminated!(arg, many0!(none_of!("%"))), eof!()) >>

    (args.0)
));

named!(arg <CompleteStr, Arg>, do_parse!(
               char!('%') >>
    flag:      opt!(arg_flag) >>
    width:     opt!(arg_width) >>
    precision: opt!(arg_precision) >>
    length:    opt!(arg_length) >>
    ty:        arg_type >>

    (Arg(flag, width, precision, length, ty))
));

named!(arg_flag <CompleteStr, ArgFlag>, value!(ArgFlag, one_of!("-+0#")));

named!(arg_width <CompleteStr, ArgWidth>, alt!(
    value!(ArgWidth, char!('*')) |
    value!(ArgWidth, take_while1!(|chr| chr >= '0' && chr <= '9'))
));

named!(arg_precision <CompleteStr, ArgPrecision>, preceded!(char!('.'), alt!(
    value!(ArgPrecision, char!('*')) |
    value!(ArgPrecision, take_while1!(|chr| chr >= '0' && chr <= '9'))
)));

named!(arg_length <CompleteStr, ArgLength>, alt!(
    value!(ArgLength::I8, tag!("hh")) |
    value!(ArgLength::I16, tag!("h")) |
    value!(ArgLength::I64, tag!("l"))
));

named!(arg_type <CompleteStr, ArgType>, alt!(
    value!(ArgType::Literal, char!('%')) |
    value!(ArgType::Signed, one_of!("di")) |
    value!(ArgType::Unsigned, one_of!("uxXo")) |
    value!(ArgType::Double, one_of!("fFeEgGaA")) |
    value!(ArgType::StringPointer, char!('s')) |
    value!(ArgType::Char, char!('c')) |
    value!(ArgType::Pointer, char!('p'))
));

#[derive(PartialEq, Debug)]
pub struct Arg(
    pub Option<ArgFlag>,
    pub Option<ArgWidth>,
    pub Option<ArgPrecision>,
    pub Option<ArgLength>,
    pub ArgType,
);

#[derive(PartialEq, Debug)]
pub struct ArgFlag;

#[derive(PartialEq, Debug)]
pub struct ArgWidth;

#[derive(PartialEq, Debug)]
pub struct ArgPrecision;

#[derive(PartialEq, Debug)]
pub enum ArgLength {
    I8,
    I16,
    I64,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ArgType {
    Literal,
    Signed,
    Unsigned,
    Double,
    StringPointer,
    Char,
    Pointer,
}

#[cfg(test)]
mod tests {
    use crate::error::PrintSyscallError;
    use nom::Context::Code;

    use super::*;
    use nom::Err::*;
    use nom::ErrorKind::*;

    #[test]
    fn test_arg_flag() {
        assert_eq!(arg_flag(CompleteStr("-")), Ok((CompleteStr(""), ArgFlag)));
        assert_eq!(arg_flag(CompleteStr("+")), Ok((CompleteStr(""), ArgFlag)));
        assert_eq!(arg_flag(CompleteStr("0")), Ok((CompleteStr(""), ArgFlag)));
        assert_eq!(arg_flag(CompleteStr("#")), Ok((CompleteStr(""), ArgFlag)));

        assert_eq!(
            arg_flag(CompleteStr("")),
            Err(Error(Code(CompleteStr(""), Eof)))
        );
        assert_eq!(
            arg_flag(CompleteStr("NEXT")),
            Err(Error(Code(CompleteStr("NEXT"), OneOf)))
        );
    }

    #[test]
    fn test_arg_width() {
        assert_eq!(arg_width(CompleteStr("*")), Ok((CompleteStr(""), ArgWidth)));
        assert_eq!(
            arg_width(CompleteStr("1f")),
            Ok((CompleteStr("f"), ArgWidth))
        );
        assert_eq!(
            arg_width(CompleteStr("12.")),
            Ok((CompleteStr("."), ArgWidth))
        );

        assert_eq!(
            arg_width(CompleteStr("")),
            Err(Error(Code(CompleteStr(""), Alt)))
        );
        assert_eq!(
            arg_width(CompleteStr("NEXT")),
            Err(Error(Code(CompleteStr("NEXT"), Alt)))
        );
    }

    #[test]
    fn test_arg_precision() {
        assert_eq!(
            arg_precision(CompleteStr(".*")),
            Ok((CompleteStr(""), ArgPrecision))
        );
        assert_eq!(
            arg_precision(CompleteStr(".12f")),
            Ok((CompleteStr("f"), ArgPrecision))
        );
        assert_eq!(
            arg_precision(CompleteStr(".2")),
            Ok((CompleteStr(""), ArgPrecision))
        );

        assert_eq!(
            arg_precision(CompleteStr("")),
            Err(Error(Code(CompleteStr(""), Eof)))
        );
        assert_eq!(
            arg_precision(CompleteStr(".")),
            Err(Error(Code(CompleteStr(""), Alt)))
        );
        assert_eq!(
            arg_precision(CompleteStr("*")),
            Err(Error(Code(CompleteStr("*"), Char)))
        );
        assert_eq!(
            arg_precision(CompleteStr("1")),
            Err(Error(Code(CompleteStr("1"), Char)))
        );
    }

    #[test]
    fn test_arg_length() {
        assert_eq!(
            arg_length(CompleteStr("hhd")),
            Ok((CompleteStr("d"), ArgLength::I8))
        );
        assert_eq!(
            arg_length(CompleteStr("hd")),
            Ok((CompleteStr("d"), ArgLength::I16))
        );
        assert_eq!(
            arg_length(CompleteStr("ld")),
            Ok((CompleteStr("d"), ArgLength::I64))
        );

        assert_eq!(
            arg_length(CompleteStr("")),
            Err(Error(Code(CompleteStr(""), Alt)))
        );
        assert_eq!(
            arg_length(CompleteStr("*")),
            Err(Error(Code(CompleteStr("*"), Alt)))
        );
        assert_eq!(
            arg_length(CompleteStr("1")),
            Err(Error(Code(CompleteStr("1"), Alt)))
        );
        assert_eq!(
            arg_length(CompleteStr("d")),
            Err(Error(Code(CompleteStr("d"), Alt)))
        );
    }

    #[test]
    fn test_arg_type() {
        assert_eq!(
            arg_type(CompleteStr("%")),
            Ok((CompleteStr(""), ArgType::Literal))
        );
        assert_eq!(
            arg_type(CompleteStr("d")),
            Ok((CompleteStr(""), ArgType::Signed))
        );
        assert_eq!(
            arg_type(CompleteStr("i")),
            Ok((CompleteStr(""), ArgType::Signed))
        );
        assert_eq!(
            arg_type(CompleteStr("u")),
            Ok((CompleteStr(""), ArgType::Unsigned))
        );
        assert_eq!(
            arg_type(CompleteStr("u")),
            Ok((CompleteStr(""), ArgType::Unsigned))
        );
        assert_eq!(
            arg_type(CompleteStr("f")),
            Ok((CompleteStr(""), ArgType::Double))
        );
        assert_eq!(
            arg_type(CompleteStr("F")),
            Ok((CompleteStr(""), ArgType::Double))
        );
        assert_eq!(
            arg_type(CompleteStr("e")),
            Ok((CompleteStr(""), ArgType::Double))
        );
        assert_eq!(
            arg_type(CompleteStr("E")),
            Ok((CompleteStr(""), ArgType::Double))
        );
        assert_eq!(
            arg_type(CompleteStr("g")),
            Ok((CompleteStr(""), ArgType::Double))
        );
        assert_eq!(
            arg_type(CompleteStr("G")),
            Ok((CompleteStr(""), ArgType::Double))
        );
        assert_eq!(
            arg_type(CompleteStr("x")),
            Ok((CompleteStr(""), ArgType::Unsigned))
        );
        assert_eq!(
            arg_type(CompleteStr("X")),
            Ok((CompleteStr(""), ArgType::Unsigned))
        );
        assert_eq!(
            arg_type(CompleteStr("o")),
            Ok((CompleteStr(""), ArgType::Unsigned))
        );
        assert_eq!(
            arg_type(CompleteStr("s")),
            Ok((CompleteStr(""), ArgType::StringPointer))
        );
        assert_eq!(
            arg_type(CompleteStr("c")),
            Ok((CompleteStr(""), ArgType::Char))
        );
        assert_eq!(
            arg_type(CompleteStr("p")),
            Ok((CompleteStr(""), ArgType::Pointer))
        );
        assert_eq!(
            arg_type(CompleteStr("a")),
            Ok((CompleteStr(""), ArgType::Double))
        );
        assert_eq!(
            arg_type(CompleteStr("A")),
            Ok((CompleteStr(""), ArgType::Double))
        );

        assert_eq!(
            arg_type(CompleteStr("")),
            Err(Error(Code(CompleteStr(""), Alt)))
        );
        assert_eq!(
            arg_type(CompleteStr("*")),
            Err(Error(Code(CompleteStr("*"), Alt)))
        );
        assert_eq!(
            arg_type(CompleteStr("1")),
            Err(Error(Code(CompleteStr("1"), Alt)))
        );
    }

    #[test]
    fn test_arg() {
        assert_eq!(
            arg(CompleteStr("%03.4ld")),
            Ok((
                CompleteStr(""),
                Arg(
                    Some(ArgFlag),
                    Some(ArgWidth),
                    Some(ArgPrecision),
                    Some(ArgLength::I64),
                    ArgType::Signed
                )
            ))
        );

        assert_eq!(
            arg(CompleteStr("%3.4lf")),
            Ok((
                CompleteStr(""),
                Arg(
                    None,
                    Some(ArgWidth),
                    Some(ArgPrecision),
                    Some(ArgLength::I64),
                    ArgType::Double
                )
            ))
        );

        assert_eq!(
            arg(CompleteStr("%3ld")),
            Ok((
                CompleteStr(""),
                Arg(
                    None,
                    Some(ArgWidth),
                    None,
                    Some(ArgLength::I64),
                    ArgType::Signed
                )
            ))
        );

        assert_eq!(
            arg(CompleteStr("%.4ld")),
            Ok((
                CompleteStr(""),
                Arg(
                    None,
                    None,
                    Some(ArgPrecision),
                    Some(ArgLength::I64),
                    ArgType::Signed
                )
            ))
        );

        assert_eq!(
            arg(CompleteStr("%ld")),
            Ok((
                CompleteStr(""),
                Arg(None, None, None, Some(ArgLength::I64), ArgType::Signed)
            ))
        );

        assert_eq!(
            arg(CompleteStr("%f")),
            Ok((
                CompleteStr(""),
                Arg(None, None, None, None, ArgType::Double)
            ))
        );

        assert_eq!(arg(CompleteStr("")), Err(Error(Code(CompleteStr(""), Eof))));
        assert_eq!(
            arg(CompleteStr("%")),
            Err(Error(Code(CompleteStr(""), Alt)))
        );
        assert_eq!(
            arg(CompleteStr("%0.4")),
            Err(Error(Code(CompleteStr(""), Alt)))
        );
        assert_eq!(
            arg(CompleteStr("%n")),
            Err(Error(Code(CompleteStr("n"), Alt)))
        );
        assert_eq!(
            arg(CompleteStr("%0.n")),
            Err(Error(Code(CompleteStr(".n"), Alt)))
        );
        assert_eq!(
            arg(CompleteStr("%0.4n")),
            Err(Error(Code(CompleteStr("n"), Alt)))
        );
        assert_eq!(
            arg(CompleteStr("%0.4ln")),
            Err(Error(Code(CompleteStr("n"), Alt)))
        );
    }

    #[test]
    fn test_args_list() {
        assert_eq!(arg_list(CompleteStr("")), Ok((CompleteStr(""), vec![])));
        assert_eq!(
            arg_list(CompleteStr("no args")),
            Ok((CompleteStr(""), vec![]))
        );

        assert_eq!(
            arg_list(CompleteStr("%d")),
            Ok((
                CompleteStr(""),
                vec![Arg(None, None, None, None, ArgType::Signed)]
            ))
        );

        assert_eq!(
            arg_list(CompleteStr("%d%x")),
            Ok((
                CompleteStr(""),
                vec![
                    Arg(None, None, None, None, ArgType::Signed),
                    Arg(None, None, None, None, ArgType::Unsigned)
                ]
            ))
        );

        assert_eq!(
            arg_list(CompleteStr("one %d arg")),
            Ok((
                CompleteStr(""),
                vec![Arg(None, None, None, None, ArgType::Signed)]
            ))
        );

        assert_eq!(
            arg_list(CompleteStr("several %d%f args")),
            Ok((
                CompleteStr(""),
                vec![
                    Arg(None, None, None, None, ArgType::Signed),
                    Arg(None, None, None, None, ArgType::Double)
                ]
            ))
        );

        assert_eq!(
            arg_list(CompleteStr("several %dseparated %f args")),
            Ok((
                CompleteStr(""),
                vec![
                    Arg(None, None, None, None, ArgType::Signed),
                    Arg(None, None, None, None, ArgType::Double)
                ]
            ))
        );

        assert_eq!(
            arg_list(CompleteStr("%")),
            Err(Error(Code(CompleteStr("%"), ManyTill)))
        );
        assert_eq!(
            arg_list(CompleteStr("% d")),
            Err(Error(Code(CompleteStr("% d"), ManyTill)))
        );
        assert_eq!(
            arg_list(CompleteStr("incomplete %0.4")),
            Err(Error(Code(CompleteStr("%0.4"), ManyTill)))
        );
        assert_eq!(
            arg_list(CompleteStr("incomplete %0.4 inside")),
            Err(Error(Code(CompleteStr("%0.4 inside"), ManyTill)))
        );
        assert_eq!(
            arg_list(CompleteStr("incorrect %n")),
            Err(Error(Code(CompleteStr("%n"), ManyTill)))
        );
        assert_eq!(
            arg_list(CompleteStr("incorrect %0.n")),
            Err(Error(Code(CompleteStr("%0.n"), ManyTill)))
        );
    }

    #[test]
    fn test_error_reporting() {
        assert_eq!(
            parse_format("%").unwrap_err(),
            PrintSyscallError::FormatSyntaxError { near: "%".into() }
        );

        assert_eq!(
            parse_format("% at the beginning").unwrap_err(),
            PrintSyscallError::FormatSyntaxError {
                near: "% at ...".into()
            }
        );

        assert_eq!(
            parse_format("incomplete %0.43").unwrap_err(),
            PrintSyscallError::FormatSyntaxError {
                near: "...%0.43".into()
            }
        );

        assert_eq!(
            parse_format("incomplete %0.434").unwrap_err(),
            PrintSyscallError::FormatSyntaxError {
                near: "...%0.43...".into()
            }
        );

        assert_eq!(
            parse_format("incomplete %0.4 deep inside").unwrap_err(),
            PrintSyscallError::FormatSyntaxError {
                near: "...%0.4 ...".into()
            }
        );
    }
}
