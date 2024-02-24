#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    columns: Vec<String>,
    // 要选择的列名
    table: String,         // 从哪个表中选择
}

use nom::{
    IResult,
    sequence::{tuple, preceded},
    character::complete::{alpha1, multispace1, char, multispace0},
    multi::separated_list1,
    combinator::map,
    bytes::complete::tag,
};

use std::str;

fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(input)
}

fn parse_column_list(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(char(','), preceded(multispace0, parse_identifier))(input)
}

fn parse_select_statement(input: &str) -> IResult<&str, SelectStatement> {
    map(
        tuple((
            preceded(tag("SELECT"), preceded(multispace1, parse_column_list)),
            preceded(preceded(multispace1, tag("FROM")), preceded(multispace1, parse_identifier)),
        )),
        |(columns, table)| SelectStatement { columns, table },
    )(input)
}

fn main() {
    let sql = "SELECT columna, columnb FROM table";
    match parse_select_statement(sql) {
        Ok((remaining, ast)) => {
            println!("AST: {:?}, Remaining: {}", ast, remaining);
        },
        Err(error) => println!("Error: {:?}", error),
    }
}
