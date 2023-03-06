pub struct Init;
pub struct Parsed;
pub struct Html;

use crate::{Markdown, Rule};
use pest::iterators::{Pair, Pairs};

use std::{
    borrow::Cow, //
    fs::read_to_string,
    marker::PhantomData,
};

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
        let content = read_to_string(file)?;

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
            Some(pairs) => pairs,
            None => panic!("pairs() called in invalid state!"),
        }
    }
}
