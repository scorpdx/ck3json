extern crate pest;
use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammars/ck3txt.pest"]
pub struct CK3Parser;

use crate::json::JSONValue;

pub fn parse(ck3txt: &str) -> Result<JSONValue, Error<Rule>> {
    let parsing_iter = CK3Parser::parse(Rule::file, ck3txt)?.skip(1).next().unwrap();

    use pest::iterators::Pair;
    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let inner_pair = inner_rules
                            .next()
                            .expect("inner_pair was None");
                        let name = inner_pair.as_str();
                        let value = parse_value(inner_rules.next().expect("inner_rules.next() was None"));
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string
            | Rule::date => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::tag => JSONValue::String(pair.as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str() == "yes"),
            Rule::file
            | Rule::EOI
            | Rule::identifier
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::int
            | Rule::float
            | Rule::date_inner
            | Rule::WHITESPACE => {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
                println!("Text:    {}", pair.as_str());
                unreachable!()
            }
        }
    }

    Ok(parse_value(parsing_iter))
}
