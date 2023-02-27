extern crate pest;
#[macro_use]
extern crate pest_derive;
use color_eyre::Result;
use pest::{iterators::Pairs, Parser};

#[derive(Parser)]
#[grammar = "md.pest"]
pub struct MdGrammar;

pub fn parse<'a>(content: &'a str) -> Result<Pairs<Rule>> {
    let r = MdGrammar::parse(Rule::file, content).unwrap_or_else(|e| panic!("{}", e));

    Ok(r)
}

pub fn parse_rule<'a>(rule: Rule, content: &'a str) -> Result<Pairs<Rule>> {
    let r = MdGrammar::parse(rule, content).unwrap_or_else(|e| panic!("{}", e));

    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_attrs() {
        println!("start");
        let result = parse_rule(
            Rule::attrs, //
            "   class=\"foo bar baz\" data-flag=\"false\"  ",
        );
        println!("{:?}", &result);

        assert!(matches!(result, Ok(_)));
    }

    // #[test]
    // fn h1_is_heading() {
    //     let result = parse_rule(
    //         Rule::h1, //
    //         "# Foobar",
    //     );
    //     println!("{:?}", &result);

    //     assert!(matches!(result, Ok(_)));
    //     // assert_eq!(result, 4);
    // }
}
