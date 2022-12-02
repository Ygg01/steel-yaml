use std::{ops::ControlFlow, slice::Windows};
use std::cmp::Ordering;
use std::ops::ControlFlow::{Break, Continue};

use memchr::memchr2;

pub struct StrReader<'a> {
    pub slice: &'a str,
    pub(crate) pos: usize,
    pub(crate) col: usize,
}

impl<'a> StrReader<'a> {
    pub fn new(slice: &'a str) -> StrReader<'a> {
        Self {
            slice,
            pos: 0,
            col: 0,
        }
    }
}

pub trait QueryUntil {
    fn position_until<P>(&mut self, predicate: P) -> usize
        where
            Self: Sized,
            P: FnMut(usize, u8, u8) -> ControlFlow<usize, usize>;
}

impl<'a> QueryUntil for Windows<'a, u8> {
    #[inline]
    fn position_until<P>(&mut self, predicate: P) -> usize
        where
            Self: Sized,
            P: FnMut(usize, u8, u8) -> ControlFlow<usize, usize>,
    {
        #[inline]
        fn check<'a>(
            mut predicate: impl FnMut(usize, u8, u8) -> ControlFlow<usize, usize>,
        ) -> impl FnMut(usize, &'a [u8]) -> ControlFlow<usize, usize> {
            move |pos, x| predicate(pos, x[0], x[1])
        }

        match self.try_fold(0usize, check(predicate)) {
            Break(x) | Continue(x) => x,
        }
    }
}

pub(crate) enum IndentType {
    None,
    Less,
    Equal,
    LessOrEqual,
}

impl IndentType {
    #[inline]
    pub(crate) fn compare(&self, lhs: u32, rhs: u32) -> bool {
        match self {
            IndentType::Less => lhs < rhs,
            IndentType::Equal => lhs == rhs,
            IndentType::LessOrEqual => lhs <= rhs,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub(crate) fn is_valid(&self, lhs: u32, rhs: u32) -> bool {
        match self {
            IndentType::Less => lhs + 1 < rhs,
            IndentType::Equal => lhs + 1 <= rhs,
            IndentType::LessOrEqual => lhs + 1 <= rhs,
            _ => unreachable!(),
        }
    }
}

pub(crate) trait Reader {
    fn eof(&self) -> bool;
    fn pos(&self) -> usize;
    fn col(&self) -> usize;
    fn peek_byte_at(&self, offset: i8) -> Option<u8>;
    fn peek_byte(&self) -> Option<u8>;
    fn peek_byte_is(&self, needle: u8) -> bool {
        match self.peek_byte() {
            Some(x) if x == needle => true,
            _ => false,
        }
    }
    fn peek_byte_at_check(&self, offset: i8, check: fn(u8) -> bool) -> bool {
        match self.peek_byte_at(offset) {
            Some(x) if check(x) => true,
            _ => false,
        }
    }
    fn position_until<L>(&self, lookahead_predicate: L) -> usize
        where
            L: FnMut(usize, u8, u8) -> ControlFlow<usize, usize>;
    fn consume_bytes(&mut self, amount: usize);
    fn slice_bytes(&self, start: usize, end: usize) -> &[u8];
    fn try_read_slice_exact(&mut self, needle: &str) -> bool;
    fn find_next_whitespace(&self) -> Option<usize>;
    fn find_fast2_offset(&self, needle1: u8, needle2: u8) -> Option<(usize, usize)>;
    fn skip_space_tab(&mut self, allow_tab: bool) -> usize;
    fn try_read_indent(&mut self, indent: u32, indent_type: IndentType) -> u32;
    fn read_break(&mut self) -> Option<(usize, usize)>;
    fn skip_whitespace(&mut self) -> usize;
    fn read_line(&mut self) -> (usize, usize);
    fn read_non_comment_line(&mut self) -> (usize, usize);
}

impl<'r> Reader for StrReader<'r> {
    #[inline]
    fn eof(&self) -> bool {
        self.pos >= self.slice.as_bytes().len()
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn col(&self) -> usize {
        self.col
    }

    fn peek_byte_at(&self, offset: i8) -> Option<u8> {
        let new_pos = if offset >= 0 {
            self.pos + offset as usize
        } else {
            self.pos - offset.abs() as usize
        };
        match self.slice.as_bytes().get(new_pos) {
            Some(x) => Some(*x),
            _ => None,
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        match self.slice.as_bytes().get(self.pos) {
            Some(x) => Some(*x),
            _ => None,
        }
    }

    fn position_until<P>(&self, predicate: P) -> usize
        where
            P: FnMut(usize, u8, u8) -> ControlFlow<usize, usize>,
    {
        self.slice.as_bytes()[self.pos..]
            .windows(2)
            .position_until(predicate)
    }

    #[inline(always)]
    fn consume_bytes(&mut self, amount: usize) {
        self.pos += amount;
        self.col += amount;
    }

    fn slice_bytes(&self, start: usize, end: usize) -> &'r [u8] {
        &self.slice.as_bytes()[start..end]
    }

    #[inline(always)]
    fn try_read_slice_exact(&mut self, needle: &str) -> bool {
        if self.slice.len() < self.pos + needle.len() {
            return false;
        }
        if self.slice.as_bytes()[self.pos..self.pos + needle.len()].starts_with(needle.as_bytes()) {
            self.pos += needle.len();
            return true;
        }
        false
    }

    fn find_next_whitespace(&self) -> Option<usize> {
        self.slice.as_bytes()[self.pos..]
            .iter()
            .position(|p| is_whitespace(*p))
    }

    fn find_fast2_offset(&self, needle1: u8, needle2: u8) -> Option<(usize, usize)> {
        if let Some(n) = memchr2(needle1, needle2, &self.slice.as_bytes()[self.pos..]) {
            return Some((self.pos, self.pos + n));
        }
        None
    }

    fn skip_space_tab(&mut self, allow_tab: bool) -> usize {
        let n = self.slice.as_bytes()[self.pos..]
            .iter()
            .position(|b| *b != b' ' && !(allow_tab && *b == b'\t'))
            .unwrap_or(0);
        self.consume_bytes(n);
        n
    }

    fn try_read_indent(&mut self, indent: u32, indent_type: IndentType) -> bool {
        let consume = match self.slice.as_bytes()[self.pos..]
            .iter()
            .try_fold(0u32, |prev, &x| {
                if x == b' ' && indent_type.is_valid(prev, indent) {
                    Continue(prev + 1)
                } else {
                    Break(prev)
                }
            })
        {
            Continue(x) | Break(x) => x,
        };
        self.consume_bytes(consume as usize);
        indent_type.compare(consume, indent)
    }

    fn read_break(&mut self) -> Option<(usize, usize)> {
        let start = self.pos;
        if self.peek_byte_is(b'\n') {
            self.pos += 1;
            self.col = 0;
            Some((start, start + 1))
        } else if self.peek_byte_is(b'\r') {
            let amount = match self.slice.as_bytes().get(start + 1) {
                Some(b'\n') => 2,
                _ => 1,
            };
            self.col = 0;
            self.pos += amount;
            Some((start, start + amount))
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) -> usize {
        let n = self.slice.as_bytes()[self.pos..]
            .iter()
            .position(|b| !is_whitespace(*b))
            .unwrap_or(0);
        self.consume_bytes(n);
        n
    }

    fn read_line(&mut self) -> (usize, usize) {
        let start = self.pos;
        let content = &self.slice.as_bytes()[start..];
        let (n, consume) = memchr::memchr2_iter(b'\r', b'\n', content)
            .next()
            .map_or((0, 0), |p| {
                if content[p] == b'\r' && p < content.len() - 1 && content[p + 1] == b'\n' {
                    (p, p + 2)
                } else {
                    (p, p + 1)
                }
            });
        self.consume_bytes(consume);
        self.col = 0;
        (start, start + n)
    }

    fn read_non_comment_line(&mut self) -> (usize, usize) {
        let start = self.pos;
        let content = &self.slice.as_bytes()[start..];
        let mut iter = memchr::memchr3_iter(b'\r', b'\n', b'#', content);
        let mut end = self.pos;
        let mut consume = 0;

        if let Some((new_end, c)) = iter.next().map(|p| (p, content[p])) {
            end = new_end;
            consume = end + 1;

            if c == b'\n' {
                self.consume_bytes(consume);
                self.col = 0;
                return (start, end);
            }
        }
        while let Some(pos) = iter.next() {
            let ascii = content[pos];
            if ascii == b'\r' && pos < content.len() - 1 && content[pos + 1] == b'\n' {
                self.consume_bytes(pos + 2);
                self.col = 0;
                return (start, end);
            } else if ascii == b'\r' || ascii == b'\n' {
                self.consume_bytes(pos + 1);
                self.col = 0;
                return (start, end);
            }
        }

        (start, end)
    }
}

impl<'r> StrReader<'r> {}

#[test]
pub fn test_readline() {
    let mut win_reader = StrReader::new("#   |\r\n");
    let mut lin_reader = StrReader::new("#   |\n");
    let mut mac_reader = StrReader::new("#   |\r");

    assert_eq!((0, 5), win_reader.read_line());
    assert_eq!(None, win_reader.peek_byte());
    assert_eq!(0, win_reader.col);

    assert_eq!((0, 5), lin_reader.read_line());
    assert_eq!(None, lin_reader.peek_byte());
    assert_eq!(0, lin_reader.col);

    assert_eq!((0, 5), mac_reader.read_line());
    assert_eq!(None, mac_reader.peek_byte());
    assert_eq!(0, mac_reader.col);
}

#[test]
pub fn test_read2lines() {
    let mut win_reader = StrReader::new("#   |\r\n \r\n");
    let mut lin_reader = StrReader::new("#   |\n\n");
    let mut mac_reader = StrReader::new("#   |\r\r");

    assert_eq!((0, 5), win_reader.read_line());
    assert_eq!(Some(b' '), win_reader.peek_byte());
    assert_eq!(0, win_reader.col);
    assert_eq!((7, 8), win_reader.read_line());
    assert_eq!(0, win_reader.col);
    assert_eq!(None, win_reader.peek_byte());

    assert_eq!((0, 5), lin_reader.read_line());
    assert_eq!(Some(b'\n'), lin_reader.peek_byte());
    assert_eq!(0, lin_reader.col);
    assert_eq!((6, 6), lin_reader.read_line());
    assert_eq!(0, lin_reader.col);
    assert_eq!(None, lin_reader.peek_byte());

    assert_eq!((0, 5), mac_reader.read_line());
    assert_eq!(Some(b'\r'), mac_reader.peek_byte());
    assert_eq!(0, mac_reader.col);
    assert_eq!((6, 6), mac_reader.read_line());
    assert_eq!(0, mac_reader.col);
    assert_eq!(None, mac_reader.peek_byte());
}

#[test]
pub fn read_non_comment_line() {
    let mut win_reader = StrReader::new("   # # \r\n");
    let mut mac_reader = StrReader::new("   # # \r");
    let mut lin_reader = StrReader::new("   # # \n");

    assert_eq!((0, 3), win_reader.read_non_comment_line());
    assert_eq!(None, win_reader.peek_byte());
    assert_eq!(9, win_reader.pos);
    assert_eq!(0, win_reader.col);

    assert_eq!((0, 3), mac_reader.read_non_comment_line());
    assert_eq!(None, mac_reader.peek_byte());
    assert_eq!(8, mac_reader.pos);
    assert_eq!(0, mac_reader.col);

    assert_eq!((0, 3), lin_reader.read_non_comment_line());
    assert_eq!(None, lin_reader.peek_byte());
    assert_eq!(8, lin_reader.pos);
    assert_eq!(0, lin_reader.col);
}

#[test]
pub fn skip_whitespace() {
    assert_eq!(0, StrReader::new("null").skip_whitespace());
    assert_eq!(0, StrReader::new("").skip_whitespace());
    assert_eq!(1, StrReader::new(" null").skip_whitespace());
    assert_eq!(2, StrReader::new("\t null").skip_whitespace());
}

#[test]
pub fn test_position_until() {
    let look_ahead = StrReader::new("test #");

    assert_eq!(
        4,
        look_ahead.position_until(|pos, x0, x1| {
            if is_tab_space(x0) && x1 == b'#' {
                Break(pos)
            } else {
                Continue(pos + 1)
            }
        })
    );

    let look_behind = StrReader::new("test# ");

    assert_eq!(
        4,
        look_behind.position_until(|pos, x0, x1| {
            if x0 == b'#' && is_tab_space(x1) {
                Break(pos)
            } else {
                Continue(pos + 1)
            }
        })
    );

    let look_any = StrReader::new("test# ");

    assert_eq!(
        5,
        look_any.position_until(|pos, x0, x1| {
            if is_tab_space(x0) {
                Break(pos)
            } else if is_tab_space(x1) {
                Break(pos + 1)
            } else {
                Continue(pos + 1)
            }
        })
    );
}

#[test]
pub fn test_try_read_indent() {
    fn try_read(input: &str, indent: u32, indent_type: IndentType, success: bool, expected_pos: usize) {
        let mut reader = StrReader::new(input);
        let read = reader.try_read_indent(indent, indent_type);

        assert_eq!(success, read);
        assert_eq!(expected_pos, reader.pos);
    }

    try_read("     #", 3, IndentType::Equal, true, 3);
    try_read("     #", 6, IndentType::Equal, false, 5);

    try_read("     #", 4, IndentType::Less, true, 3);
    try_read("     #", 0, IndentType::Less, false, 0);

    try_read("     #", 4, IndentType::LessOrEqual, true, 4);
    try_read("     #", 7, IndentType::LessOrEqual, true, 5);
}




#[inline]
pub(crate) fn is_tab_space(b: u8) -> bool {
    match b {
        b' ' | b'\t' => true,
        _ => false,
    }
}

#[inline]
pub(crate) fn is_whitespace(chr: u8) -> bool {
    match chr {
        b' ' | b'\t' | b'\r' | b'\n' => true,
        _ => false,
    }
}

#[inline]
pub(crate) fn is_indicator(chr: u8) -> bool {
    match chr {
        b'-' | b'?' | b':' | b',' | b'[' | b']' | b'{' | b'}' | b'#' | b'&' | b'*' | b'!'
        | b'|' | b'>' | b'\'' | b'"' | b'%' | b'@' | b'`' => true,
        _ => false,
    }
}

#[inline]
pub(crate) fn is_flow_indicator(chr: u8) -> bool {
    match chr {
        b',' | b'[' | b']' | b'{' | b'}' => true,
        _ => false,
    }
}
