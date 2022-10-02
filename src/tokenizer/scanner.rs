use std::collections::VecDeque;

use State::{InBlockMap, InBlockScalar, InBlockSeq, InFlowMap, InFlowSeq};

use crate::error::YamlError;
use crate::tokenizer::event::DirectiveType;
use crate::tokenizer::event::YamlEvent::{Directive, DocEnd};
use crate::tokenizer::iter::ErrorType::StartedBlockInFlow;
use crate::tokenizer::iter::{ErrorType, StrIterator};
use crate::tokenizer::reader::{Reader, StrReader};
use crate::tokenizer::scanner::Control::{Continue, Eof};
use crate::tokenizer::scanner::QuoteType::{Double, Plain, Single};
use crate::tokenizer::scanner::ScannerContext::{BlockIn, BlockOut, FlowOut};
use crate::tokenizer::scanner::State::{BlockNode, InFlowScalar, StreamEnd, StreamStart};

#[derive(Clone)]
pub struct Scanner {
    pub(crate) curr_state: State,
    closing: bool,
    tokens: VecDeque<SpanToken>,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            curr_state: StreamStart,
            closing: false,
            tokens: VecDeque::new(),
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum ScannerContext {
    BlockIn,
    BlockOut,
    BlockKey,
    FlowIn,
    FlowOut,
    FlowKey,
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum State {
    StreamStart,
    BlockNode,
    StreamEnd,
    InFlowSeq,
    InBlockSeq,
    InFlowMap,
    InBlockMap,
    InMap,
    InFlowScalar(QuoteType),
    InBlockScalar,
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum QuoteType {
    Plain,
    Single,
    Double,
}

impl Default for QuoteType {
    fn default() -> Self {
        Plain
    }
}

#[derive(PartialEq)]
pub enum Control {
    Continue,
    Eof,
    End,
}

impl Scanner {
    pub fn from_str_reader(slice: &str) -> StrIterator<'_> {
        StrIterator {
            state: Default::default(),
            reader: StrReader::new(slice),
        }
    }

    pub(crate) fn emit_end_of_stream(&mut self) {
        match self.curr_state {
            BlockNode => self.tokens.push_back(SpanToken::DocEnd),
            _ => (),
        }
        self.tokens.push_back(SpanToken::StreamEnd);
        self.curr_state = StreamEnd;
        self.closing = true;
    }

    pub(crate) fn pop_token(&mut self) -> Option<SpanToken> {
        self.tokens.pop_front()
    }

    pub(crate) fn next_state<R: Reader>(&mut self, reader: &mut R) -> Control {
        if reader.eof() && !self.closing {
            self.closing = true;
            self.emit_end_of_stream()
        }
        match self.curr_state {
            StreamStart => self.read_start_stream(reader),
            DocStart => self.read_block_node(reader, 0, BlockOut),
            // InFlowScalar(Plain) => self.read_flow_scalar_unquote(reader, ),
            _ => (),
        };
        if !self.tokens.is_empty() || !self.closing {
            return Continue;
        }

        return Eof;
    }

    pub(crate) fn read_start_stream<R: Reader>(&mut self, reader: &mut R) {
        self.try_skip_comments(reader);
        self.tokens.push_back(SpanToken::StreamStart);
        if reader.peek_byte_is(b'%') {
            if reader.try_read_slice_exact("%YAML") {
                reader.skip_space_tab();
                if let Some(x) = reader.find_next_whitespace() {
                    self.tokens.push_back(SpanToken::Directive(
                        DirectiveType::Yaml,
                        reader.pos(),
                        reader.pos() + x,
                    ));
                    reader.consume_bytes(x);
                    reader.read_line();
                }
            } else {
                let tag = if reader.try_read_slice_exact("%TAG") {
                    DirectiveType::Tag
                } else {
                    DirectiveType::Reserved
                };
                reader.skip_space_tab();
                let x = reader.read_non_comment_line();
                if x.0 != x.1 {
                    self.tokens.push_back(SpanToken::Directive(tag, x.0, x.1));
                }
            }
            if !reader.try_read_slice_exact("---") {
                self.tokens
                    .push_back(SpanToken::ErrorToken(ErrorType::ExpectedDocumentStart));
            }
        }
        self.curr_state = BlockNode;
        self.tokens.push_back(SpanToken::DocStart);
    }

    pub(crate) fn read_block_node<R: Reader>(
        &mut self,
        reader: &mut R,
        indent: usize,
        context: ScannerContext,
    ) {
        reader.skip_whitespace();
        if let Some(x) = reader.peek_byte() {
            match x {
                b'[' => {
                    reader.consume_bytes(1);
                    self.switch_state(InFlowSeq);
                }
                b'-' => {
                    self.switch_state(InBlockSeq); // Can be re-consumed as scalar `--`
                }
                b'{' => {
                    reader.consume_bytes(1);
                    self.switch_state(InFlowMap);
                }
                b'?' => {
                    self.switch_state(InBlockMap); // Can be re-consumed as `??` scalar
                }
                b'\'' => {
                    reader.consume_bytes(1);
                    self.switch_state(InFlowScalar(Single));
                }
                b'"' => {
                    reader.consume_bytes(1);
                    self.switch_state(InFlowScalar(Double));
                }
                b'|' => {
                    reader.consume_bytes(1);
                    self.switch_state(InBlockScalar);
                }
                b'.' => {
                    if reader.try_read_slice_exact("...") {
                        self.curr_state = StreamEnd;
                    }
                }
                _ => {
                    self.read_flow_scalar_unquote(reader, indent, context);
                }
            }
        };
    }

    #[inline(always)]
    pub(crate) fn switch_state(&mut self, next_state: State) {
        self.curr_state = next_state;
    }

    pub(crate) fn read_flow_scalar_unquote<R: Reader>(
        &mut self,
        reader: &mut R,
        indent: usize,
        context: ScannerContext,
    ) {
        self.curr_state = InFlowScalar(Plain);
        let n = reader.skip_whitespace();
        reader.read_line();
    }

    fn try_skip_comments<T: Reader>(&self, reader: &mut T) {
        while {
            // do
            reader.skip_whitespace();
            reader.peek_byte_is(b'#')
        } {
            // while
            reader.read_line();
        }
    }
}

#[derive(Copy, Clone)]
pub enum SpanToken {
    ErrorToken(ErrorType),
    Scalar(usize, usize),
    Directive(DirectiveType, usize, usize),
    DocStart,
    DocEnd,
    StreamStart,
    StreamEnd,
}
