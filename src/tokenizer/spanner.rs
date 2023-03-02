#![allow(clippy::match_like_matches_macro)]

use std::collections::VecDeque;

use ErrorType::NoDocStartAfterTag;
use SpanToken::{Alias, Anchor, DocumentStart, Separator};

use crate::tokenizer::{DirectiveType, ErrorType};
use crate::tokenizer::ErrorType::UnexpectedSymbol;
use crate::tokenizer::reader::{is_white_tab_or_break, Reader};
use crate::tokenizer::spanner::ParserState::{
    AfterDocEnd, BlockKeyExp, BlockMap, BlockSeq, BlockValExp, FlowKey, FlowKeyExp, FlowMap,
    FlowSeq, PreDocStart, RootBlock,
};
use crate::tokenizer::spanner::SpanToken::{
    ErrorToken, KeyEnd, MappingEnd, MappingStart, SequenceEnd, SequenceStart,
};
use crate::tokenizer::SpanToken::{MarkEnd, MarkStart, TagStart};

#[derive(Clone)]
pub struct Spanner {
    pub(crate) curr_state: ParserState,
    pub stream_end: bool,
    tokens: VecDeque<SpanToken>,
    stack: Vec<ParserState>,
}

impl Default for Spanner {
    fn default() -> Self {
        Self {
            stream_end: false,
            tokens: VecDeque::new(),
            curr_state: PreDocStart,
            stack: vec![],
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ParserState {
    PreDocStart,
    RootBlock,
    FlowSeq(u32),
    FlowMap(u32),
    FlowKey(u32),
    FlowKeyExp(u32),
    BlockSeq(u32),
    BlockMap(u32),
    BlockKeyExp(u32),
    BlockValExp(u32),
    AfterDocEnd,
}

impl ParserState {
    #[inline]
    pub(crate) fn indent(&self, default: usize) -> u32 {
        match self {
            FlowKey(ind) | FlowKeyExp(ind) | FlowMap(ind) | FlowSeq(ind) | BlockSeq(ind)
            | BlockMap(ind) | BlockKeyExp(ind) | BlockValExp(ind) => *ind,
            RootBlock => default as u32,
            PreDocStart | AfterDocEnd => 0,
        }
    }

    #[inline]
    pub(crate) fn get_block_indent(&self, default: usize) -> usize {
        match self {
            BlockKeyExp(ind) | BlockValExp(ind) => *ind as usize,
            _ => default,
        }
    }

    #[inline]
    pub(crate) fn wrong_exp_indent(&self, curr_indent: usize) -> bool {
        match self {
            BlockKeyExp(ind) | BlockValExp(ind) => *ind as usize != curr_indent,
            _ => false,
        }
    }

    #[inline]
    pub fn in_flow_collection(&self) -> bool {
        match &self {
            FlowKey(_) | FlowKeyExp(_) | FlowSeq(_) | FlowMap(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub(crate) fn is_implicit(&self) -> bool {
        match &self {
            FlowKeyExp(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub(crate) fn is_new_block_col(&self, curr_indent: usize) -> bool {
        match &self {
            FlowKey(_) | FlowKeyExp(_) | FlowMap(_) | FlowSeq(_) => false,
            BlockMap(x) | BlockKeyExp(x) if *x as usize == curr_indent => false,
            _ => true,
        }
    }
}

impl Spanner {
    #[inline(always)]
    pub fn pop_token(&mut self) -> Option<SpanToken> {
        self.tokens.pop_front()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn fetch_next_token<B, R: Reader<B>>(&mut self, reader: &mut R) {
        reader.skip_separation_spaces(true);
        match self.curr_state {
            PreDocStart => {
                if reader.peek_byte_is(b'%') {
                    reader.try_read_yaml_directive(&mut self.tokens);
                    if reader.try_read_slice_exact("---") {
                        self.tokens.push_back(DocumentStart)
                    } else {
                        self.tokens.push_back(ErrorToken(NoDocStartAfterTag))
                    }
                } else if reader.try_read_slice_exact("---") {}
                self.curr_state = RootBlock;
                return;
            }
            RootBlock | BlockMap(_) | BlockKeyExp(_) | BlockValExp(_) | BlockSeq(_) => {
                let indent = self.curr_state.indent(reader.col());
                let init_indent = match self.curr_state {
                    BlockKeyExp(ind) | BlockValExp(ind) => ind,
                    BlockMap(_) => reader.col() as u32,
                    _ => indent,
                };
                match reader.peek_byte() {
                    Some(b'{') => self.fetch_flow_col(reader, indent as usize),
                    Some(b'[') => self.fetch_flow_col(reader, indent as usize),
                    Some(b'&') => reader.consume_anchor_alias(&mut self.tokens, Anchor),
                    Some(b'*') => reader.consume_anchor_alias(&mut self.tokens, Alias),
                    Some(b':') => {
                        reader.consume_bytes(1);
                        self.tokens.push_back(KeyEnd);
                        if let BlockKeyExp(x) = self.curr_state {
                            self.curr_state = BlockMap(x);
                        }
                    }
                    Some(b'-') => self.fetch_block_seq(reader, indent as usize),
                    Some(b'?') => self.fetch_block_map_key(reader, indent as usize),
                    Some(b'!') => self.fetch_tag(reader),
                    Some(b'|') => {
                        reader.read_block_scalar(true, &self.curr_state, &mut self.tokens)
                    }
                    Some(b'>') => {
                        reader.read_block_scalar(false, &self.curr_state, &mut self.tokens)
                    }
                    Some(b'\'') => reader.read_single_quote(false, &mut self.tokens),
                    Some(b'"') => reader.read_double_quote(false, &mut self.tokens),
                    Some(b'#') => {
                        // comment
                        reader.read_line();
                    }
                    Some(x) => {
                        if x != b']' && x != b'}' && x != b'@' {
                            self.fetch_plain_scalar(reader, indent as usize, init_indent as usize);
                        } else {
                            reader.consume_bytes(1);
                            self.tokens
                                .push_back(ErrorToken(UnexpectedSymbol(x as char)))
                        }
                    }
                    None => self.stream_end = true,
                }
            }
            FlowSeq(indent) => match reader.peek_byte() {
                Some(b'&') => reader.consume_anchor_alias(&mut self.tokens, Anchor),
                Some(b'*') => reader.consume_anchor_alias(&mut self.tokens, Alias),
                Some(b'[') => self.fetch_flow_col(reader, (indent + 1) as usize),
                Some(b'{') => self.fetch_flow_col(reader, (indent + 1) as usize),
                Some(b']') => {
                    reader.consume_bytes(1);
                    self.tokens.push_back(SequenceEnd);
                    self.pop_state();
                }
                Some(b'}') => {
                    reader.consume_bytes(1);
                    self.tokens.push_back(ErrorToken(UnexpectedSymbol('}')));
                }
                Some(b',') => {
                    reader.consume_bytes(1);
                    self.tokens.push_back(Separator);
                }
                Some(b'\'') => {
                    reader.read_single_quote(self.curr_state.is_implicit(), &mut self.tokens)
                }
                Some(b'"') => {
                    reader.read_double_quote(self.curr_state.is_implicit(), &mut self.tokens)
                }
                Some(b':') => {
                    reader.consume_bytes(1);
                    self.tokens.push_back(MappingStart);
                    self.push_state(FlowKeyExp(indent));
                }
                Some(b'?') => self.fetch_explicit_map(reader),
                Some(b'#') => {
                    // comment
                    reader.read_line();
                }
                Some(_) => {
                    self.fetch_plain_scalar(reader, indent as usize, reader.col());
                }
                None => self.stream_end = true,
            },
            FlowMap(indent) | FlowKey(indent) | FlowKeyExp(indent) => match reader.peek_byte() {
                Some(b'&') => reader.consume_anchor_alias(&mut self.tokens, Anchor),
                Some(b'*') => reader.consume_anchor_alias(&mut self.tokens, Alias),
                Some(b'[') => self.fetch_flow_col(reader, (indent + 1) as usize),
                Some(b'{') => self.fetch_flow_col(reader, (indent + 1) as usize),
                Some(b'}') => {
                    reader.consume_bytes(1);
                    self.tokens.push_back(MappingEnd);
                    self.pop_state();
                }
                Some(b':') => self.process_map_key(reader, indent as usize),
                Some(b']') => {
                    if self.is_prev_sequence() {
                        self.tokens.push_back(MappingEnd);
                        self.pop_state();
                    } else {
                        reader.consume_bytes(1);
                        self.tokens.push_back(ErrorToken(UnexpectedSymbol(']')));
                    }
                }
                Some(b'?') => self.fetch_explicit_map(reader),
                Some(b',') => {
                    reader.consume_bytes(1);
                }
                Some(b'\'') => {
                    reader.read_single_quote(self.curr_state.is_implicit(), &mut self.tokens)
                }
                Some(b'"') => {
                    reader.read_double_quote(self.curr_state.is_implicit(), &mut self.tokens)
                }
                Some(b'#') => {
                    // comment
                    reader.read_line();
                }
                Some(_) => {
                    self.fetch_plain_scalar(reader, indent as usize, reader.col());
                }
                None => self.stream_end = true,
            },
            _ => {}
        }

        if reader.eof() {
            self.stream_end = true;
            self.stack.push(self.curr_state);
            for state in self.stack.iter().rev() {
                let x = match *state {
                    BlockSeq(_) => SequenceEnd,
                    BlockMap(_) | BlockKeyExp(_) => MappingEnd,
                    _ => continue,
                };
                self.tokens.push_back(x);
            }
        }
    }

    fn fetch_flow_col<B, R: Reader<B>>(&mut self, reader: &mut R, indent: usize) {
        let peek = reader.peek_byte().unwrap_or(b'\0');
        reader.consume_bytes(1);

        if reader.col() != 0 {
            reader.skip_space_tab(true);
        }

        if peek == b'[' {
            self.tokens.push_back(SequenceStart);
            self.push_state(FlowSeq(indent as u32));
        } else if peek == b'{' {
            if reader.col() != 0 {
                reader.skip_space_tab(true);
            }
            if reader.peek_byte_is(b'?') {
                self.push_state(FlowKey(indent as u32));
            } else {
                self.push_state(FlowKeyExp(indent as u32 ));
            }
            self.tokens.push_back(MappingStart);
        }
    }

    #[inline]
    fn push_state(&mut self, state: ParserState) {
        self.stack.push(self.curr_state);
        self.curr_state = state;
    }

    #[inline]
    fn pop_state(&mut self) {
        match self.stack.pop() {
            Some(x) => self.curr_state = x,
            None => self.curr_state = AfterDocEnd,
        }
    }

    fn fetch_block_seq<B, R: Reader<B>>(&mut self, reader: &mut R, indent: usize) {
        if let Some(new_state) = reader.read_block_seq(indent) {
            self.tokens.push_back(SequenceStart);
            self.push_state(new_state);
        } else {
            self.fetch_plain_scalar(reader, indent, indent);
        }
    }

    fn fetch_block_map_key<B, R: Reader<B>>(&mut self, reader: &mut R, indent: usize) {
        reader.consume_bytes(1);
        self.push_state(BlockKeyExp(indent as u32));
        self.tokens.push_back(MappingStart);
    }

    fn fetch_tag<B, R: Reader<B>>(&mut self, reader: &mut R) {
        let start = reader.consume_bytes(1);
        if let Some((mid, end)) = reader.read_tag() {
            self.tokens.push_back(TagStart(start));
            self.tokens.push_back(MarkStart(mid));
            self.tokens.push_back(MarkEnd(end));
            reader.consume_bytes(end - start);
        }
    }

    fn fetch_plain_scalar<B, R: Reader<B>>(
        &mut self,
        reader: &mut R,
        start_indent: usize,
        init_indent: usize,
    ) {
        let (tokens, new_state) =
            reader.read_plain_scalar(start_indent, init_indent, &self.curr_state);

        match new_state {
            Some(BlockValExp(x)) => self.curr_state = BlockValExp(x),
            Some(state) => self.push_state(state),
            None => {}
        }
        self.tokens.extend(tokens);
    }

    fn fetch_explicit_map<B, R: Reader<B>>(&mut self, reader: &mut R) {
        if !self.is_map() {
            self.tokens.push_back(MappingStart);
        }

        if !reader.peek_byte_at_check(1, is_white_tab_or_break) {
            self.fetch_plain_scalar(reader, reader.col(), reader.col());
        } else {
            reader.consume_bytes(1);
            reader.skip_space_tab(true);
        }
    }

    fn process_map_key<B, R: Reader<B>>(&mut self, reader: &mut R, indent: usize) {
        reader.consume_bytes(1);

        if self.is_key() {
            self.curr_state = FlowMap(indent as u32);
            self.tokens.push_back(KeyEnd);
        } else {
            self.fetch_plain_scalar(reader, indent, indent);
        }
    }

    #[inline]
    fn is_prev_sequence(&self) -> bool {
        match self.stack.last() {
            Some(FlowSeq(_)) => true,
            _ => false,
        }
    }

    #[inline]
    fn is_map(&self) -> bool {
        match self.curr_state {
            FlowMap(_) | FlowKey(_) | FlowKeyExp(_) => true,
            _ => false,
        }
    }

    #[inline]
    fn is_key(&self) -> bool {
        match self.curr_state {
            FlowKey(_) | FlowKeyExp(_) => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone)]
pub enum SpanToken {
    ErrorToken(ErrorType),
    MarkStart(usize),
    MarkEnd(usize),
    NewLine(u32),
    Space,
    Directive(DirectiveType),
    /// Element with alternative name e.g. `&foo [x,y]`
    Alias,
    /// Reference to an element with alternative name e.g. `*foo`
    Anchor,
    Separator,
    /// YAML tag start followed by MarkStart, MarkEnd
    /// It decomposes like this:
    /// ```text
    ///   !schema!tag
    ///    ^     ^   ^
    ///    |     |   |
    ///    |     |   +- MarkEnd
    ///    |     +- MarkStart
    ///    +- TagStart
    /// ```
    TagStart(usize),
    KeyEnd,
    SequenceStart,
    SequenceEnd,
    MappingStart,
    MappingEnd,
    DocumentStart,
    DocumentEnd,
}
