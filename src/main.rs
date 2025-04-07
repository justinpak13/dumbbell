use chrono::prelude::Local;

mod model;

fn main() {
    let local = Local::now();

    println!("date: {:?}", local.date_naive());
    println!("time: {:?}", local.time());
}
