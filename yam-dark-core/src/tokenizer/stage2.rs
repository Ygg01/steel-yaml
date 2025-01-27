#![allow(unused)]

// MIT License
//
// Copyright (c) 2024 Ygg One
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::impls::{AvxScanner, NativeScanner};
use crate::tape::Node;
use crate::tokenizer::stage1::{NextFn, Stage1Scanner};
use crate::util::NoopValidator;
use crate::{ChunkyIterator, YamlChunkState};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::marker::PhantomData;
use simdutf8::basic::imp::ChunkedUtf8Validator;
use yam_core::error::{YamlError, YamlResult};

pub type ParseResult<T> = Result<T, YamlError>;

pub struct Deserializer<'de> {
    idx: usize,
    tape: Vec<Node<'de>>,
    _data: &'de PhantomData<()>,
}

pub trait Buffer {}

#[derive(Default)]
pub struct Buffers {
    string_buffer: Vec<u8>,
    structural_indexes: Vec<u32>,
}

fn fill_tape<'de, B: Buffer>(
    input: &'de [u8],
    buffer: &mut B,
    tape: &mut [Node<'de>],
) -> ParseResult<()> {
    Deserializer::fill_tape(input, buffer, tape)
}

impl<'de> Deserializer<'de> {
    fn fill_tape<B: Buffer>(
        input: &'de [u8],
        buffer: &mut B,
        tape: &mut [Node<'de>],
    ) -> YamlResult<()> {
        let mut iter = ChunkyIterator::from_bytes(input);
        let mut state = YamlParserState::default();
        let mut validator = get_validator(false);

        let next_fn = get_stage1_next::<B>();

        for chunk in iter {
            // SAFETY: The get_validator function should return the correct validator for any given
            // CPU architecture.
            // PANIC safe: the chunk is always 64 characters long
            unsafe {
                validator.update_from_chunks(chunk);
            }

            // SAFETY: The next_fn should return the correct function for any given CPU
            let chunk_state: YamlChunkState = unsafe { next_fn(chunk, buffer, &mut state) };
            state.process_chunk(buffer, &chunk_state)?;
        }

        Self::build_tape(&state, tape)
    }

    fn build_tape(state: &YamlParserState, tape: &mut [Node]) -> YamlResult<()> {
        //TODO state machine
        Ok(())
    }
}

impl Buffer for Buffers {}

/// Represents the state of the YAML parser.
///
/// This struct is used internally to keep track of various aspects of the parser's state
/// as it processes a YAML document.
///
/// # Fields (for internal use only)
///
/// - `last_indent`: The indentation level of the last parsed line.
/// - `last_col`: The column number of the last parsed character.
/// - `last_row`: The row number of the last parsed character.
/// - `is_prev_double_quotes`: Indicates whether the previous character was a double quote.
/// - `is_prev_iter_odd_single_quote`: Indicates whether the previous iteration ended with an odd number of single quotes.
/// - `is_indent_frozen`: Indicates whether the current indentation level is frozen (cannot be changed).
/// - `is_previous_white_space`: Indicates whether the previous character was whitespace.
/// - `prev_iter_inside_quote`: A bitmask indicating whether each character in the previous chunk was inside quotes.
/// - `is_in_comment`: Indicates whether the parser is currently inside a comment.
#[derive(Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct YamlParserState {
    pub(crate) structurals: Vec<usize>,
    pub(crate) byte_cols: Vec<u32>,
    pub(crate) byte_rows: Vec<u32>,
    pub(crate) indents: Vec<u32>,
    pub(crate) idx: usize,
    pub(crate) last_indent: u32,
    pub(crate) last_col: u32,
    pub(crate) last_row: u32,
    pub(crate) previous_indent: u32,
    pub(crate) prev_iter_inside_quote: u64,
    pub(crate) is_indent_running: bool,
    pub(crate) is_previous_white_space: bool,
    pub(crate) is_prev_iter_odd_single_quote: bool,
    pub(crate) is_prev_double_quotes: bool,
    pub(crate) is_in_comment: bool,
}

impl YamlParserState {
    pub(crate) fn process_chunk<B: Buffer>(
        &self,
        p0: &mut B,
        p1: &YamlChunkState,
    ) -> YamlResult<()> {
        todo!()
    }
}

/// Function that returns right validator for the right architecture
///
/// # Arguments
///
/// * `pre_checked`: `true` when working with [`core::str`] thus not requiring any validation, `false`
///   otherwise. **Note:** if your [`core::str`] isn't UTF-8 formatted this will cause Undefined behavior.
///
/// returns: `Box<dyn ChunkedUtf8Validator, Global>` a heap allocated [`ChunkedUtf8Validator`] that
/// is guaranteed to be correct for your CPU architecture.
///
#[cfg_attr(not(feature = "no-inline"), inline)]
fn get_validator(pre_checked: bool) -> Box<dyn ChunkedUtf8Validator> {
    if pre_checked {
        /// Safety: Always safe for preformatted utf8
        unsafe {
            // Is always safe for preformatted utf8
            return Box::new(NoopValidator::new());
        }
    }

    /// Safety: Only unsafe thing here is from calling right Scanner for right CPU architecture
    /// i.e. don't call Neon on x86 architecture
    unsafe {
        if core_detect::is_x86_feature_detected!("avx2") {
            Box::new(AvxScanner::validator())
        } else {
            Box::new(NativeScanner::validator())
        }
    }
}

#[cfg_attr(not(feature = "no-inline"), inline)]
fn get_stage1_next<B: Buffer>() -> NextFn<B> {
    NativeScanner::next::<B>
}
