//! 入口函数
//! 接受 sql_str 返回 Result
use crate::parser::{parse_sql, Ast};
use crate::executor::{parse_ast, Plan};
use crate::storage::{execute_plan, Result};

pub fn execute_sql(sql: &str) -> Result {
    let ast: Ast = parse_sql(sql);
    let plan: Plan = parse_ast(ast);
    let result: Result = execute_plan(plan);
    result
}