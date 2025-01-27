#![allow(clippy::match_like_matches_macro)]

use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
use std::hint::unreachable_unchecked;
use std::mem::take;

use LexerState::PreDocStart;
use SeqState::BeforeFirstElem;

use crate::tokenizer::lexer::LexerState::{
    AfterDocBlock, BlockMap, BlockMapExp, BlockSeq, DirectiveSection, DocBlock, EndOfDirective,
    FlowKeyExp, FlowMap, FlowSeq, InDocEnd,
};
use crate::tokenizer::lexer::LexerToken::*;
use crate::tokenizer::lexer::MapState::{AfterColon, BeforeColon, BeforeKey};
use crate::tokenizer::lexer::SeqState::{BeforeElem, InSeq};
use crate::tokenizer::reader::{is_white_tab_or_break, Reader};
use crate::tokenizer::ErrorType::*;

use super::iterator::{DirectiveType, ScalarType};
use super::reader::{is_flow_indicator, is_newline, is_not_whitespace, ns_plain_safe};
use crate::tokenizer::ErrorType;

#[derive(Clone, Default)]
pub struct Lexer {
    pub stream_end: bool,
    pub(crate) tokens: VecDeque<usize>,
    pub(crate) errors: Vec<ErrorType>,
    pub(crate) tags: HashMap<Vec<u8>, (usize, usize)>,
    stack: Vec<LexerState>,
    last_block_indent: usize,
    had_anchor: bool,
    has_tab: bool,
    prev_anchor: Option<(usize, usize)>,
    continue_processing: bool,
    prev_scalar: Scalar,
    last_map_line: usize,
    col_start: Option<usize>,
}

#[derive(Clone, Default)]
pub(crate) struct Scalar {
    scalar_start: usize,
    is_multiline: bool,
    tokens: Vec<usize>,
}

impl Scalar {
    pub fn is_empty(&self) -> bool {
        self.tokens.len() == 0
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum MapState {
    #[default]
    BeforeKey,
    BeforeColon,
    AfterColon,
}

impl MapState {
    pub fn next_state(&self) -> MapState {
        match self {
            BeforeKey => BeforeColon,
            BeforeColon => AfterColon,
            AfterColon => BeforeKey,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum SeqState {
    BeforeFirstElem,
    #[default]
    BeforeElem,
    InSeq,
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum LexerState {
    #[default]
    PreDocStart,
    DirectiveSection,
    EndOfDirective,
    AfterDocBlock,
    InDocEnd,
    // Flow nodes
    // u32 is the index of the token insertion point for flow nodes
    FlowSeq(u32, SeqState),
    FlowMap(u32, MapState),
    FlowKeyExp(u32, MapState),
    // Blocks nodes
    // u32 is the indent of block node
    DocBlock,
    BlockSeq(u32),
    BlockMap(u32, MapState),
    BlockMapExp(u32, MapState),
}

impl LexerState {
    #[inline]
    pub fn in_flow_collection(&self) -> bool {
        match &self {
            FlowKeyExp(_, _) | FlowSeq(_, _) | FlowMap(_, _) => true,
            _ => false,
        }
    }

    #[inline]
    pub(crate) fn is_implicit(&self) -> bool {
        match &self {
            FlowKeyExp(_, _) => true,
            _ => false,
        }
    }

    fn get_map(&self, scalar_start: usize) -> LexerState {
        match *self {
            BlockSeq(_) | BlockMap(_, _) | BlockMapExp(_, _) | DocBlock => {
                BlockMap(scalar_start as u32, BeforeColon)
            }
            _ => panic!("Unexpected state {:?}", self),
        }
    }

    pub(crate) fn is_map_start(&self, scalar_start: usize) -> bool {
        match self {
            DocBlock => true,
            BlockSeq(ind) | BlockMap(ind, _) if scalar_start > *ind as usize => true,
            _ => false,
        }
    }

    fn is_incorrectly_indented(&self, scalar_start: usize) -> bool {
        match &self {
            BlockMapExp(ind, _) => scalar_start < *ind as usize,
            BlockMap(ind, _) | BlockSeq(ind) => scalar_start <= *ind as usize,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DirectiveState {
    NoContent,
    Tag,
    Directive,
    DirectiveAndTag,
}

impl DirectiveState {
    fn add_tag(&mut self) {
        *self = match self {
            Self::NoContent => Self::Tag,
            Self::Directive => Self::DirectiveAndTag,
            _ => *self,
        }
    }

    fn add_directive(&mut self) {
        *self = match self {
            Self::NoContent => Self::Directive,
            Self::Tag => Self::DirectiveAndTag,
            _ => *self,
        }
    }
}

impl Lexer {
    pub fn fetch_next_token<B, R: Reader<B>>(&mut self, reader: &mut R) {
        self.continue_processing = true;
        let mut directive_state = DirectiveState::NoContent;

        while self.continue_processing && !reader.eof() {
            let curr_state = self.curr_state();
            if self.skip_separation_spaces(reader, true).1 && !self.has_tab {
                self.has_tab = true;
            }

            match curr_state {
                PreDocStart => self.fetch_pre_doc(reader),
                DirectiveSection => self.fetch_directive_section(reader, &mut directive_state),
                EndOfDirective => self.fetch_end_of_directive(reader, &mut directive_state),
                DocBlock | BlockMap(_, _) | BlockMapExp(_, _) => {
                    self.fetch_block_map(reader, curr_state)
                }
                BlockSeq(_) => self.fetch_block_seq(reader, curr_state),
                FlowSeq(_, seq_state) => self.fetch_flow_seq(reader, seq_state),
                FlowMap(_, _) | FlowKeyExp(_, _) => self.fetch_flow_map(reader, curr_state),
                AfterDocBlock => self.fetch_after_doc(reader),
                InDocEnd => self.fetch_end_doc(reader),
            }
        }

        if reader.eof() {
            self.stream_end = true;
            self.finish_eof();
        }
    }

    fn finish_eof(&mut self) {
        for state in self.stack.iter().rev() {
            let token = match *state {
                BlockSeq(_) => SEQ_END,
                BlockMapExp(_, AfterColon | BeforeColon) | BlockMap(_, AfterColon) => {
                    self.tokens.push_back(SCALAR_PLAIN);
                    self.tokens.push_back(SCALAR_END);
                    MAP_END
                }
                BlockMapExp(_, _) | BlockMap(_, _) | FlowMap(_, _) => MAP_END,
                DirectiveSection => {
                    self.errors.push(ErrorType::DirectiveEndMark);
                    ERROR_TOKEN
                }
                EndOfDirective => {
                    self.tokens.push_back(SCALAR_PLAIN);
                    self.tokens.push_back(SCALAR_END);
                    DOC_END
                }
                DocBlock | AfterDocBlock => DOC_END,
                _ => continue,
            };
            self.tokens.push_back(token);
        }
    }

    fn fetch_pre_doc<B, R: Reader<B>>(&mut self, reader: &mut R) {
        match reader.peek_chars() {
            [b'%', ..] => {
                self.set_curr_state(DirectiveSection, 0);
            }
            b"---" => {
                reader.consume_bytes(3);
                self.tokens.push_back(DOC_START_EXP);
                self.set_curr_state(EndOfDirective, 0);
            }
            b"..." => {
                reader.consume_bytes(3);
                reader.skip_separation_spaces(true);
                self.set_curr_state(InDocEnd, 0);
            }
            [b'#', ..] => {
                reader.read_line();
            }
            [_, ..] => {
                self.tokens.push_back(DOC_START);
                self.set_curr_state(DocBlock, 0);
            }
            [] => {}
        }
    }

    fn fetch_directive_section<B, R: Reader<B>>(
        &mut self,
        reader: &mut R,
        directive_state: &mut DirectiveState,
    ) {
        match reader.peek_chars() {
            [b'%', b'Y', ..] => {
                if matches!(
                    directive_state,
                    DirectiveState::NoContent | DirectiveState::Tag
                ) {
                    if self.try_read_yaml_directive(reader) {
                        directive_state.add_directive();
                    } else {
                        directive_state.add_tag();
                    }
                } else {
                    self.push_error(ErrorType::TwoDirectivesFound);
                    reader.read_line();
                    self.continue_processing = false;
                }
            }
            [b'#', ..] => {
                reader.read_line();
            }
            [b'%', ..] => self.fetch_read_tag(reader, directive_state),
            b"..." => {
                reader.consume_bytes(3);
                self.tokens.push_back(DOC_START);
                self.tokens.push_back(DOC_END_EXP);
                self.prepend_error(ErrorType::UnexpectedEndOfStream);
                self.set_curr_state(PreDocStart, 0);
                self.continue_processing = false;
            }
            b"---" => {
                reader.consume_bytes(3);
                self.tokens.push_back(DOC_START_EXP);
                self.set_curr_state(EndOfDirective, 0);
                self.continue_processing = true;
            }
            [x, ..] if !is_white_tab_or_break(*x) => {
                self.prepend_error(ErrorType::YamlMustHaveOnePart);
                reader.read_line();
            }
            _ => {
                self.continue_processing = false;
            }
        }
    }

    fn try_read_yaml_directive<B, R: Reader<B>>(&mut self, reader: &mut R) -> bool {
        if reader.try_read_slice_exact("%YAML") {
            reader.skip_space_tab();
            return match reader.peek_chars() {
                b"1.0" | b"1.1" | b"1.2" | b"1.3" => {
                    self.tokens.push_back(DIR_YAML);
                    self.tokens.push_back(reader.pos());
                    self.tokens.push_back(reader.consume_bytes(3));
                    let has_ws_break = reader.peek_byte().map_or(false, is_white_tab_or_break);
                    if !has_ws_break {
                        self.prepend_error(ErrorType::UnsupportedYamlVersion);
                        reader.read_line();
                    }
                    has_ws_break
                }
                b"..." | b"---" => false,
                _ => {
                    reader.read_line();
                    false
                }
            };
        } else {
            reader.read_line();
            false
        }
    }

    fn fetch_read_tag<B, R: Reader<B>>(
        &mut self,
        reader: &mut R,
        directive_state: &mut DirectiveState,
    ) {
        self.continue_processing = false;
        // TODO actual tag handling
        directive_state.add_tag();
        reader.try_read_slice_exact("%TAG");
        reader.read_line();
    }

    fn fetch_end_of_directive<B, R: Reader<B>>(
        &mut self,
        reader: &mut R,
        _directive_state: &mut DirectiveState,
    ) {
        let col = reader.col();
        self.continue_processing = false;
        match reader.peek_chars() {
            b"..." => {
                reader.consume_bytes(3);
                if col != 0 {
                    self.push_error(ErrorType::UnxpectedIndentDocEnd {
                        actual: col,
                        expected: 0,
                    });
                }
                self.push_empty_token();
                self.tokens.push_back(DOC_END_EXP);
                self.set_curr_state(InDocEnd, 0);
            }
            [b'%', ..] => {
                self.prepend_error(ErrorType::ExpectedDocumentEndOrContents);
                self.tokens.push_back(DOC_END);
                self.set_curr_state(DirectiveSection, 0);
            }
            [x, ..] if is_not_whitespace(*x) => {
                self.set_curr_state(DocBlock, reader.line());
                self.continue_processing = true;
            }
            [..] => {}
        };
    }

    fn fetch_after_doc<B, R: Reader<B>>(&mut self, reader: &mut R) {
        let mut consume_line = false;
        match reader.peek_chars() {
            b"..." => {
                let col = reader.col();
                reader.consume_bytes(3);
                if col != 0 {
                    self.push_error(ErrorType::UnxpectedIndentDocEnd {
                        actual: col,
                        expected: 0,
                    });
                }
                self.tokens.push_back(DOC_END_EXP);
                self.set_curr_state(InDocEnd, 0);
            }
            [b'#', ..] => {
                consume_line = true;
                self.set_curr_state(PreDocStart, 0);
            }
            [chr, ..] => {
                consume_line = true;
                self.tokens.push_back(DOC_END);
                self.push_error(UnexpectedSymbol(*chr as char));
                self.set_curr_state(PreDocStart, 0);
            }
            [] => {}
        }
        if consume_line {
            reader.read_line();
        }
    }

    fn fetch_end_doc<B, R: Reader<B>>(&mut self, reader: &mut R) {
        reader.skip_space_tab();
        if let Some(chr) = reader.peek_byte() {
            if is_not_whitespace(chr) {
                self.push_error(ErrorType::ExpectedDocumentStartOrContents);
            }
            reader.read_line();
        }
        if reader.col() == 0 {
            self.set_curr_state(PreDocStart, 0);
        }
    }

    fn fetch_block_seq<B, R: Reader<B>>(&mut self, reader: &mut R, curr_state: LexerState) {
        self.continue_processing = false;
        match reader.peek_chars() {
            [b'{', ..] => self.process_flow_map_start(reader),
            [b'[', ..] => self.process_flow_seq_start(reader),
            [b'&', ..] => self.parse_anchor(reader),
            [b'*', ..] => self.parse_alias(reader),
            [b'-', x, ..] if is_white_tab_or_break(*x) => {
                self.process_block_seq(reader, curr_state);
            }
            b"---" => self.unwind_to_root_start(reader),
            b"..." => self.unwind_to_root_end(reader),
            [b'?', x, ..] if is_white_tab_or_break(*x) => {
                self.fetch_exp_block_map_key(reader, curr_state)
            }
            [b'!', ..] => self.fetch_tag(reader),
            [b'|', ..] => self.process_block_literal(reader, curr_state),
            [b'>', ..] => self.process_block_folded(reader, curr_state),
            [b'\'', ..] => self.process_single_quote_block(reader, curr_state),
            [b'"', ..] => self.process_double_quote_block(reader, curr_state),
            [b'#', ..] => {
                // comment
                reader.read_line();
            }
            [peek_chr, ..] => self.fetch_plain_scalar_block(reader, curr_state, *peek_chr),
            [] => self.stream_end = true,
        }
    }

    fn fetch_block_map<B, R: Reader<B>>(&mut self, reader: &mut R, curr_state: LexerState) {
        self.continue_processing = false;
        match reader.peek_chars() {
            [b'{', ..] => self.process_flow_map_start(reader),
            [b'[', ..] => self.process_flow_seq_start(reader),
            [b'&', ..] => self.parse_anchor(reader),
            [b'*', ..] => self.parse_alias(reader),
            [b':'] => self.process_colon_block(reader, curr_state),
            [b':', peek, ..] if !ns_plain_safe(*peek) => {
                self.process_colon_block(reader, curr_state)
            }
            [b'-', peek, ..] if !ns_plain_safe(*peek) => {
                self.process_block_seq(reader, curr_state);
            }
            b"..." => {
                self.unwind_to_root_end(reader);
            }
            b"---" => {
                self.unwind_to_root_start(reader);
            }
            [b'?', peek, ..] if !ns_plain_safe(*peek) => {
                self.fetch_exp_block_map_key(reader, curr_state)
            }
            [b'!', ..] => self.fetch_tag(reader),
            [b'|', ..] => {
                self.next_map_state();
                self.process_block_literal(reader, curr_state);
            }
            [b'>', ..] => {
                self.next_map_state();
                self.process_block_folded(reader, curr_state);
            }
            [b'\'', ..] => {
                self.process_single_quote_block(reader, curr_state);
            }
            [b'"', ..] => {
                self.process_double_quote_block(reader, curr_state);
            }
            [b'#', ..] => {
                // comment
                reader.read_line();
            }
            [peek, ..] => {
                self.fetch_plain_scalar_block(reader, curr_state, *peek);
            }
            _ => self.stream_end = true,
        }
    }

    fn process_block_literal<B, R: Reader<B>>(&mut self, reader: &mut R, curr_state: LexerState) {
        let had_tab = self.has_tab;
        let scalar_line = reader.line();
        let scalar_start = reader.col();

        let tokens = reader.read_block_scalar(
            true,
            &self.curr_state(),
            self.last_block_indent,
            &mut self.errors,
        );
        let is_multiline = reader.line() != scalar_line;
        reader.skip_space_tab();

        let is_key = reader.peek_byte().map_or(false, |chr| chr == b':');

        self.process_block_scalar(
            curr_state,
            is_key,
            scalar_start,
            had_tab,
            is_multiline,
            tokens,
            scalar_line,
        );
    }

    fn process_block_folded<B, R: Reader<B>>(&mut self, reader: &mut R, curr_state: LexerState) {
        let had_tab = self.has_tab;
        let scalar_line = reader.line();
        let scalar_start = reader.col();

        let tokens = reader.read_block_scalar(
            false,
            &self.curr_state(),
            self.last_block_indent,
            &mut self.errors,
        );
        let is_multiline = reader.line() != scalar_line;
        reader.skip_space_tab();

        let is_key = reader.peek_byte().map_or(false, |chr| chr == b':');

        self.process_block_scalar(
            curr_state,
            is_key,
            scalar_start,
            had_tab,
            is_multiline,
            tokens,
            scalar_line,
        );
    }

    // #[inline(always)]
    fn push_error(&mut self, error: ErrorType) {
        self.tokens.push_back(ERROR_TOKEN);
        self.errors.push(error);
    }

    // #[inline(always)]
    fn prepend_error(&mut self, error: ErrorType) {
        self.tokens.push_front(ERROR_TOKEN);
        self.errors.push(error);
    }

    fn parse_anchor<B, R: Reader<B>>(&mut self, reader: &mut R) {
        self.update_col(reader);
        let anchor = reader.consume_anchor_alias();

        let line = self.skip_separation_spaces(reader, true);
        match line.0 {
            0 => {
                self.prev_anchor = Some(anchor);
            }
            _ => {
                self.tokens.push_back(ANCHOR);
                self.tokens.push_back(anchor.0);
                self.tokens.push_back(anchor.1);
                self.had_anchor = true;
            }
        }
    }

    fn parse_alias<B, R: Reader<B>>(&mut self, reader: &mut R) {
        let alias_start: usize = reader.col();
        let had_tab = self.has_tab;
        let alias = reader.consume_anchor_alias();
        self.skip_separation_spaces(reader, true);

        let next_is_colon = reader.peek_byte_is(b':');

        self.next_map_state();
        if next_is_colon {
            self.process_block_scalar(
                self.curr_state(),
                true,
                alias_start,
                had_tab,
                false,
                vec![ALIAS, alias.0, alias.1],
                reader.line(),
            );
        } else {
            self.tokens.push_back(ALIAS);
            self.tokens.push_back(alias.0);
            self.tokens.push_back(alias.1);
        }
    }

    fn process_post_seq<B, R: Reader<B>>(&mut self, reader: &mut R, index: u32, in_flow: bool) {
        // could be `[a]: b` map
        if reader.peek_byte_is(b':') {
            if !self.is_flow_map() {
                let token = if in_flow { MAP_START } else { MAP_START_BLOCK };
                self.tokens.insert(index as usize, token);
                let state = FlowMap(self.get_token_pos(), AfterColon);
                self.push_state(state, reader.line());
                self.continue_processing = true;
            }
            reader.consume_bytes(1);
        }
    }

    fn fetch_flow_seq<B, R: Reader<B>>(&mut self, reader: &mut R, seq_state: SeqState) {
        match reader.peek_chars() {
            [b'&', ..] => self.parse_anchor(reader),
            [b'*', ..] => self.parse_alias(reader),
            [b'[', ..] => self.process_flow_seq_start(reader),
            [b'{', ..] => self.process_flow_map_start(reader),
            [b']', ..] => {
                reader.consume_bytes(1);
                self.tokens.push_back(SEQ_END);
                let index = self.pop_state().map_or(0, |f| match f {
                    FlowSeq(x, _) => x.saturating_sub(1),
                    _ => 0,
                });
                self.process_post_seq(reader, index, self.curr_state().in_flow_collection());
            }
            [b'-', ..] if seq_state == BeforeFirstElem => {
                reader.consume_bytes(1);
                self.push_error(UnexpectedSymbol('-'));
            }
            [b':', chr, ..] if seq_state != InSeq && !ns_plain_safe(*chr) => {
                self.tokens.push_back(MAP_START);
                let indent = self.get_token_pos();
                self.push_empty_token();
                self.set_curr_state(FlowSeq(indent, InSeq), reader.line());
                let indent = self.get_token_pos();
                let state = FlowMap(indent, AfterColon);
                self.push_state(state, reader.line());
            }
            [b'}', ..] => {
                reader.consume_bytes(1);
                self.push_error(UnexpectedSymbol('}'));
            }
            [b',', ..] => {
                reader.consume_bytes(1);
                self.set_seq_state(BeforeElem);
            }
            [b'\'', ..] => self.process_single_quote_flow(reader, self.curr_state()),
            [b'"', ..] => self.process_double_quote_flow(reader),
            [b'?', chr, ..] if !ns_plain_safe(*chr) => {
                self.fetch_explicit_map(reader, self.curr_state())
            }
            [b'#', ..] => {
                // comment
                reader.read_line();
            }
            [_, ..] => {
                self.fetch_plain_scalar_flow(reader, self.curr_state());
            }
            [] => self.stream_end = true,
        }
    }

    fn fetch_flow_map<B, R: Reader<B>>(&mut self, reader: &mut R, curr_state: LexerState) {
        match reader.peek_chars() {
            [b'&', ..] => self.parse_anchor(reader),
            [b'*', ..] => self.parse_alias(reader),
            [b'[', ..] => {
                self.next_map_state();
                self.process_flow_seq_start(reader);
            }
            [b'{', ..] => {
                self.next_map_state();
                self.process_flow_map_start(reader);
            }
            [b'}', ..] => {
                reader.consume_bytes(1);
                if matches!(self.curr_state(), FlowMap(_, BeforeColon)) {
                    self.push_empty_token();
                }
                self.tokens.push_back(MAP_END);
                self.pop_state();
                self.continue_processing = false;
            }
            [b':', peek, ..] if !ns_plain_safe(*peek) => {
                reader.consume_bytes(1);
                self.process_colon_flow(curr_state);
            }
            [b':', peek, ..]
                if ns_plain_safe(*peek) && matches!(curr_state, FlowMap(_, BeforeColon)) =>
            {
                reader.consume_bytes(1);
                self.process_colon_flow(curr_state);
            }
            [b']', ..] => {
                if self.is_prev_sequence() {
                    if self.is_unfinished() {
                        self.push_empty_token();
                    }
                    self.tokens.push_back(MAP_END);
                    self.pop_state();
                } else {
                    reader.consume_bytes(1);
                    self.push_error(UnexpectedSymbol(']'));
                }
            }
            [b'?', peek, ..] if !ns_plain_safe(*peek) => {
                self.fetch_explicit_map(reader, curr_state)
            }
            [b',', ..] => {
                reader.consume_bytes(1);
                if !self.prev_scalar.is_empty() {
                    self.emit_prev_anchor();
                    self.tokens.extend(take(&mut self.prev_scalar.tokens));
                    self.set_map_state(BeforeKey);
                } else if self.is_prev_sequence() {
                    self.tokens.push_back(MAP_END);
                    self.pop_state();
                } else if matches!(curr_state, FlowMap(_, AfterColon) | FlowMap(_, BeforeColon)) {
                    self.push_empty_token();
                    self.set_map_state(BeforeKey);
                }
            }
            [b'\'', ..] => {
                self.next_map_state();
                self.process_single_quote_flow(reader, curr_state);
            }
            [b'"', ..] => {
                self.next_map_state();
                self.process_double_quote_flow(reader);
            }
            [b'#', ..] => {
                // comment
                reader.read_line();
            }
            [_, ..] => {
                self.next_map_state();
                self.fetch_plain_scalar_flow(reader, curr_state);
            }
            [] => self.stream_end = true,
        }
    }

    fn process_colon_flow(&mut self, curr_state: LexerState) {
        if !self.prev_scalar.is_empty() {
            self.emit_prev_anchor();
            self.tokens.extend(take(&mut self.prev_scalar.tokens));
            self.set_map_state(AfterColon);
        } else if matches!(curr_state, FlowMap(_, BeforeKey)) {
            self.push_empty_token();
            self.next_map_state();
        } else if matches!(curr_state, FlowKeyExp(_, _)) {
            self.next_map_state();
            self.tokens.push_back(SCALAR_END);
        }
    }

    fn unwind_map(&mut self, curr_state: LexerState, scalar_start: usize, reader_line: usize) {
        if let Some(unwind) = self.find_matching_state(
            scalar_start,
            |state, indent| matches!(state, BlockMap(ind, _) | BlockMapExp(ind, _) if ind as usize == indent),
        ) {
            self.pop_block_states(unwind);
        } else {
            self.tokens.push_back(MAP_START_BLOCK);
            self.push_state(curr_state.get_map(scalar_start), reader_line);
        }
    }

    fn process_single_quote_flow<B, R: Reader<B>>(
        &mut self,
        reader: &mut R,
        curr_state: LexerState,
    ) {
        let scalar_start = self.update_col(reader);
        let tokens = reader.read_single_quote(curr_state.is_implicit());

        self.skip_separation_spaces(reader, true);
        if reader.peek_byte_is(b':') {
            self.unwind_map(curr_state, scalar_start, reader.line());
            self.set_map_state(BeforeColon);
        }
        self.emit_prev_anchor();
        self.tokens.extend(tokens);
    }

    #[inline]
    fn process_double_quote<B, R: Reader<B>>(&mut self, reader: &mut R) -> Scalar {
        let scalar_start = self.update_col(reader);
        let start_line = reader.line();
        let tokens = reader.read_double_quote(&mut self.errors);
        let is_multiline = start_line != reader.line();
        Scalar {
            scalar_start,
            is_multiline,
            tokens,
        }
    }

    #[inline]
    fn process_single_quote<B, R: Reader<B>>(&mut self, reader: &mut R) -> Scalar {
        let scalar_start = self.update_col(reader);
        let start_line = reader.line();
        let tokens = reader.read_single_quote(self.curr_state().is_implicit());
        let is_multiline = start_line != reader.line();
        Scalar {
            scalar_start,
            is_multiline,
            tokens,
        }
    }

    fn process_double_quote_flow<B, R: Reader<B>>(&mut self, reader: &mut R) {
        let scalar = self.process_double_quote(reader);
        reader.skip_space_tab();

        if reader.peek_byte().map_or(false, |c| c == b':') {
            self.prev_scalar = scalar;
            self.continue_processing = true;
        } else {
            self.emit_prev_anchor();
            self.tokens.extend(scalar.tokens);
        }
    }

    fn process_double_quote_block<B, R: Reader<B>>(
        &mut self,
        reader: &mut R,
        curr_state: LexerState,
    ) {
        let had_tab = self.has_tab;
        let scalar_line = reader.line();
        let Scalar {
            scalar_start,
            is_multiline,
            tokens,
        } = self.process_double_quote(reader);
        reader.skip_space_tab();

        let is_key = reader.peek_byte().map_or(false, |chr| chr == b':');

        self.process_block_scalar(
            curr_state,
            is_key,
            scalar_start,
            had_tab,
            is_multiline,
            tokens,
            scalar_line,
        );
    }

    fn process_block_scalar(
        &mut self,
        curr_state: LexerState,
        is_key: bool,
        scalar_start: usize,
        had_tab: bool,
        is_multiline: bool,
        tokens: Vec<usize>,
        scalar_line: usize,
    ) {
        if is_key {
            let is_map_start = curr_state.is_map_start(scalar_start);
            let scalar_start = scalar_start;
            self.prev_scalar = Scalar {
                scalar_start,
                is_multiline,
                tokens,
            };
            if self.prev_scalar.is_multiline {
                self.push_error(ErrorType::ImplicitKeysNeedToBeInline);
            }
            self.last_map_line = scalar_line;
            if is_map_start {
                self.next_map_state();
                self.continue_processing = true;
                if had_tab {
                    self.push_error(ErrorType::TabsNotAllowedAsIndentation);
                }
                self.tokens.push_back(MAP_START_BLOCK);
                self.push_state(BlockMap(scalar_start as u32, BeforeColon), scalar_line);
            }
        } else {
            if self.last_map_line != scalar_line && curr_state.is_incorrectly_indented(scalar_start)
            {
                self.push_error(ErrorType::ImplicitKeysNeedToBeInline);
            }
            self.next_map_state();
            self.emit_prev_anchor();
            self.tokens.extend(tokens);
        }
    }

    #[inline]
    fn emit_prev_anchor(&mut self) {
        if let Some(anchor) = take(&mut self.prev_anchor) {
            if self.had_anchor {
                self.push_error(ErrorType::NodeWithTwoAnchors);
            }
            self.tokens.push_back(ANCHOR);
            self.tokens.push_back(anchor.0);
            self.tokens.push_back(anchor.1);
        };
        self.had_anchor = false;
    }

    #[inline]
    fn skip_separation_spaces<B, R: Reader<B>>(
        &mut self,
        reader: &mut R,
        allow_comments: bool,
    ) -> (usize, bool) {
        let (lines, has_tab) = reader.skip_separation_spaces(allow_comments);
        if lines > 0 {
            self.reset_col();
        }
        (lines as usize, has_tab)
    }

    fn process_flow_seq_start<B, R: Reader<B>>(&mut self, reader: &mut R) {
        reader.consume_bytes(1);
        self.tokens.push_back(SEQ_START);

        let state = FlowSeq(self.get_token_pos(), BeforeFirstElem);
        self.push_state(state, reader.line());

        self.continue_processing = true;
    }

    fn process_flow_map_start<B, R: Reader<B>>(&mut self, reader: &mut R) {
        reader.consume_bytes(1);
        reader.skip_space_tab();
        self.emit_prev_anchor();

        if reader.peek_byte_is(b'?') {
            let state = FlowKeyExp(self.get_token_pos(), BeforeKey);
            self.push_state(state, reader.line());
        } else {
            let state = FlowMap(self.get_token_pos(), BeforeKey);
            self.push_state(state, reader.line());
        }
        self.tokens.push_back(MAP_START);
    }

    #[inline]
    fn push_empty_token(&mut self) {
        self.tokens.push_back(SCALAR_PLAIN);
        self.tokens.push_back(SCALAR_END);
    }

    #[inline]
    fn get_token_pos(&self) -> u32 {
        self.tokens.len() as u32
    }

    #[inline]
    fn pop_state(&mut self) -> Option<LexerState> {
        let pop_state = self.stack.pop();
        if let Some(state) = self.stack.last_mut() {
            match state {
                BlockMap(indent, _) | BlockMapExp(indent, _) | BlockSeq(indent) => {
                    self.last_block_indent = *indent as usize;
                }
                DocBlock => {
                    *state = AfterDocBlock;
                }
                _ => {}
            }
        };
        pop_state
    }

    fn push_state(&mut self, state: LexerState, read_line: usize) {
        match state {
            BlockMap(indent, _) | BlockMapExp(indent, _) => {
                self.last_block_indent = indent as usize;
                self.had_anchor = false;
                self.last_map_line = read_line;
            }
            BlockSeq(indent) => {
                self.last_block_indent = indent as usize;
                self.had_anchor = false;
            }
            _ => {}
        }
        self.stack.push(state);
    }

    fn pop_block_states(&mut self, unwind: usize) {
        if unwind == 0 {
            return;
        }
        for _ in 0..unwind {
            match self.pop_state() {
                Some(BlockSeq(_)) => self.tokens.push_back(SEQ_END),
                Some(BlockMap(_, AfterColon) | BlockMapExp(_, AfterColon)) => {
                    self.push_empty_token();
                    self.tokens.push_back(MAP_END)
                }
                Some(BlockMap(_, _) | BlockMapExp(_, _)) => self.tokens.push_back(MAP_END),
                _ => {}
            }
        }
    }

    fn unwind_to_root_start<B, R: Reader<B>>(&mut self, reader: &mut R) {
        let pos = reader.col();
        reader.consume_bytes(3);
        self.pop_block_states(self.stack.len().saturating_sub(1));
        self.tokens.push_back(DOC_END);
        if pos != 0 {
            self.push_error(ErrorType::ExpectedIndentDocStart {
                actual: pos,
                expected: 0,
            });
        }
        self.tokens.push_back(DOC_START_EXP);
        self.set_curr_state(DocBlock, reader.line());
    }

    fn unwind_to_root_end<B, R: Reader<B>>(&mut self, reader: &mut R) {
        let col = reader.col();
        self.pop_block_states(self.stack.len().saturating_sub(1));
        if col != 0 {
            self.push_error(ErrorType::UnxpectedIndentDocEnd {
                actual: col,
                expected: 0,
            });
        }
        self.set_curr_state(AfterDocBlock, reader.line());
    }

    fn fetch_exp_block_map_key<B, R: Reader<B>>(&mut self, reader: &mut R, curr_state: LexerState) {
        let indent = reader.col();
        self.last_map_line = reader.line();
        reader.consume_bytes(1);
        reader.skip_space_tab();
        self.emit_prev_anchor();
        match curr_state {
            DocBlock => {
                let state = BlockMapExp(indent as u32, BeforeKey);
                self.push_state(state, reader.line());
                self.tokens.push_back(MAP_START_BLOCK);
            }
            BlockMapExp(prev_indent, BeforeColon) if prev_indent as usize == indent => {
                self.push_empty_token();
                self.set_map_state(BeforeKey);
            }
            _ => {}
        }
    }

    fn fetch_tag<B, R: Reader<B>>(&mut self, reader: &mut R) {
        pub use LexerToken::*;
        let start = reader.pos();
        reader.consume_bytes(1);
        if let Ok((mid, end)) = reader.read_tag() {
            self.tokens.push_back(TAG_START);
            self.tokens.push_back(start);
            self.tokens.push_back(mid);
            self.tokens.push_back(end);
            // Dont consume the last character it could be newline
            reader.consume_bytes(end - start - 1);
        }
    }
    fn fetch_plain_scalar_block<B, R: Reader<B>>(
        &mut self,
        reader: &mut R,
        curr_state: LexerState,
        peek_chr: u8,
    ) {
        if peek_chr == b']' || peek_chr == b'}' && peek_chr == b'@' {
            reader.consume_bytes(1);
            self.push_error(UnexpectedSymbol(peek_chr as char));
            return;
        }
        let mut ends_with = b'\x7F';
        self.update_col(reader);

        let curr_state = curr_state;
        let has_tab = self.has_tab;
        let scalar_line = reader.line();

        let Scalar {
            scalar_start,
            is_multiline,
            tokens,
        } = self.get_plain_scalar(reader, curr_state, &mut ends_with);

        let is_key = ends_with == b':'
            || matches!(reader.peek_chars(), [b':', x, ..] if is_white_tab_or_break(*x))
                && matches!(curr_state, BlockMap(_, BeforeKey) | BlockSeq(_) | DocBlock);

        self.pop_other_states(is_key, curr_state, scalar_start);

        self.process_block_scalar(
            curr_state,
            is_key,
            scalar_start,
            has_tab,
            is_multiline,
            tokens,
            scalar_line,
        );
    }

    fn pop_other_states(&mut self, is_key: bool, curr_state: LexerState, scalar_start: usize) {
        let find_unwind = if is_key && matches!(curr_state, BlockSeq(_)) {
            self.find_matching_state(scalar_start,
                |curr_state, curr_indent| { matches!(curr_state, BlockMap(ind,_) if ind as usize == curr_indent)
            })
        } else if !is_key && !matches!(curr_state, BlockMap(_, _)) {
            self.find_matching_state(scalar_start,
                |curr_state, curr_indent| { matches!(curr_state, BlockSeq(ind) if ind as usize == curr_indent)
            })
        } else {
            None
        };
        if let Some(unwind) = find_unwind {
            self.pop_block_states(unwind);
        }
    }

    fn process_colon_block<B, R: Reader<B>>(&mut self, reader: &mut R, curr_state: LexerState) {
        let indent = self.last_block_indent;
        let colon_pos = reader.col();
        let col = self.col_start.unwrap_or(colon_pos);
        reader.consume_bytes(1);

        if colon_pos == 0 && curr_state == DocBlock {
            let state = BlockMap(0, AfterColon);
            self.push_state(state, reader.line());
            self.tokens.push_back(MAP_START_BLOCK);
            self.push_empty_token();
        } else if matches!(curr_state, BlockMap(ind, BeforeKey) if colon_pos == ind as usize) {
            self.push_empty_token();
            self.set_map_state(AfterColon);
        } else if matches!(curr_state, BlockMap(ind, AfterColon) if colon_pos == ind as usize)
            && !self.prev_scalar.is_empty()
        {
            if !self.prev_scalar.is_empty() {
                self.emit_prev_anchor();
                self.set_map_state(AfterColon);
                self.tokens.extend(take(&mut self.prev_scalar.tokens));
            }
            self.push_empty_token();
        } else if matches!(curr_state, BlockMapExp(_, _) if colon_pos != indent ) {
            self.push_error(ExpectedIndent {
                actual: col,
                expected: indent,
            });
            self.next_map_state();
        } else if !self.prev_scalar.is_empty()
            && matches!(curr_state, BlockMap(ind, AfterColon) if ind as usize == self.prev_scalar.scalar_start)
        {
            self.push_empty_token();
        } else if matches!(curr_state, BlockMap(ind, BeforeColon) if col == ind as usize)
            || matches!(curr_state, BlockMapExp(ind, _) if colon_pos == ind as usize)
            || matches!(curr_state, BlockMap(_, _) if col > indent)
        {
            self.next_map_state();
        } else if let Some(unwind) = self.find_matching_state(
                col,
                |state, indent| matches!(state, BlockMap(ind, _) | BlockMapExp(ind, _) if ind as usize == indent),
            ) {
                self.pop_block_states(unwind);
            } else {
                self.push_error(ExpectedIndent {
                    actual: reader.col(),
                    expected: indent,
                });
            }

        if !self.prev_scalar.is_empty() {
            self.emit_prev_anchor();
            self.set_map_state(AfterColon);
            self.tokens.extend(take(&mut self.prev_scalar.tokens));
        }
    }

    fn process_block_seq<B, R: Reader<B>>(&mut self, reader: &mut R, curr_state: LexerState) {
        let indent = reader.col();
        let expected_indent = self.last_block_indent;
        reader.consume_bytes(1);

        let new_seq = match curr_state {
            DocBlock => true,
            BlockSeq(ind) if indent > ind as usize => true,
            BlockSeq(ind) if indent == ind as usize => false,
            _ => {
                if let Some(last_seq) = self.stack.iter().rposition(|x| matches!(x, BlockSeq(_))) {
                    if let Some(unwind) = self.find_matching_state(
                        indent,
                        |state, indent| matches!(state, BlockSeq(ind) if ind as usize == indent),
                    ) {
                        self.pop_block_states(unwind);
                    } else {
                        self.pop_block_states(self.stack.len() - last_seq);
                        self.push_error(ExpectedIndent {
                            actual: indent,
                            expected: expected_indent,
                        });
                    }
                    false
                } else {
                    self.next_map_state();
                    true
                }
            }
        };
        if new_seq {
            if self.prev_anchor.is_some() && !self.had_anchor {
                self.push_error(ErrorType::InvalidAnchorDeclaration);
            }
            self.emit_prev_anchor();
            self.push_state(BlockSeq(indent as u32), reader.line());
            self.tokens.push_back(SEQ_START_BLOCK);
        }
    }

    fn find_matching_state(
        &self,
        matching_indent: usize,
        f: fn(LexerState, usize) -> bool,
    ) -> Option<usize> {
        self.stack
            .iter()
            .rposition(|state| f(*state, matching_indent))
            .map(|x| self.stack.len() - x - 1)
    }

    fn fetch_plain_scalar_flow<B, R: Reader<B>>(&mut self, reader: &mut R, curr_state: LexerState) {
        let mut ends_with = b'\x7F';
        let scalar = self.get_plain_scalar(reader, curr_state, &mut ends_with);

        if ends_with == b':' {
            self.prev_scalar = scalar;
            self.continue_processing = true;
        } else {
            self.emit_prev_anchor();
            self.tokens.extend(scalar.tokens);
        }
    }

    fn get_plain_scalar<B, R: Reader<B>>(
        &mut self,
        reader: &mut R,
        curr_state: LexerState,
        ends_with: &mut u8,
    ) -> Scalar {
        let mut curr_indent = match curr_state {
            BlockMapExp(ind, _) => ind as usize,
            _ => reader.col(),
        };
        let start_line = reader.line();
        let mut end_line = reader.line();
        let mut tokens = vec![ScalarPlain as usize];
        let mut offset_start = None;
        let in_flow_collection = curr_state.in_flow_collection();
        let mut had_comment = false;
        let mut num_newlines = 0;
        let scalar_start = self.scalar_start(curr_state, reader.col());
        let scalar_limit = match curr_state {
            BlockSeq(x) => x as usize,
            _ => scalar_start,
        };

        while !reader.eof() {
            if curr_indent < scalar_limit && start_line != reader.line() {
                // if plain scalar is less indented than previous
                // It can be
                // a) Part of BlockMap so we must break
                // b) An error outside of block map
                // However not important for first line.
                match curr_state {
                    DocBlock => {
                        reader.read_line();
                        tokens.push(ErrorToken as usize);
                        self.errors.push(ExpectedIndent {
                            actual: curr_indent,
                            expected: scalar_start,
                        });
                    }
                    BlockMap(ind, _) | BlockMapExp(ind, _) => {
                        reader.read_line();
                        tokens.push(ErrorToken as usize);
                        self.errors.push(ExpectedIndent {
                            actual: curr_indent,
                            expected: ind as usize,
                        });
                    }
                    _ => {}
                }
                break;
            }

            let (start, end, error) =
                reader.read_plain_one_line(offset_start, &mut had_comment, in_flow_collection);

            if let Some(err) = error {
                tokens.push(ErrorToken as usize);
                self.errors.push(err);
            };

            match num_newlines {
                x if x == 1 => {
                    tokens.push(NewLine as usize);
                    tokens.push(0);
                }
                x if x > 1 => {
                    tokens.push(NewLine as usize);
                    tokens.push(x - 1);
                }
                _ => {}
            }

            tokens.push(start);
            tokens.push(end);
            end_line = reader.line();
            reader.skip_space_tab();

            if reader.peek_byte().map_or(false, is_newline) {
                let folded_newline = self.skip_separation_spaces(reader, false);
                if reader.col() >= self.last_block_indent {
                    num_newlines = folded_newline.0;
                }
                curr_indent = reader.col();
                *ends_with = u8::min(*ends_with, b'\n')
            }

            let chr = reader.peek_byte_at(0).unwrap_or(b'\0');

            if chr == b'-' && matches!(curr_state, BlockSeq(ind) if reader.col() > ind as usize) {
                offset_start = Some(reader.pos());
            } else if (in_flow_collection && is_flow_indicator(chr)) || chr == b':' || chr == b'-' {
                *ends_with = u8::min(*ends_with, chr);
                break;
            } else if self.find_matching_state(
                curr_indent,
                |state, indent| matches!(state, BlockMap(ind, _)| BlockSeq(ind) | BlockMapExp(ind, _) if ind as usize == indent)
            ).is_some() {
                break;
            }
        }
        let is_multiline = end_line != start_line;
        tokens.push(ScalarEnd as usize);
        Scalar {
            scalar_start,
            is_multiline,
            tokens,
        }
    }

    fn scalar_start(&mut self, curr_state: LexerState, curr_col: usize) -> usize {
        match curr_state {
            BlockMapExp(ind, _) => ind as usize,
            BlockSeq(_) | BlockMap(_, BeforeColon | AfterColon) | DocBlock => {
                self.col_start.unwrap_or(curr_col)
            }
            _ => curr_col,
        }
    }

    fn fetch_explicit_map<B, R: Reader<B>>(&mut self, reader: &mut R, curr_state: LexerState) {
        if !self.is_flow_map() {
            self.tokens.push_back(MAP_START);
        }

        if !reader.peek_byte2().map_or(false, is_white_tab_or_break) {
            let scalar = self.get_plain_scalar(reader, curr_state, &mut b'\0');
            self.prev_scalar = scalar;
            self.continue_processing = true;
        } else {
            reader.consume_bytes(1);
            reader.skip_space_tab();
        }
    }

    pub const fn get_default_namespace(namespace: &[u8]) -> Option<Cow<'static, [u8]>> {
        match namespace {
            b"!!str" => Some(Cow::Borrowed(b"tag:yaml.org,2002:str")),
            b"!!int" => Some(Cow::Borrowed(b"tag:yaml.org,2002:int")),
            b"!!null" => Some(Cow::Borrowed(b"tag:yaml.org,2002:null")),
            b"!!bool" => Some(Cow::Borrowed(b"tag:yaml.org,2002:bool")),
            b"!!float" => Some(Cow::Borrowed(b"tag:yaml.org,2002:float")),
            b"!!map" => Some(Cow::Borrowed(b"tag:yaml.org,2002:map")),
            b"!!seq" => Some(Cow::Borrowed(b"tag:yaml.org,2002:seq")),
            b"!!set" => Some(Cow::Borrowed(b"tag:yaml.org,2002:set")),
            b"!!binary" => Some(Cow::Borrowed(b"tag:yaml.org,2002:binary")),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn curr_state(&self) -> LexerState {
        *self.stack.last().unwrap_or(&LexerState::default())
    }

    #[inline(always)]
    pub fn set_curr_state(&mut self, state: LexerState, read_line: usize) {
        match self.stack.last_mut() {
            Some(x) => *x = state,
            None => self.push_state(state, read_line),
        }
    }

    #[inline]
    fn set_map_state(&mut self, map_state: MapState) {
        match self.stack.last_mut() {
            Some(FlowMap(_, state)) | Some(BlockMap(_, state)) | Some(BlockMapExp(_, state)) => {
                *state = map_state
            }
            _ => {}
        };
    }

    #[inline]
    fn set_seq_state(&mut self, seq_state: SeqState) {
        if let Some(FlowSeq(_, state)) = self.stack.last_mut() {
            *state = seq_state;
        };
    }

    #[inline]
    fn next_map_state(&mut self) {
        let new_state = match self.stack.last() {
            Some(FlowMap(ind, state)) => FlowMap(*ind, state.next_state()),
            Some(FlowKeyExp(ind, state)) => FlowKeyExp(*ind, state.next_state()),
            Some(BlockMap(ind, state)) => BlockMap(*ind, state.next_state()),
            Some(BlockMapExp(ind, AfterColon)) => BlockMap(*ind, BeforeKey),
            Some(BlockMapExp(ind, state)) => BlockMapExp(*ind, state.next_state()),
            _ => return,
        };
        if let Some(x) = self.stack.last_mut() {
            *x = new_state
        };
    }

    #[inline(always)]
    pub fn pop_token(&mut self) -> Option<usize> {
        self.tokens.pop_front()
    }

    #[inline(always)]
    pub fn tokens(self) -> VecDeque<usize> {
        self.tokens
    }

    #[inline(always)]
    pub fn peek_token(&mut self) -> Option<usize> {
        self.tokens.front().copied()
    }

    #[inline(always)]
    pub fn peek_token_next(&mut self) -> Option<usize> {
        self.tokens.get(1).copied()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    #[inline]
    fn update_col<B, R: Reader<B>>(&mut self, reader: &R) -> usize {
        match self.col_start {
            Some(x) => x,
            None => {
                let col = reader.col();
                self.col_start = Some(col);
                col
            }
        }
    }

    #[inline]
    fn reset_col(&mut self) {
        self.col_start = None;
        self.has_tab = false;
    }

    #[inline]
    fn is_prev_sequence(&self) -> bool {
        match self.stack.iter().nth_back(1) {
            Some(FlowSeq(_, _)) => true,
            _ => false,
        }
    }

    #[inline]
    fn is_unfinished(&self) -> bool {
        match self.curr_state() {
            FlowMap(_, AfterColon) | FlowKeyExp(_, AfterColon) => true,
            _ => false,
        }
    }

    #[inline]
    fn is_flow_map(&self) -> bool {
        match self.curr_state() {
            FlowMap(_, _) | FlowKeyExp(_, _) => true,
            _ => false,
        }
    }

    fn process_single_quote_block<B, R: Reader<B>>(
        &mut self,
        reader: &mut R,
        curr_state: LexerState,
    ) {
        let has_tab = self.has_tab;
        let scalar_line = reader.line();
        let Scalar {
            scalar_start,
            is_multiline,
            tokens,
        } = self.process_single_quote(reader);
        reader.skip_space_tab();

        let ends_with = reader.peek_byte().map_or(false, |chr| chr == b':');

        self.process_block_scalar(
            curr_state,
            ends_with,
            scalar_start,
            has_tab,
            is_multiline,
            tokens,
            scalar_line,
        );
    }
}

const DOC_END: usize = usize::MAX;
const DOC_END_EXP: usize = usize::MAX - 1;
const DOC_START: usize = usize::MAX - 2;
const DOC_START_EXP: usize = usize::MAX - 3;
const MAP_END: usize = usize::MAX - 4;
const MAP_START: usize = usize::MAX - 5;
const MAP_START_BLOCK: usize = usize::MAX - 6;
const SEQ_END: usize = usize::MAX - 7;
const SEQ_START: usize = usize::MAX - 8;
const SEQ_START_BLOCK: usize = usize::MAX - 9;
const SCALAR_PLAIN: usize = usize::MAX - 10;
const SCALAR_FOLD: usize = usize::MAX - 11;
const SCALAR_LIT: usize = usize::MAX - 12;
const SCALAR_QUOTE: usize = usize::MAX - 13;
const SCALAR_DQUOTE: usize = usize::MAX - 14;
const SCALAR_END: usize = usize::MAX - 15;
const TAG_START: usize = usize::MAX - 16;
const ANCHOR: usize = usize::MAX - 17;
const ALIAS: usize = usize::MAX - 18;
const DIR_RES: usize = usize::MAX - 19;
const DIR_TAG: usize = usize::MAX - 20;
const DIR_YAML: usize = usize::MAX - 21;
const ERROR_TOKEN: usize = usize::MAX - 22;
const NEWLINE: usize = usize::MAX - 32;

#[repr(usize)]
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(clippy::enum_clike_unportable_variant)] //false positive see https://github.com/rust-lang/rust-clippy/issues/8043
///
/// [LexerToken] used to Lex YAML files
pub enum LexerToken {
    /// Denotes that value is a [usize] less than [NewLine] and thus its meaning decided by previous Tokens
    /// usually marks a start/end token.
    Mark,
    /// Denotes a newline and must be followed by a [Mark]. If next Mark is 0, it's space otherwise it's a `n`
    /// number of newlines `\n`
    NewLine = NEWLINE,
    /// Error in stream, check [Lexer.errors] for details
    ErrorToken = ERROR_TOKEN,
    /// Directive Tag denoted by `%TAG` and followed by two [Mark] tokens
    DirectiveTag = DIR_TAG,
    /// Directive Tag denoted by `@value` and followed by two [Mark] tokens
    DirectiveReserved = DIR_RES,
    /// YAML directive showing minor/major version of e.g.
    /// ```yaml
    ///     %YAML 1.2
    /// ```
    DirectiveYaml = DIR_YAML,
    /// Plain Scalar that's neither quoted or literal or folded
    /// ```yaml
    ///     example: plain_scalar
    /// ```
    ScalarPlain = SCALAR_PLAIN,
    /// Helper token to end token
    ScalarEnd = SCALAR_END,
    /// Folded scalar token
    /// ```yaml
    ///     example: >
    ///         folded_scalar
    /// ```
    ScalarFold = SCALAR_FOLD,
    /// Literal scalar token
    /// ```yaml
    ///     example: |
    ///         literal_scalar
    /// ```
    ScalarLit = SCALAR_LIT,
    /// Single quoted scalar
    /// ```yaml
    ///     example: 'single quote scalar'
    /// ```
    ScalarSingleQuote = SCALAR_QUOTE,
    /// Double quoted scalar
    /// ```yaml
    ///     example: "double quote scalar"
    /// ```
    ScalarDoubleQuote = SCALAR_DQUOTE,
    /// Element with alternative name e.g. `&foo [x,y]`
    AnchorToken = ANCHOR,
    /// Reference to an element with alternative name e.g. `*foo`
    AliasToken = ALIAS,
    /// Tag
    TagStart = TAG_START,
    /// Start of a sequence token, e.g. `[` in
    /// ```yaml
    ///  [a, b, c]
    /// #^-- start of sequence
    /// ```
    SequenceStart = SEQ_START,
    /// Start of a sequence token, e.g. `[` in
    /// ```yaml
    ///  [a, b, c]
    /// #^-- start of sequence
    /// ```
    SequenceStartImplicit = SEQ_START_BLOCK,
    /// End of a sequence token, e.g. `]` in
    /// ```yaml
    ///  [a, b, c]
    /// #        ^-- end of sequence
    /// ```
    SequenceEnd = SEQ_END,
    /// Start of a map  token, e.g. `{` in
    /// ```yaml
    ///  { a: b,}
    /// #^-- start of mapping
    /// ```
    MappingStart = MAP_START,
    /// Start of a map  token, e.g. `{` in
    /// ```yaml
    ///   [a]: 3
    /// #^-- start of mapping
    /// ```
    MappingStartImplicit = MAP_START_BLOCK,
    /// End of a map  token, e.g. `}` in
    /// ```yaml
    ///  { a: b}
    /// #      ^-- start of mapping
    /// ```
    MappingEnd = MAP_END,
    /// Start of implicit Document
    DocumentStart = DOC_START,
    /// Start of explicit Document
    DocumentStartExplicit = DOC_START_EXP,
    /// End of implicit document.
    DocumentEnd = DOC_END,
    /// End of explicit document.
    DocumentEndExplicit = DOC_END_EXP,
}

impl LexerToken {
    ///
    /// This method transforms a [LexerToken] into a [DirectiveType]
    ///
    /// It's UB to call on any [LexerToken] that isn't [DirectiveTag], [DirectiveYaml], or  [DirectiveReserved].
    #[inline(always)]
    pub(crate) unsafe fn to_yaml_directive(self) -> DirectiveType {
        match self {
            DirectiveTag => DirectiveType::Tag,
            DirectiveYaml => DirectiveType::Yaml,
            DirectiveReserved => DirectiveType::Reserved,
            _ => unreachable_unchecked(),
        }
    }

    ///
    /// This method transforms a [LexerToken] into a [ScalarType]
    ///
    /// It's UB to call on any [LexerToken] that isn't [ScalarPlain], [Mark], [ScalarFold], [ScalarLit],
    /// [ScalarSingleQuote], [ScalarDoubleQuote].
    #[inline(always)]
    pub(crate) unsafe fn to_scalar(self) -> ScalarType {
        match self {
            ScalarPlain | Mark => ScalarType::Plain,
            ScalarFold => ScalarType::Folded,
            ScalarLit => ScalarType::Literal,
            ScalarSingleQuote => ScalarType::SingleQuote,
            ScalarDoubleQuote => ScalarType::DoubleQuote,
            _ => unreachable_unchecked(),
        }
    }
}

impl From<usize> for LexerToken {
    fn from(value: usize) -> Self {
        pub use LexerToken::*;

        match value {
            DOC_END => DocumentEnd,
            DOC_END_EXP => DocumentEndExplicit,
            DOC_START => DocumentStart,
            DOC_START_EXP => DocumentStartExplicit,
            MAP_END => MappingEnd,
            MAP_START => MappingStart,
            MAP_START_BLOCK => MappingStartImplicit,
            SEQ_START_BLOCK => SequenceStartImplicit,
            SEQ_END => SequenceEnd,
            SEQ_START => SequenceStart,
            SCALAR_PLAIN => ScalarPlain,
            SCALAR_END => ScalarEnd,
            SCALAR_FOLD => ScalarFold,
            SCALAR_LIT => ScalarLit,
            SCALAR_QUOTE => ScalarSingleQuote,
            SCALAR_DQUOTE => ScalarDoubleQuote,
            TAG_START => TagStart,
            ANCHOR => AnchorToken,
            ALIAS => AliasToken,
            DIR_RES => DirectiveReserved,
            DIR_TAG => DirectiveTag,
            DIR_YAML => DirectiveYaml,
            NEWLINE => NewLine,
            ERROR_TOKEN => ErrorToken,
            _ => Mark,
        }
    }
}

impl From<&usize> for LexerToken {
    fn from(value: &usize) -> Self {
        LexerToken::from(*value)
    }
}
