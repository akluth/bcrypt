extern crate clap;
extern crate bcrypt;

use clap::{Arg, App};
use bcrypt::{DEFAULT_COST, hash, verify};

fn main() {
    let matches = App::new("bcrypt")
                          .version("0.1")
                          .author("Alexander Kluth <deralex@cpan.org>")
                          .about("Create bcrypt hash from input string")
                          .arg(Arg::with_name("STRING")
                               .help("String to hash")
                               .required(true)
                               .index(1))
                          .get_matches();

    match hash(matches.value_of("STRING").unwrap(), DEFAULT_COST) {
        Ok(hashed) => println!("{}", hashed),
        Err(e) => println!("{}", e)
    }
}
