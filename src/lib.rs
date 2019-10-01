extern crate sqlite;
//use sqlite::{Connection, OpenFlags, State, Type, Value};
#[no_mangle]
pub extern fn echo(echo: &str) -> &str {
    echo
}

#[no_mangle]
pub extern fn bsqlite(){
    let connection = sqlite::open(":memory:").unwrap();
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER);
            INSERT INTO users VALUES ('Alice', 42);
            INSERT INTO users VALUES ('Bob', 69);
            ",
        )
        .unwrap();
    connection
        .iterate("SELECT * FROM users WHERE age > 50", |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{} = {}", column, value.unwrap());
            }
            true
        })
        .unwrap();
}

#[allow(dead_code)]
fn main() {
    unimplemented!();
}