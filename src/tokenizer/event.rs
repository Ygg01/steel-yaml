use std::borrow::Cow;
use std::fmt::{write, Debug, Display, Formatter};
use std::str::from_utf8_unchecked;

use YamlEvent::Error;

use crate::tokenizer::event::YamlEvent::{
    Directive, DocEnd, DocStart, ScalarValue, SeqEnd, SeqStart, StreamEnd, StreamStart,
};
use crate::tokenizer::iter::ErrorType;

pub enum YamlEvent<'a> {
    StreamStart,
    StreamEnd,
    DocStart,
    DocEnd,
    SeqStart,
    SeqEnd,
    Directive(DirectiveType, Cow<'a, [u8]>),
    ScalarValue(Cow<'a, [u8]>),
    Error(ErrorType),
}

#[derive(Copy, Clone)]
pub enum DirectiveType {
    Yaml,
    Tag,
    Reserved,
}

impl Display for DirectiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectiveType::Yaml => write!(f, "YAML"),
            DirectiveType::Tag => write!(f, "TAG"),
            DirectiveType::Reserved => write!(f, "RESERVED"),
        }
    }
}

impl<'a> Debug for YamlEvent<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamStart => write!(f, "+STR"),
            StreamEnd => write!(f, "-STR"),
            DocStart => write!(f, "+DOC"),
            DocEnd => write!(f, "-DOC"),
            SeqStart => write!(f, "+SEQ"),
            SeqEnd => write!(f, "-SEQ"),
            Directive(typ, x) => {
                write!(f, "#{} {}", typ, unsafe { from_utf8_unchecked(x.as_ref()) })
            }
            ScalarValue(x) => write!(f, "+VAL {}", unsafe { from_utf8_unchecked(x.as_ref()) }),
            Error(x) => write!(f, "ERR"),
        }
    }
}
