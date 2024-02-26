mod ast;
mod lexer;

use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::{Parser, ParserError};

pub struct Ast;
pub fn parse_sql(sql: &str) -> Ast {
    let dialect = GenericDialect {};
    Parser::parse_sql(&dialect, sql).expect("parse_sql fail!");
    Ast
}
