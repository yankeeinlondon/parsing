use std::fmt::Display;

use pest::{error::Error, iterators::Pairs, RuleType};

pub trait PestilentParser {
    fn parse<R: RuleType>(rule: R, input: &str) -> Result<Pairs<'_, R>, Error<R>>;

    /// Produces a string representation of a parsed rule
    /// where
    fn describe(&self) -> String;

    fn text(&self) -> String;

    fn is_leaf_node(&self) -> bool;

    fn has_only_child(&self) -> bool;
}

// impl Display for PestilentParser,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.text())
//     }
// }
