extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pairs;
use pest::inputs::StrInput;

#[derive(Parser)]
#[grammar = "ddl.pest"]
struct DdlParser;

#[derive(Debug)]
enum SqlType {
    Int,
    Varchar,
    Short,
    Double,
}

#[derive(Debug)]
struct ColumnConfig {
    name: String,
    typ: SqlType,
}

#[derive(Debug)]
struct TableConfig {
    temp: bool,
    if_not_exists: bool,
    name: String,
    schema: String,
    columns: Vec<ColumnConfig>,
}

impl TableConfig {
    fn new() -> Self {
        TableConfig {
            temp: false,
            if_not_exists: false,
            name: String::new(),
            schema: String::new(),
            columns: Vec::new(),
        }
    }
}

fn parse_full_table_name(pairs: Pairs<Rule, StrInput>) -> (String, String) {
    let mut schema = String::new();
    let mut table = String::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::schema_name => schema = pair.clone().into_span().as_str().to_owned(),
            Rule::table_name => table = pair.clone().into_span().as_str().to_owned(),
            _ => unreachable!(),
        }
    }

    (schema, table)
}

fn parse_table_name(pairs: Pairs<Rule, StrInput>) -> (String, String) {
    let mut schema = String::new();
    let mut table = String::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::full_table_name => {
                let (s, t) = parse_full_table_name(pair.into_inner());
                schema = s;
                table = t;
            } 
            Rule::table_name => table = pair.clone().into_span().as_str().to_owned(),
            _ => unreachable!(),
        }
    }

    (schema, table)
}

fn parse_column_type(typ: &str) -> SqlType {
    match typ.to_lowercase().as_str() {
        "int" => SqlType::Int,
        "varchar" => SqlType::Varchar,
        "double" => SqlType::Double,
        "short" => SqlType::Short,
        _ => unreachable!(),
    }
}

fn parse_column_def(pairs: Pairs<Rule, StrInput>) -> ColumnConfig {
    let mut cfg = ColumnConfig {
        name: String::new(),
        typ: SqlType::Int,
    };

    for pair in pairs {
        match pair.as_rule() {
            Rule::column_name => cfg.name = pair.clone().into_span().as_str().to_owned(),
            Rule::column_type => cfg.typ = parse_column_type(pair.clone().into_span().as_str()),
            _ => unreachable!(),
        }
    }

    cfg
}

fn parse_columns_def(pairs: Pairs<Rule, StrInput>) -> Vec<ColumnConfig> {
    pairs.map(|pair| parse_column_def(pair.into_inner())).collect()
}

fn parse_create_table_statement(pairs: Pairs<Rule, StrInput>) -> TableConfig {
    let mut cfg = TableConfig::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::temp_mod => cfg.temp = true,
            Rule::if_not_exists_mod => cfg.if_not_exists = true,
            Rule::table_ident => {
                let (schema, table) = parse_table_name(pair.into_inner());
                cfg.schema = schema;
                cfg.name = table;
            }
            Rule::columns_def => cfg.columns = parse_columns_def(pair.into_inner()),
            _ => unreachable!("{:?}", pair),
        };
    }

    cfg
}

fn main() {
    let test_str = "CREATE temp TABLE some.table (id INT, name VARCHAR)";
    let mut pairs = DdlParser::parse_str(Rule::create_table_statement, test_str)
        .unwrap_or_else(|e| panic!("{}", e));

    let cfg = parse_create_table_statement(pairs.next().unwrap().into_inner());

    println!("{:?}", cfg);
}