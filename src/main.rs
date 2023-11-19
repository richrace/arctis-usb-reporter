use clap::Parser;

use crate::headphone_helpers::search_for_headphones::find_headphones;

mod headphone_helpers;
mod known_headphone;
mod report;

use hidapi::HidApi;
use report::report_headphones;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    probe: bool,
}

fn main() {
    let api = HidApi::new().expect("Hidapi init failed");

    let args = Args::parse();

    if args.probe {
        find_headphones(&api).for_each(|headphone| {
            println!("Device Name: {:?}", headphone.name);
            println!("\tProduct ID: {}", headphone.product_id);
            println!("\tPath: {}", headphone.path);
            println!("\tUsage Page: {}", headphone.usage_page);
            println!("\tUsage: {}", headphone.usage);
            println!("\tInterface: {}", headphone.interface);
        });
    } else {
        report_headphones(&api);
    }
}
