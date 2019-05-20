#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::fs::File;

#[get("/")]
fn index() -> File{
    // "Hello, world!"
    File::open("src/index.html").expect("File not  Found")
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}