#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::fs::File;

use rust_gpiozero::*;

#[get("/")]
fn index() -> File{

    let mut led =  LED::new(17);
    led.blink(2.0,3.0);

    led.wait();
    
    "Hello, world!"
    // File::open("src/index.html").expect("File not  Found")
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}