use bf::BloomFilter;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Bloom Filter CLI")
        .version("1.0")
        .author("Author Name <author@example.com>")
        .about("Demonstrates a basic Bloom Filter")
        .arg(Arg::new("add")
            .short('a')
            .long("add")
            .value_name("VALUE")
            .help("Adds an item to the Bloom Filter"))
        .arg(Arg::new("check")
            .short('c')
            .long("check")
            .value_name("VALUE")
            .help("Checks if an item is in the Bloom Filter"))
        .get_matches();

    let mut bloom_filter = BloomFilter::new(10000, 3);

    if let Some(item) = matches.get_one::<String>("add") {
        bloom_filter.add(item);
        println!("Value '{}' added to the Bloom Filter.", item);
    }

    if let Some(item) = matches.get_one::<String>("check") {
        if bloom_filter.contains(item) {
            println!("Value '{}' is probably in the Bloom Filter.", item);
        } else {
            println!("Value '{}' is definitely not in the Bloom Filter.", item);
        }
    }
}