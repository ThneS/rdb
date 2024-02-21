mod interface;

use interface::execute_sql;

fn main() {
    execute_sql("select name, age from rdb").unwrap()
}