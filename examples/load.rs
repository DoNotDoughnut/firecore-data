use std::ops::Deref;

use firecore_data::configuration::Configuration;

fn main() {

    macroquad::Window::new("Window", start())

}

async fn start() {

    println!("Attempting to load configuration!");

    firecore_data::store().await;

    println!("Loaded configuration!");

    println!("{:?}", firecore_data::get::<Configuration>().unwrap().deref());

}