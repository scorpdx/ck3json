extern crate pest;
use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammars/ck3txt.pest"]
pub struct CK3Parser;

use crate::json::JSONValue;

pub fn parse(ck3txt: &str) -> Result<JSONValue, Error<Rule>> {
    use pest::iterators::Pair;

    fn parse_pair(pair: Pair<Rule>) -> (&str, JSONValue) {
        let mut inner_rules = pair.into_inner();
        let inner_pair = inner_rules
            .next()
            .expect("inner_pair was None");
        let name = inner_pair.as_str();
        let value = parse_value(inner_rules.next().expect("inner_rules.next() was None"));
        (name, value)
    }

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::rgb
            | Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| parse_pair(pair))
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::array_pair => JSONValue::Object(vec![parse_pair(pair.into_inner().next().unwrap())]),
            Rule::string
            | Rule::date => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::tag => JSONValue::String(pair.as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str() == "yes"),
            Rule::file
            | Rule::EOI
            | Rule::body
            | Rule::identifier
            | Rule::pair
            | Rule::value
            | Rule::string_inner
            | Rule::char
            | Rule::int
            | Rule::float
            | Rule::date_inner
            | Rule::save_id
            | Rule::WHITESPACE => {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
                println!("Text:    {}", pair.as_str());
                unreachable!()
            }
        }
    }

    let mut file = CK3Parser::parse(Rule::file, &ck3txt).expect("unsuccessful parse");
    let first = file.next().unwrap();
    let main = match first.as_rule() {
        Rule::save_id => file.next().unwrap(),
        _ => first
    };
    let body = JSONValue::Object(main.into_inner().map(|pair| parse_pair(pair)).collect());

    Ok(body)
}
