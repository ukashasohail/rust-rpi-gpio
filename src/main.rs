#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::fs::File;
use rocket::response::Redirect;

// use rust_gpiozero::*;

#[get("/")]
fn index() -> File{

    // let mut led =  LED::new(17);
    // led.blink(2.0,3.0);

    // led.wait();
    
    // "Hello, world!"
    File::open("src/index.html").expect("File not  Found")
}

#[get("/lighton")]
fn lighton()-> Redirect{
        let mut led =  LED::new(17);
        led.off();
        Redirect::to("/") //this will redirect to root

}

#[get("/lightoff")]
fn lightoff()-> Redirect{
        let mut led =  LED::new(17);
        led.on();
        Redirect::to("/")


}


fn main() {
    rocket::ignite().mount("/", routes![index,lighton,lightoff]).launch();
  
}