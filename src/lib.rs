extern crate sqlite;
extern crate libc;
extern crate serde_json;
//use sqlite::{Connection, OpenFlags, State, Type, Value};
use libc::{c_char};
use std::ffi::CStr;
use serde_json::{Value, Map};

#[no_mangle]
pub extern fn koneksi(koneksi: &str) -> &str{
    koneksi
}

#[no_mangle]
pub extern fn execute(koneksi: *const c_char, query: *const c_char) ->  &'static str{

    let koneksi = unsafe {
        assert!(!koneksi.is_null());
        CStr::from_ptr(koneksi)
    };
    let query = unsafe {
        assert!(!query.is_null());
        CStr::from_ptr(query)
    };

    let connection = sqlite::open(koneksi.to_str().unwrap()).unwrap();

    let ddl = ["create","insert", "drop"];
    let query_first: Vec<&str> = query.to_str().unwrap().split(" ").collect();
    if ddl.contains(&query_first[0].to_lowercase().as_str()){
        connection
        .execute(
            query.to_str().unwrap(),
        )
        .unwrap();
        return "1"
    }
    else{
        let mut vec = Vec::new();
        connection
        .iterate(query.to_str().unwrap(), |pairs| {
            let mut inner_map = Map::new();
            for &(column, value) in pairs.iter() {
                inner_map.insert(column.to_string(), Value::String(value.unwrap().to_string()));
            }
            vec.push(inner_map);
            true
        })
        .unwrap();
        let result = serde_json::to_string(&vec).unwrap();
        return Box::leak(result.into_boxed_str());
    }
}

#[allow(dead_code)]
fn main() {
    unimplemented!();
}