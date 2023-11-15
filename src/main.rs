use crate::headphone_helpers::search_for_headphones::find_headphones;

mod headphone_helpers;

use hidapi::HidApi;

fn main() {
    let api = HidApi::new().expect("Hidapi init failed");

    find_headphones(&api).for_each(|headphone| {
        println!("Device Name: {:?}", headphone.name);
        println!("\tProduct ID: {}", headphone.product_id);
        println!("\tPath: {}", headphone.path);
        println!("\tUsage Page: {}", headphone.usage_page);
        println!("\tUsage: {}", headphone.usage);
        println!("\tInterface: {}", headphone.interface);
    })
}
