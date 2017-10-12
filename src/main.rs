extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pairs;
use pest::inputs::StrInput;

#[derive(Parser)]
#[grammar = "ddl.pest"]
struct DdlParser;

fn print_pairs(pairs: Pairs<Rule, StrInput>) {
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.clone().into_span());
        println!("Text:    {}", pair.clone().into_span().as_str());

        println!("{{");
        print_pairs(pair.into_inner());
        println!("}}");
    }
}

fn main() {
    let test_str = "CREATE TABLE some.table (id INT, name VARCHAR)";
    let pairs = DdlParser::parse_str(Rule::create_table_statement, test_str)
        .unwrap_or_else(|e| panic!("{}", e));

    print_pairs(pairs);
}