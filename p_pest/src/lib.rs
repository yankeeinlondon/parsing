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

pub struct Init;
pub struct Parsed;
pub struct Html;

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
    pub fn new(content: &'a str) -> Parkdown<'a, Init> {
        Self {
            content: Cow::Borrowed(content),
            rule: Rule::file,
            file: None,
            pairs: None,
            state: PhantomData::<Init>,
        }
    }
    pub fn new_owned(content: String) -> Parkdown<'a, Init> {
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
        let content = read_to_string(&file)?;

        let mut p = Parkdown::new_owned(content);
        p.file = Some(file.to_string());

        Ok(p)
    }

    pub fn parse<'b>(&'b self) -> Result<Parkdown<'b, Parsed>> {
        let pairs: Pairs<'b, Rule> = Markdown::parse(
            self.rule, //
            &self.content,
        )?;

        let p: Parkdown<'b, Parsed> = Parkdown {
            state: PhantomData::<Parsed>,
            pairs: Some(pairs),
            content: Cow::Borrowed(&self.content),
            rule: self.rule,
            file: self.file.clone(),
        };

        Ok(p)
    }
}

impl<'a, Parsed> Parkdown<'a, Parsed> {
    pub fn pairs(&self) -> &Pairs<'a, Rule> {
        match &self.pairs {
            Some(pairs) => &pairs,
            None => panic!("pairs() called in invalid state!"),
        }
    }
}
/// **parse_rule**
///
/// Parses markdown content using a specified rule defined Markdown
/// struct/parser
pub fn parse_rule<'a>(rule: Rule, content: &'a str) -> Result<Pairs<Rule>> {
    let r: Pairs<Rule> = Markdown::parse(rule, content) //
        .unwrap_or_else(|e| panic!("{}", e));

    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_attrs() {
        let p = Markdown::parse(
            Rule::attrs, //
            "   class=\"foo bar baz\" data-flag=\"false\"  ",
        );

        assert!(matches!(p, Ok(_)))
    }

    #[test]
    fn self_closing_tag() {
        let p = Markdown::parse(
            Rule::tag, //
            "<test class=\"foo bar\" style=\"color: red\" />",
        );

        assert!(matches!(p, Ok(_)))
    }
    #[test]
    fn block_tag() {
        let p = Markdown::parse(
            Rule::block_tag,
            "<foo-bar class=\"foo bar\">hello world</foo-bar>",
        );

        assert!(matches!(p, Ok(_)))
    }

    #[test]
    fn h1() {
        let p = Markdown::parse(
            Rule::h1, //
            "# Foobar\n",
        );

        assert!(matches!(p, Ok(_)));
    }
    #[test]
    fn h6_with_two_space_indent() {
        let p = Markdown::parse(
            Rule::h6, //
            "  ###### Foobar\n",
        );

        assert!(matches!(p, Ok(_)));
    }
    #[test]
    fn h4_from_heading() {
        let p = Markdown::parse(
            Rule::heading, //
            " #### Foobar\n",
        );
        assert!(matches!(p, Ok(_)));
    }

    #[test]
    fn thematic_break() {
        let md = r#"
# Foobar

something
---
something else
        "#;
        let p = Markdown::parse(
            Rule::file, //
            &md,
        );

        assert!(matches!(p, Ok(_)))
    }
}
