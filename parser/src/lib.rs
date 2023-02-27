use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize)]
pub enum ParserStage {
    /// markdown content has been read into
    /// memory and we are ready to parse.
    Init,
    /// parser has completed creating all tokens
    Parsed,
    /// parser tokens have been converted to HTML
    Transformed,
}

#[derive(Serialize, Deserialize)]
pub enum TokenContainer {
    Pest(Box<String>),
    Nom(Box<String>),
}

pub trait AbstractParser {
    fn name(&self) -> String;
    fn tokenize(&self, content: &str) -> TokenContainer;
    fn to_html(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub enum Output {
    /// HTML output
    HTML,
    /// Parse tokens are output
    Tokens,
}

#[derive(Serialize, Deserialize)]
/**
 * Abstracts the parser being used while maintaining
 * the core state management.
 */
pub struct Parser<'a, T: AbstractParser> {
    /// the name of the parser
    pub name: String,
    pub stage: ParserStage,
    /// The output that the parser should produce
    pub output: Output,
    /// The implementation of the parser
    parser: T,

    /// A reference to the original markdown content
    /// loaded from file.
    pub md: &'a str,

    /// The tokens returned by the parser
    tokens: Option<TokenContainer>,

    /// The HTML content
    html: Option<String>,
}
