use std::collections::VecDeque;
use std::ops::ControlFlow::{Break, Continue};
use std::ops::{RangeFrom, RangeInclusive};
use std::usize;

use memchr::{memchr, memchr3_iter};

use reader::{is_flow_indicator, ns_plain_safe};
use ErrorType::ExpectedIndent;

use crate::tokenizer::lexer::LexerState;
use crate::tokenizer::lexer::LexerState::{BlockMap, BlockMapExp, BlockSeq};
use crate::tokenizer::reader::{
    is_indicator, is_uri_char, is_white_tab, is_white_tab_or_break, ChompIndicator, LookAroundBytes,
};
use crate::tokenizer::ErrorType::{TagNotTerminated, UnexpectedComment};
use crate::tokenizer::LexerToken::*;
use crate::tokenizer::{reader, ErrorType, Reader, Slicer};

use super::reader::is_newline;

pub struct StrReader<'a> {
    pub slice: &'a [u8],
    pub(crate) pos: usize,
    pub(crate) col: usize,
    pub(crate) line: usize,
}

enum Flow {
    Continue,
    Break,
    Error(usize),
}

impl<'a> From<&'a str> for StrReader<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            slice: value.as_bytes(),
            pos: 0,
            col: 0,
            line: 0,
        }
    }
}

impl<'a> From<&'a [u8]> for StrReader<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self {
            slice: value,
            pos: 0,
            col: 0,
            line: 0,
        }
    }
}

impl<'a> Slicer<'a> for StrReader<'a> {
    fn slice(&self, start: usize, end: usize) -> &'a [u8] {
        unsafe { self.slice.get_unchecked(start..end) }
    }
}

impl<'a> StrReader<'a> {
    #[inline]
    fn eof_or_pos(&self, pos: usize) -> usize {
        pos.min(self.slice.len() - 1)
    }

    #[inline]
    fn get_lookahead_iterator(&self, range: RangeInclusive<usize>) -> LookAroundBytes {
        LookAroundBytes::new(self.slice, range)
    }

    #[inline]
    fn peek_byte_unwrap(&self, offset: usize) -> u8 {
        match self.slice.get(self.pos + offset) {
            Some(x) => *x,
            _ => b'\0',
        }
    }

    #[inline]
    fn count_space_tab_range_from(&self, range: RangeFrom<usize>, allow_tab: bool) -> usize {
        match self.slice[range]
            .iter()
            .try_fold(0usize, |acc, x| reader::is_tab_space(acc, *x, allow_tab))
        {
            Continue(x) | Break(x) => x,
        }
    }

    fn find_next_whitespace(&self) -> Option<usize> {
        self.slice[self.pos..]
            .iter()
            .position(|p| is_white_tab_or_break(*p))
    }

    fn skip_n_spaces(&mut self, num_spaces: usize, prev_indent: usize) -> Flow {
        let count = self.slice[self.pos..]
            .iter()
            .enumerate()
            .take_while(|&(count, &x)| x == b' ' && count < num_spaces)
            .count();

        if count == prev_indent {
            Flow::Break
        } else if count != num_spaces {
            Flow::Error(count)
        } else {
            self.pos += count;
            Flow::Continue
        }
    }

    fn get_line_offset(&self) -> (usize, usize, usize) {
        let slice = self.slice;
        let start = self.pos;
        let remaining = slice.len() - start;
        let content = &slice[start..];
        let (n, newline) = memchr::memchr2_iter(b'\r', b'\n', content).next().map_or(
            (remaining, remaining),
            |p| {
                if content[p] == b'\r' && p < content.len() - 1 && content[p + 1] == b'\n' {
                    (p, 2)
                } else {
                    (p, 1)
                }
            },
        );
        (start, start + n, start + n + newline)
    }

    fn read_non_comment_line(&mut self) -> (usize, usize) {
        let start = self.pos;
        let content = &self.slice[start..];
        let mut iter = memchr3_iter(b'\r', b'\n', b'#', content);
        let mut end = self.pos;
        let consume: usize;

        if let Some((new_end, c)) = iter.next().map(|p| (p, content[p])) {
            end = new_end;
            consume = end + 1;

            if c == b'\n' {
                self.consume_bytes(consume);
                self.col = 0;
                self.line += 1;
                return (start, end);
            }
        }
        for pos in iter {
            let ascii = content[pos];
            if ascii == b'\r' && pos < content.len() - 1 && content[pos + 1] == b'\n' {
                self.consume_bytes(pos + 2);
                self.col = 0;
                self.line += 1;
                return (start, end);
            } else if ascii == b'\r' || ascii == b'\n' {
                self.consume_bytes(pos + 1);
                self.col = 0;
                self.line += 1;
                return (start, end);
            }
        }

        (start, end)
    }
}

impl<'r> Reader<()> for StrReader<'r> {
    #[inline]
    fn eof(&self) -> bool {
        self.pos >= self.slice.len()
    }

    #[inline]
    fn col(&self) -> usize {
        self.col
    }

    #[inline]
    fn line(&self) -> usize {
        self.line
    }

    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    fn peek_chars(&self) -> &[u8] {
        let max = std::cmp::min(self.slice.len(), self.pos + 3);
        &self.slice[self.pos..max]
    }

    #[inline]
    fn peek_byte(&self) -> Option<u8> {
        self.slice.get(self.pos).copied()
    }

    #[inline]
    fn peek_byte2(&self) -> Option<u8> {
        self.slice.get(self.pos + 1).copied()
    }

    #[inline]
    fn peek_byte_at(&self, offset: usize) -> Option<u8> {
        self.slice.get(self.pos + offset).copied()
    }

    #[inline]
    fn count_space_tab(&self, allow_tab: bool) -> usize {
        self.count_space_tab_range_from(self.pos.., allow_tab)
    }

    #[inline(always)]
    fn consume_bytes(&mut self, amount: usize) -> usize {
        self.pos += amount;
        self.col += amount;
        self.pos
    }
    #[inline(always)]
    fn try_read_slice_exact(&mut self, needle: &str) -> bool {
        if self.slice.len() < self.pos + needle.len() {
            return false;
        }
        if self.slice[self.pos..self.pos + needle.len()].starts_with(needle.as_bytes()) {
            self.pos += needle.len();
            return true;
        }
        false
    }

    #[inline]
    fn read_line(&mut self) -> (usize, usize) {
        let (start, end, consume) = self.get_line_offset();
        self.pos = consume;
        self.col = 0;
        (start, end)
    }

    fn not_safe_char(&self) -> bool {
        match self.slice[self.pos..] {
            [b'-', x, ..] => is_white_tab(x),
            [b'?', x, ..] => is_white_tab_or_break(x),
            [b':', x, ..] => is_white_tab_or_break(x),
            [x, ..] if is_white_tab_or_break(x) || is_indicator(x) => true,
            _ => false,
        }
    }

    fn try_read_yaml_directive(&mut self, tokens: &mut VecDeque<usize>) -> bool {
        if self.peek_byte_is(b'%') {
            if self.try_read_slice_exact("%YAML") {
                self.skip_space_tab(true);
                if let Some(x) = self.find_next_whitespace() {
                    tokens.push_back(DirectiveYaml as usize);
                    tokens.push_back(self.pos);
                    tokens.push_back(self.pos + x);

                    self.consume_bytes(x);
                    self.read_line();
                }
            } else {
                let tag = if self.try_read_slice_exact("%TAG") {
                    DirectiveTag
                } else {
                    DirectiveReserved
                };
                self.skip_space_tab(true);
                let x = self.read_non_comment_line();
                if x.0 != x.1 {
                    tokens.push_back(tag as usize);
                    tokens.push_back(x.0);
                    tokens.push_back(x.1);
                }
            }
            true
        } else {
            false
        }
    }

    fn read_plain_one_line(
        &mut self,
        offset_start: Option<usize>,
        had_comment: &mut bool,
        in_flow_collection: bool,
    ) -> (usize, usize, Option<ErrorType>) {
        let start = offset_start.unwrap_or(self.pos);
        let (_, line_end, _) = self.get_line_offset();
        let end = self.consume_bytes(1);
        let mut pos_end = end;
        let line_end = StrReader::eof_or_pos(self, line_end);
        let mut end_of_str = end;

        for (prev, curr, next, pos) in self.get_lookahead_iterator(end..=line_end) {
            // ns-plain-char  prevent ` #`
            if curr == b'#' && is_white_tab_or_break(prev) {
                // if we encounter two or more comment print error and try to recover
                return if *had_comment {
                    self.pos = line_end;
                    (start, end_of_str, Some(UnexpectedComment))
                } else {
                    *had_comment = true;
                    self.pos = line_end;
                    (start, end_of_str, None)
                };
            }

            // ns-plain-char prevent `: `
            // or `:{`  in flow collections
            if curr == b':' && !ns_plain_safe(next, in_flow_collection) {
                pos_end = end_of_str;
                break;
            }

            // // if current character is a flow indicator, break
            if in_flow_collection && is_flow_indicator(curr) {
                pos_end = end_of_str;
                break;
            }

            if is_white_tab_or_break(curr) {
                if is_newline(curr) {
                    pos_end = line_end;
                    break;
                }
                pos_end = pos;
            } else {
                end_of_str = pos + 1;
                pos_end = end_of_str;
            }
        }
        self.pos = pos_end;
        (start, end_of_str, None)
    }

    fn read_block_scalar(
        &mut self,
        literal: bool,
        curr_state: &LexerState,
        block_indent: usize,
        tokens: &mut VecDeque<usize>,
        errors: &mut Vec<ErrorType>,
    ) {
        self.consume_bytes(1);
        let mut chomp = ChompIndicator::Clip;
        let mut indentation: usize = 0;

        match (self.peek_byte_unwrap(0), self.peek_byte_unwrap(1)) {
            (_, b'0') | (b'0', _) => {
                self.consume_bytes(2);
                tokens.push_back(ErrorToken as usize);
                errors.push(ErrorType::ExpectedChompBetween1and9);
                return;
            }
            (b'-', len) | (len, b'-') if matches!(len, b'1'..=b'9') => {
                self.consume_bytes(2);
                chomp = ChompIndicator::Strip;
                indentation = block_indent + (len - b'0') as usize;
            }
            (b'+', len) | (len, b'+') if matches!(len, b'1'..=b'9') => {
                self.consume_bytes(2);
                chomp = ChompIndicator::Keep;
                indentation = block_indent + (len - b'0') as usize;
            }
            (b'-', _) => {
                self.consume_bytes(1);
                chomp = ChompIndicator::Strip;
            }
            (b'+', _) => {
                self.consume_bytes(1);
                chomp = ChompIndicator::Keep;
            }
            (len, _) if matches!(len, b'1'..=b'9') => {
                self.consume_bytes(1);
                indentation = block_indent + (len - b'0') as usize;
            }
            _ => {}
        }

        let token = if literal {
            ScalarLit as usize
        } else {
            ScalarFold as usize
        };

        // allow comment in first line of block scalar
        self.skip_space_tab(true);
        match self.peek_byte() {
            Some(b'#' | b'\r' | b'\n') => {
                self.read_line();
            }
            Some(chr) => {
                self.read_line();
                tokens.push_back(ErrorToken as usize);
                errors.push(ErrorType::UnexpectedSymbol(chr as char));
                return;
            }
            _ => {}
        }

        let mut new_line_token = 0;

        tokens.push_back(token);
        if self.eof() {
            tokens.push_back(ScalarEnd as usize);
            return;
        }
        let mut trailing = vec![];
        let mut is_trailing_comment = false;
        let mut previous_indent = 0;
        while !self.eof() {
            match (self.peek_byte_unwrap(block_indent), curr_state) {
                (b'-', BlockSeq(ind)) | (b':', BlockMapExp(ind, _)) => {
                    if self.col + block_indent == *ind as usize {
                        self.consume_bytes(block_indent);
                        break;
                    }
                }
                (_, BlockMap(ind, _))
                    if *ind as usize == block_indent && block_indent < previous_indent =>
                {
                    break;
                }
                _ => {}
            }

            // count indents important for folded scalars
            let newline_indent = self.count_space_tab(false);

            if !is_trailing_comment
                && newline_indent < indentation
                && self.peek_byte_unwrap(newline_indent) == b'#'
            {
                trailing.push(NewLine as usize);
                trailing.push(new_line_token - 1);
                is_trailing_comment = true;
                new_line_token = 1;
            };

            let newline_is_empty = self.peek_byte_at(newline_indent).map_or(false, is_newline)
                || (is_trailing_comment && self.peek_byte_unwrap(newline_indent) == b'#');

            if indentation == 0 && newline_indent > 0 && !newline_is_empty {
                indentation = newline_indent;
            }

            if newline_is_empty {
                new_line_token += 1;
                self.read_line();
                continue;
            } else if let Flow::Error(actual) = self.skip_n_spaces(indentation, block_indent) {
                tokens.push_back(ErrorToken as usize);
                errors.push(ExpectedIndent {
                    actual,
                    expected: indentation,
                });
                break;
            }

            let (start, end) = self.read_line();
            if start != end {
                if new_line_token > 0 {
                    if !literal && previous_indent == newline_indent {
                        tokens.push_back(NewLine as usize);
                        tokens.push_back(new_line_token - 1);
                    } else {
                        tokens.push_back(NewLine as usize);
                        tokens.push_back(new_line_token);
                    }
                }
                previous_indent = newline_indent;
                tokens.push_back(start);
                tokens.push_back(end);
                new_line_token = 1;
            }
        }
        match chomp {
            ChompIndicator::Keep => {
                if is_trailing_comment {
                    new_line_token = 1;
                }
                trailing.insert(0, NewLine as usize);
                trailing.insert(1, new_line_token);
                tokens.extend(trailing);
            }
            ChompIndicator::Clip => {
                trailing.insert(0, NewLine as usize);
                trailing.insert(1, 1);
                tokens.extend(trailing);
            }
            ChompIndicator::Strip => {}
        }
    }

    fn read_double_quote(
        &mut self,
        prev_indent: usize,
        _is_implicit: bool,
        is_multiline: &mut bool,
        errors: &mut Vec<ErrorType>,
    ) -> Vec<usize> {
        let start_str = self.consume_bytes(1);
        let mut quote_token = (start_str, start_str);
        let mut tokens = vec![ScalarDoubleQuote as usize];
        let mut last_non_space = 0;
        let mut newspaces = None;

        while !self.eof() {
            let (line_start, line_end, _) = self.get_line_offset();
            if line_start == line_end {
                if *is_multiline {
                    quote_token.0 = last_non_space;
                } else {
                    quote_token.1 = last_non_space;
                }

                *is_multiline = true;

                emit_token(&mut quote_token, &mut newspaces, &mut tokens);
                newspaces = Some(self.skip_separation_spaces(true).saturating_sub(1));
                quote_token.0 = self.pos;
                if prev_indent != 0 && self.col < prev_indent {
                    tokens.insert(0, ErrorToken as usize);
                    errors.push(ErrorType::ExpectedIndent {
                        actual: self.col,
                        expected: prev_indent,
                    })
                }
                continue;
            }
            let haystack = &self.slice[line_start..line_end];
            if let Some(find_pos) = memchr::memchr3(b'"', b' ', b'\t', haystack) {
                self.consume_bytes(find_pos.saturating_sub(1));
                match self.peek_chars() {
                    [b'\\', b'"', ..] => {
                        quote_token.1 = self.pos;
                        emit_token(&mut quote_token, &mut newspaces, &mut tokens);
                        quote_token.0 = self.pos + 1;
                        self.consume_bytes(2);
                    }
                    [_, b'"', ..] => {
                        quote_token.1 = self.pos + 1;
                        emit_token(&mut quote_token, &mut newspaces, &mut tokens);
                        self.consume_bytes(2);
                        break;
                    }
                    [_, b' ' | b'\t', ..] | [b' ' | b'\t'] => {
                        if *is_multiline {
                            last_non_space = self.pos;
                            self.consume_bytes(1);
                            self.skip_space_tab(true);
                        } else {
                            last_non_space = self.consume_bytes(1);
                            self.skip_space_tab(true);
                        }
                    }
                    _ => {}
                }
            } else {
                let amount = line_end - line_start;
                last_non_space = self.consume_bytes(amount);
            }
        }
        tokens.push(ScalarEnd as usize);
        tokens
    }

    fn read_single_quote(&mut self, is_implicit: bool) -> Vec<usize> {
        self.consume_bytes(1);
        let mut tokens = Vec::new();
        tokens.push(ScalarSingleQuote as usize);

        while !self.eof() {
            let (line_start, line_end, _) = self.get_line_offset();
            let pos = memchr::memchr(b'\'', &self.slice[line_start..line_end]);
            match pos {
                Some(len) => {
                    // Converts double '' to ' hence why we consume one extra char
                    let offset = len + 1;
                    if self.slice.get(self.pos + offset).copied() == Some(b'\'') {
                        tokens.push(line_start);
                        tokens.push(line_start + len + 1);
                        self.consume_bytes(len + 2);
                        continue;
                    } else {
                        tokens.push(line_start);
                        tokens.push(line_start + len);
                        self.consume_bytes(len + 1);
                        break;
                    }
                }
                None => {
                    tokens.push(line_start);
                    tokens.push(line_end);
                    tokens.push(NewLine as usize);
                    tokens.push(0);
                    self.read_line();
                    self.skip_space_tab(is_implicit);
                }
            }
        }
        tokens.push(ScalarEnd as usize);
        tokens
    }

    fn skip_separation_spaces(&mut self, allow_comments: bool) -> usize {
        let mut num_breaks = 0;
        let mut found_eol = true;
        while !self.eof() {
            self.skip_space_tab(true);

            if allow_comments && self.peek_byte_is(b'#') {
                self.read_line();
                found_eol = true;
                num_breaks += 1;
            }

            if self.read_break().is_some() {
                num_breaks += 1;
                found_eol = true;
            }

            if !found_eol {
                break;
            } else {
                self.skip_space_tab(false);
                found_eol = false;
            }
        }
        num_breaks
    }

    fn consume_anchor_alias(&mut self) -> (usize, usize) {
        let start = self.consume_bytes(1);

        let amount = self.slice[self.pos..]
            .iter()
            .position(|p| is_white_tab_or_break(*p) || is_flow_indicator(*p))
            .unwrap_or(self.slice.len() - self.pos);
        self.consume_bytes(amount);
        (start, start + amount)
    }

    fn read_tag(&self) -> Result<(usize, usize), ErrorType> {
        if let Some(mid) = memchr(b'!', &self.slice[self.pos..]) {
            let mid = self.pos + mid + 1;
            let end = self.slice[mid..]
                .iter()
                .position(|c| !is_uri_char(*c))
                .unwrap_or(self.slice.len() - self.pos);
            return Ok((mid, mid + end));
        }
        Err(TagNotTerminated)
    }

    fn read_break(&mut self) -> Option<(usize, usize)> {
        let start = self.pos;
        if self.peek_byte_is(b'\n') {
            self.pos += 1;
            self.col = 0;
            self.line += 1;
            Some((start, start + 1))
        } else if self.peek_byte_is(b'\r') {
            let amount = match self.slice.get(start + 1) {
                Some(b'\n') => 2,
                _ => 1,
            };
            self.col = 0;
            self.pos += amount;
            self.line += 1;
            Some((start, start + amount))
        } else {
            None
        }
    }
}

fn emit_token(
    quote_token: &mut (usize, usize),
    newspaces: &mut Option<usize>,
    tokens: &mut Vec<usize>,
) {
    if quote_token.0 != quote_token.1 {
        if let Some(newspace) = newspaces.take() {
            tokens.push(NewLine as usize);
            tokens.push(newspace);
        }
        tokens.push(quote_token.0);
        tokens.push(quote_token.1);
        quote_token.0 = quote_token.1;
    }
}

#[test]
pub fn test_plain_scalar() {
    let mut reader = StrReader::from("ab  \n xyz ");
    let mut had_comment = true;
    let (start, end, _) = reader.read_plain_one_line(None, &mut had_comment, false);
    assert_eq!("ab".as_bytes(), &reader.slice[start..end]);
    reader.skip_separation_spaces(false);
    let (start, end, _) = reader.read_plain_one_line(None, &mut had_comment, false);
    assert_eq!("xyz".as_bytes(), &reader.slice[start..end]);
}
