use crate::interface::execute_sql;

mod interface;
mod network; // binary data -> sql
mod parser; // sql -> ast
mod executor; // ast -> plan
mod storage;
mod error;  // plan -> result


fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
#[test]
fn insert() {
    use interface::execute_sql;
    use storage::Result;
    let sql = "select name, age from user where agd > 10;";
    let res = execute_sql(sql);
}