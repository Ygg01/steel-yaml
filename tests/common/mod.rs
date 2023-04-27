extern crate steel_yaml;

use std::fmt::Write;

use steel_yaml::tokenizer::{EventIterator, StrReader};

pub fn assert_eq_event(input: &str, events: &str) {
    let mut line = String::new();
    let scan: EventIterator<StrReader> = EventIterator::from(input);
    scan.for_each(|(ev, indent)| {
        line.push_str("\n");
        line.push_str(&" ".repeat(indent));
        write!(line, "{:}", ev).unwrap();
    });

    assert_eq!(events, line, "Error in {input}");
}
