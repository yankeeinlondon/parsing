extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::pest::Parser;
use color_eyre::Result;
use pest::iterators::Pairs;
use std::{
    borrow::Cow, //
    fs::read_to_string,
    marker::PhantomData,
};

#[derive(Parser)]
#[grammar = "markdown.pest"]
pub struct Markdown;

struct Init;
struct Parsed;
struct Html;

/// Our own Markdown parser built using **Pest**.
pub struct Parkdown<'a, TState = Init> {
    /// raw markdown content
    pub content: Cow<'a, str>,
    /// the file which the content was derived from
    /// (if provided)
    file: Option<String>,
    /// rule used to parse
    pub rule: Rule,
    /// "pairs" produced by the rule
    pairs: Option<Pairs<'a, Rule>>,

    state: PhantomData<TState>,
}

// initializer implementation
impl<'a, Init> Parkdown<'a, Init> {
    /// create a new Parkdown parser with a reference
    /// to the underlying raw markdown content.
    pub fn new(content: &'a str) -> Self {
        Self {
            content: Cow::Borrowed(content),
            rule: Rule::file,
            file: None,
            pairs: None,
            state: PhantomData::<Init>,
        }
    }
    pub fn new_owned(content: String) -> Self {
        Self {
            content: Cow::Owned(content),
            rule: Rule::file,
            file: None,
            pairs: None,
            state: PhantomData::<Init>,
        }
    }

    /// Specify a specific rule within the Markdown parser along with
    /// content you wish to parse.
    pub fn with_rule(rule: Rule, content: &'a str) -> Parkdown<'a, Init> {
        Self {
            content: Cow::Borrowed(content),
            rule,
            file: None,
            pairs: None,
            state: PhantomData::<Init>,
        }
    }

    pub fn from_file(file: &str) -> Result<Parkdown<'a, Init>> {
        color_eyre::install()?;
        let content = read_to_string(&file)?;

        let mut p = Parkdown::new_owned(content);
        p.file = Some(file.to_string());

        Ok(p)
    }

    pub fn parse<'b>(&'b self) -> Result<Parkdown<'b, Parsed>> {
        color_eyre::install()?;
        let pairs: Pairs<'b, Rule> = Markdown::parse(
            self.rule, //
            &self.content,
        )?;

        let p: Parkdown<'b, Parsed> = Parkdown {
            state: PhantomData::<Parsed>,
            pairs: Some(pairs),
            content: self.content,
            rule: self.rule,
            file: self.file.clone(),
        };

        Ok(p)
    }
}

/// **parse_rule**
///
/// Parses markdown content using a specified rule defined in MdGrammar
fn parse_rule<'a>(rule: Rule, content: &'a str) -> Result<Pairs<Rule>> {
    let r = Markdown::parse(rule, content).unwrap_or_else(|e| panic!("{}", e));

    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_attrs() {
        let p = Parkdown::with_rule(
            Rule::attrs,
            "   class=\"foo bar baz\" data-flag=\"false\"  ",
        );

        let result = parse_rule(
            Rule::attrs, //
            "   class=\"foo bar baz\" data-flag=\"false\"  ",
        );
        // println!("{:#?}", &result);

        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn self_closing_tag() {
        let self_closing = "<test class=\"foo bar\" style=\"color: red\" />";
        assert!(matches!(parse_rule(Rule::tag, self_closing), Ok(_)));
    }
    #[test]
    fn block_tag() {
        let inline = "<foo-bar class=\"foo bar\">hello world</foo-bar>";
        assert!(matches!(parse_rule(Rule::tag, inline), Ok(_)));
    }

    #[test]
    fn h1_is_heading() {
        let result = parse_rule(
            Rule::h1, //
            "# Foobar",
        );
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn thematic_break() {
        let md = r#"
# Foobar

something
---
something else
        "#;
        let parser = Parkdown::new(&md);

        assert!(matches!(parser, Parkdown<'_, Init>));
    }
}
