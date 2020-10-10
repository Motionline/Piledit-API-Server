#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate opencv;

use opencv::{
    core,
    highgui,
    imgproc,
    prelude::*,
    videoio,
};

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

#[get("/grayscale/<path>")]
fn grayscale(path: String) {
    let pictures_path = "/Users/momiyama/Pictures";
    let be_copied_image_path = format!("{}/icon.png", pictures_path);
    let new_image_path = format!("{}/{}.png", pictures_path, path);
    let img = opencv::imgcodecs::imread(&be_copied_image_path, opencv::imgcodecs::IMREAD_UNCHANGED).unwrap();
    let _ = opencv::imgcodecs::imwrite(&new_image_path, &img, &opencv::core::Vector::new());
}

#[get("/json_parse")]
fn json_parse() -> &'static str {
    // jsonを受け取る / 処理
    "Json Parse"
}

fn main() {
    println!("Hello, world!");
    rocket::ignite().mount("/", routes![index, json_parse, grayscale]).launch();
}