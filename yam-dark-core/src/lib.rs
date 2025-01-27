#![no_std]
extern crate alloc;
extern crate core_detect;

use crate::error::Error;

mod error;
mod safer_unchecked;
mod stage1;
mod stage2;
mod tokenizer;
mod util;

pub const SIMD_INPUT_LENGTH: usize = 64;
pub const SIMD_JSON_PADDING: usize = 32;

pub const EVEN_BITS: u64 = 0x5555_5555_5555_5555;
pub const ODD_BITS: u64 = !EVEN_BITS;

pub type ParseResult<T> = Result<T, Error>;

// MIT License
//
// Copyright (c) 2024 Simd-json developers
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct SillyWrapper<'de> {
    input: *mut u8,
    _marker: core::marker::PhantomData<&'de ()>,
}
