extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::{Parser, Span};
use std::{collections::HashMap, fmt::Display, sync::Once};
use tracing::instrument;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        color_eyre::install().expect("failed to install error reporter");
    });
}

#[derive(Parser)]
#[grammar = "markdown.pest"]
pub struct Markdown;

use color_eyre::{eyre::eyre, eyre::Error as ColorError, eyre::Report, Result, Section};
use pest::iterators::{Pair, Pairs};

/// **parse_rule**
///
/// Parses markdown content using a specified rule defined Markdown
/// struct/parser
#[instrument]
pub fn parse_rule(rule: Rule, content: &str) -> Result<Pairs<Rule>, Report> {
    let res = Markdown::parse(rule, &content);

    match res {
        Ok(pairs) => Ok(pairs),
        Err(err) => {
            Err(eyre!("Failed to parse rule '{:?}'!", rule)) //
                .with_section(|| format!("The Error was structured as:\n {:#?}", err))
        }
    }
}

#[instrument]
pub fn test_parse(rule: Rule, content: &str) -> Option<Pairs<Rule>> {
    let res = Markdown::parse(rule, &content);

    match res {
        Ok(pairs) => Some(pairs),
        Err(err) => {
            println!("The {:?} rule failed to parse while trying to process the text:\n\n{}!\n\nThe parse error is:\n{:#?} ", rule, content, err );

            assert!(false);
            None
        }
    }
}

/// **parse_or_panic**
///
/// Parses the passed in _content_ with the given _rule_ or **panics** if it
/// can't parse.
#[instrument]
pub fn parse_or_panic(rule: Rule, content: &str) -> Pairs<Rule> {
    parse_rule(rule, content).unwrap_or_else(|err| panic!("{:?}\n", err))
}

fn pad(level: usize, content: String) -> String {
    let mut padding = format!("{}", " ".repeat(level * 2));
    padding.push_str(&content);

    padding
}

/// Tests whether a given `Pairs` only has a single child
pub fn is_only_child(pairs: &Pairs<Rule>) -> bool {
    let arr: Vec<Pair<Rule>> = pairs.clone().collect();
    arr.len() == 1
}

pub enum ParsedContainer<'a> {
    Pairs(&'a Pairs<'a, Rule>),
    Pair(&'a Pair<'a, Rule>),
    Span(&'a Span<'a>),
}

/// The text value of any set of `Pairs`
pub fn get_text<'a, T: Into<ParsedContainer<'a>>>(container: T) -> String {
    let mut result: String = "".to_string();
    let container = container.into();

    match container {
        ParsedContainer::Pairs(pairs) => {
            let spans: Vec<&str> = pairs
                .clone() //
                .map(|p| p.as_span())
                .map(|s| {
                    let txt = s.get(0..);
                    match txt {
                        Some(txt) => txt.as_str(),
                        None => "",
                    }
                })
                .collect();

            for s in spans.into_iter() {
                result.push_str(s);
            }
        }
        ParsedContainer::Pair(pair) => {
            if let Some(text) = pair.as_span().get(0..) {
                text.as_str()
            } else {
                ""
            };
        }
        ParsedContainer::Span(span) => {
            if let Some(text) = span.get(0..) {
                text.as_str()
            } else {
                ""
            };
        }
    }

    result
}

#[derive(Debug, Clone)]
pub struct RuleChain<'a> {
    /// The root rule pairing which this `RuleChain`
    /// derives from.
    pub pair: Pair<'a, Rule>,
    /// An optional hashmap created when needed and
    /// used primarily to improve performance for
    /// certain operations
    mapping: HashMap<String, Vec<RuleChain<'a>>>,
}

impl<'a> From<Pair<'a, Rule>> for RuleChain<'a> {
    #[instrument]
    fn from(pair: Pair<'a, Rule>) -> Self {
        RuleChain::new(pair.clone())
    }
}
impl<'a> From<&'a Pair<'a, Rule>> for RuleChain<'a> {
    #[instrument]
    fn from(pair: &'a Pair<Rule>) -> Self {
        RuleChain::new(pair.to_owned())
    }
}
impl<'a> TryFrom<Pairs<'a, Rule>> for RuleChain<'a> {
    type Error = Report;

    #[instrument]
    fn try_from(value: Pairs<'a, Rule>) -> Result<RuleChain<'a>, Self::Error> {
        let first: Option<Pair<'a, Rule>> = value.clone().next();

        if let Some(first) = first {
            let second = value.next();
            if let Some(second) = second {
                Err(eyre!("The Pairs<Rule> structure passed in has more than one root rule which is not allowed for in a RuleChain!"))
            } else {
                let chain: RuleChain<'a> = RuleChain::from(first);
                Ok(chain)
            }
        } else {
            Err(eyre!("The Pairs<Rule> structure had no rules in it!"))
        }
    }
}

impl<'a> RuleChain<'a> {
    #[instrument]
    pub fn new(pair: Pair<'a, Rule>) -> Self {
        Self {
            pair,
            mapping: HashMap::new(),
        }
    }

    pub fn parse(rule: Rule, content: &'a str) -> Result<RuleChain<'a>> {
        let res = Markdown::parse(rule, &content);

        match res {
            Ok(pairs) => {
                let pairs: Pairs<'a, Rule> = pairs;
                match RuleChain::try_from(pairs) {
                    Ok(chain) => Ok(chain as RuleChain<'a>),
                    Err(err) => Err(err),
                }
            }
            Err(err) => {
                Err(eyre!("Failed to parse rule '{:?}'!", rule)) //
                .with_section(|| format!("The Error was structured as:\n {:#?}", err))
            }
        }
    }

    /// the _name_ of this rule
    #[instrument]
    pub fn name(&self) -> String {
        format!("{:?}", self.pair.as_rule())
    }
    /// index pos in the source content where this
    /// rule's matching starts
    #[instrument]
    pub fn get_start(&self) -> usize {
        self.pair.as_span().start()
    }
    /// index pos in the source content where this
    /// rule's matching ends
    #[instrument]
    pub fn get_end(&self) -> usize {
        self.pair.as_span().end()
    }

    /// the textual content which this rule captures
    /// between the `start` and `end` indexes of the
    /// source content.
    #[instrument]
    pub fn get_text(&self) -> String {
        if let Some(text) = self.pair.as_span().get(0..) {
            text.as_str().to_string()
        } else {
            "".to_string()
        }
    }

    /// Gets the text which a given rule in this rule's
    /// "rule chain" consumes.
    ///
    /// - if the rule is not defined then an empty String is returned
    /// - if more than one rule with the specified _rule name_ is within
    /// the chain then the text for all will be concatenated.
    #[instrument]
    pub fn get_rule_text(&mut self, rule: &str) -> String {
        self.prep_mapping();

        let rules = self.mapping.get(rule);

        if let Some(rules) = rules {
            let rules: Vec<String> = rules
                .into_iter()
                .map(|r: &RuleChain| r.get_text())
                .collect();

            rules.concat()
        } else {
            "".to_string()
        }
    }

    /// whether or not this rule has child rules used to capture
    /// sub-sections of the source content.
    #[instrument]
    pub fn has_children(&self) -> bool {
        let arr: Vec<Pair<Rule>> = self.pair.clone().into_inner().collect();
        arr.len() > 0
    }

    /// makes sure the mapping property is available
    #[instrument]
    fn prep_mapping(&mut self) {
        if self.mapping.is_empty() {
            // ensure mapping hashmap
            let mut mapping: HashMap<String, Vec<RuleChain<'a>>> = HashMap::new();

            let map_children =
                move |
                mapping: &mut HashMap<String, Vec<RuleChain<'a>>>,
                pairs: &mut Pairs<'a, Rule> //
            | {
                for child in pairs {
                    let rule = RuleChain::from(child);
                    let name = rule.name();

                    if !mapping.contains_key(&name) {
                        mapping.insert(
                            name.clone(), //
                            Vec::with_capacity(10),
                        );
                    }

                    let v = mapping.get_mut(&name).unwrap();
                    v.push(rule);
                }
            };

            let mut root_children: Pairs<'a, Rule> = self.pair.clone().into_inner();

            map_children(&mut mapping, &mut root_children);

            self.mapping = mapping;
        };
    }

    #[instrument]
    fn push_rule(&'a mut self, name: &str, rule: RuleChain<'a>) {
        let name = name.to_string();

        let has_key = self.mapping.contains_key(&name);

        if !has_key {
            self.mapping
                .insert(name.to_string(), Vec::with_capacity(10));
        }

        // get the KV pair associated with current rule
        let kv = self.mapping.get_mut(&name).unwrap();

        kv.push(rule);
    }

    pub fn get_rule_name(&'a self, rule_name: &str) -> Option<&Vec<RuleChain<'a>>> {
        match self.mapping.get(rule_name) {
            Some(rules) => Some(rules),
            None => None,
        }
    }

    /// Get's the first rule with a given name found in the current
    /// `RuleChain` or None if there are no instances of that name.
    #[instrument]
    pub fn find_rule(&'a mut self, rule_name: &str) -> Option<RuleChain<'a>> {
        // self.prep_mapping();

        // let rules = self.mapping.unwrap();

        match self.get_rule_name(rule_name) {
            Some(rules) => match rules.first() {
                Some(rule) => {
                    let rule: RuleChain<'a> = RuleChain::new(rule.pair.clone());
                    return Some(rule);
                }
                None => None,
            },
            None => None,
        }

        // if rules.is_some() {
        //     let rule: Vec<RuleChain<'a>> = rules.unwrap().clone();

        //     if let Some(first) = rule.first() {
        //         Some(first.clone())
        //     } else {
        //         None
        //     }
        // } else {
        //     None
        // }
    }

    /// Gets _all_ of the `RuleChains` for a given rule name.
    ///
    /// - returns an empty vector if this rule does not exist
    #[instrument]
    pub fn get_rules(&mut self, rule_name: &str) -> Option<Vec<RuleChain<'a>>> {
        self.prep_mapping();

        match self.mapping.get(rule_name) {
            Some(rules) => {
                let rules: Vec<RuleChain<'a>> = rules.clone();
                Some(rules)
            }
            None => None,
        }
    }

    /// Provides a descriptive syntax for the RuleChain
    /// that is useful in exploring your results across
    /// all child-rules.
    #[instrument]
    pub fn describe(&self) -> String {
        let children = self.get_children();
        let root_node = format!(
            "\n[{} is \"{}\"], composed of [\n{}", //
            self.name(),
            self.get_text(),
            self.describe_at_level(&children, 1 as usize)
        );

        format!("{}", root_node)
    }

    #[instrument]
    fn describe_at_level(&self, pairs: &Pairs<Rule>, level: usize) -> String {
        let mut result = "".to_string();

        for pair in pairs.clone() {
            let rule_name = format!("{:?}", pair.as_rule());
            let text = if let Some(text) = pair.as_span().get(0..) {
                text.as_str()
            } else {
                ""
            };
            let mut inner: Pairs<Rule> = pair.into_inner();
            let children: &Vec<Pair<Rule>> = &inner.clone().collect();
            let has_children = children.len() != 0;
            let is_orphan = children.len() == 1;

            // not a root level node
            if has_children {
                if is_orphan {
                    let child: Pair<Rule> = inner.next().unwrap();
                    let child_rule = format!("{:?}", &child.as_rule());
                    let grand_children = child.into_inner();

                    let grand_child_is_orphan = is_only_child(&grand_children);

                    let grand_children = format!(
                        "{} -> {} [\n{}\n{}\n",
                        pad(level, rule_name),
                        child_rule,
                        if grand_child_is_orphan {
                            pad(level, get_text(ParsedContainer::Pairs(&inner)))
                        } else {
                            self.describe_at_level(&grand_children, level + 1)
                        },
                        pad(level, "],\n".to_string())
                    );
                    result.push_str(&grand_children);
                } else {
                    let children = self.describe_at_level(&inner, level + 1);
                    let children = format!(
                        "{} [\n{}\n{}",
                        pad(level, rule_name),
                        children,
                        pad(level, "],\n".to_string())
                    );
                    result.push_str(&children);
                }
            } else {
                let leaf_node = format!(
                    "{} [\"{}\"],\n", //
                    pad(level, rule_name),
                    text
                );
                result.push_str(&leaf_node);
            }
        }

        result
    }

    /// checks whether a given rule was found in the
    /// given rule's rule chain.
    #[instrument]
    pub fn has_rule(&'a mut self, rule: &str) -> bool {
        self.get_rule_name(rule).is_some()
    }

    #[instrument]
    pub fn how_many(&'a mut self, rule: &str) -> usize {
        todo!()
    }

    /// clones the node and returns the child nodes as owned
    /// `Pairs<Rule>`
    #[instrument]
    pub fn get_children(&'a self) -> Pairs<'a, Rule> {
        self.pair.clone().into_inner()
    }
}

impl<'a> Display for RuleChain<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name(), self.get_text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_attrs() {
        initialize();
        let p = parse_rule(
            Rule::attrs, //
            "   class=\"foo bar baz\" data-flag=\"false\"  ",
        );

        assert!(matches!(p, Ok(_)))
    }

    #[test]
    fn self_closing_tag() {
        initialize();
        let p = RuleChain::parse(
            Rule::tag, //
            "<test class=\"foo bar\" style=\"color: red\" />",
        );

        if let Ok(tag) = p {
            println!("{}", tag.describe());
        }
    }
    #[test]
    fn block_tag() {
        initialize();
        let p1 = RuleChain::parse(
            Rule::tag,
            r#"<foo-bar class="foo bar">hello world</foo-bar>"#,
        );

        match p1 {
            Ok(tag) => println!("{}", tag.describe()),
            Err(err) => {
                panic!("{}", err);
            }
        }

        // let p2 = Markdown::parse(
        //     Rule::tag,
        //     "<foo-bar    class=\"foo bar\">\nhello world\n</foo-bar >",
        // );
    }

    #[test]
    fn h1() {
        initialize();
        let p = RuleChain::parse(
            Rule::h1, //
            "# Foobar\n",
        );

        assert!(matches!(p, Ok(_)));
    }
    #[test]
    fn h6_with_two_space_indent() {
        initialize();
        let p = RuleChain::parse(
            Rule::h6, //
            "  ###### Foobar\n",
        );

        assert!(matches!(p, Ok(_)));
    }
    #[test]
    fn h4_from_heading() {
        let p = RuleChain::parse(
            Rule::heading, //
            " #### Foobar\n",
        );
        assert!(matches!(p, Ok(_)));
    }

    #[test]
    fn fenced_code_block() {
        initialize();
        let mut dict = RuleChain::parse(
            Rule::fence_defn,
            r#"```ts { foo: "bar", bar: "baz" } bad-juju"#,
        );
        if let Ok(tag) = dict {
            println!("{}", &tag.describe());

            let lang = tag.get_rule_text("lang");
            assert!(matches!(lang.as_str(), "ts"));
        }

        // let csv = parse_rule(
        //     Rule::fence_defn,
        //     r#"```ts foo: "bar", bar: "baz" bad-juju\n"#,
        // );
        // assert!(matches!(csv, Ok(_)));
    }

    #[test]
    fn thematic_break() {
        initialize();
        let md = r#"
# Foobar

something
---
something else
        "#;
        let p = parse_rule(
            Rule::file, //
            md,
        );

        assert!(matches!(p, Ok(_)))
    }
}
