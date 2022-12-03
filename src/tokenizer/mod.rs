use std::borrow::Cow;

pub use scanner::Scanner;

mod event;
mod iter;
mod reader;
mod scanner;

pub enum YamlToken<'a> {
    // strings, booleans, numbers, nulls, all treated the same
    Scalar(Cow<'a, [u8]>),

    // flow style like `[x, x, x]`
    // or block style like:
    //     - x
    //     - x
    Sequence(Vec<YamlToken<'a>>),

    // flow style like `{x: X, x: X}`
    // or block style like:
    //     x: X
    //     x: X
    Mapping(Vec<Entry<'a>>),

    // Error during parsing
    Error,
}

pub struct Entry<'a> {
    key: YamlToken<'a>,
    value: YamlToken<'a>,
}

#[inline(always)]
pub(crate) fn is_empty(token_bound: (usize, usize)) -> bool {
    token_bound.0 == token_bound.1
}
