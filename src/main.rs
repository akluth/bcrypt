extern crate clap;
extern crate bcrypt;

use clap::{Arg, App};
use bcrypt::{DEFAULT_COST, hash};

fn main() {
    let matches = App::new("bcrypt")
                          .version("0.1")
                          .author("Alexander Kluth <deralex@cpan.org>")
                          .about("Create bcrypt hash from input string")
                          .arg(Arg::with_name("STRING")
                               .help("String to hash")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("cost")
                               .short("c")
                               .long("cost")
                               .value_name("VALUE")
                               .help("Sets the cost to use (between 4 and 31, defaults to 12)")
                               .takes_value(true))
                          .get_matches();

    let cost;

    match matches.value_of("cost") {
        Some(c) => cost = c.parse::<u32>().unwrap(),
        None => cost = DEFAULT_COST
    }

    match hash(matches.value_of("STRING").unwrap(), cost) {
        Ok(hashed) => println!("{}", hashed),
        Err(e) => println!("{}", e)
    }
}
