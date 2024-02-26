use crate::error::{Result};
use std::collections::BTreeMap;
use std::mem::replace;

pub enum Statement {
    CreateTable {
        name: String,
        columns: Vec<Column>,
    },
    DropTable(String),
    Delete {
        table: String,
        r#where: Option<Expression>,
    },
    Insert {
        table: String,
        columns: Option<Vec<String>>,
        values: Vec<Vec<Expression>>,
    },
    Update {
        table: String,
        rer: BTreeMap<String, Expression>,
        r#where: Option<Expression>,
    },
    Select {
        select: Vec<(Expression, Option<String>)>,
        from: Vec<FromItem>,
        r#where: Option<Expression>,
        group_by: Vec<Expression>,
        having: Option<Expression>,
        order: Vec<(Expression, Order)>,
        offset: Option<Expression>,
        limit: Option<Expression>,
    },
}

pub struct Column {
    name: String,
    datatype: DataType,
    primary_key: bool,
    nullable: Option<bool>,
    default: Option<Expression>,
    unique: bool,
    index: bool,
    references: Option<String>,
}

pub enum DataType {
    Boolean,
    Integer,
    Float,
    String,
}

pub enum Expression {
    Field(Option<String>, String),
    Column(usize),
    Literal(Literal),
    Function(String, Vec<Expression>),
    Operation(Operation),
}

pub enum Literal {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

pub enum Operation {}

pub enum Order {
    Asc,
    Desc,
}

pub enum FromItem {
    Table {
        name: String,
        alias: Option<String>,
    }
}


impl Expression {}