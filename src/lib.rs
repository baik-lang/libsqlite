extern crate sqlite;
extern crate libc;
//use sqlite::{Connection, OpenFlags, State, Type, Value};
use libc::{c_char};
use std::ffi::CStr;

#[no_mangle]
pub extern fn koneksi(koneksi: &str) -> &str{
    koneksi
}

#[no_mangle]
pub extern fn execute(koneksi: *const c_char, query: *const c_char) {
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
    }
    else{
        connection
        .iterate(query.to_str().unwrap(), |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{} = {}", column, value.unwrap());
            }
            true
        })
        .unwrap();
    }
}

#[allow(dead_code)]
fn main() {
    unimplemented!();
}