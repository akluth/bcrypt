use bcrypt::{DEFAULT_COST, hash};
use clap::{Arg, Command, value_parser};

fn cli() -> Command {
    Command::new("bcrypt")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Alexander Kluth <alex@clooze.io>")
        .about("Create bcrypt hash from input string")
        .arg(
            Arg::new("STRING")
                .help("String to hash")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("cost")
                .short('c')
                .long("cost")
                .value_name("VALUE")
                .help("Sets the cost to use")
                .value_parser(value_parser!(u32).range(4..=31)),
        )
}

fn main() -> Result<(), bcrypt::BcryptError> {
    let matches = cli().get_matches();
    let cost = matches
        .get_one::<u32>("cost")
        .copied()
        .unwrap_or(DEFAULT_COST);
    let input = matches
        .get_one::<String>("STRING")
        .expect("STRING is required by clap");

    let hashed = hash(input, cost)?;
    println!("{hashed}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::cli;

    #[test]
    fn accepts_input_without_cost() {
        let matches = cli().try_get_matches_from(["bcrypt", "secret"]).unwrap();

        assert_eq!(
            matches.get_one::<String>("STRING").map(String::as_str),
            Some("secret")
        );
        assert_eq!(matches.get_one::<u32>("cost").copied(), None);
    }

    #[test]
    fn accepts_valid_cost() {
        let matches = cli()
            .try_get_matches_from(["bcrypt", "--cost", "4", "secret"])
            .unwrap();

        assert_eq!(matches.get_one::<u32>("cost").copied(), Some(4));
    }

    #[test]
    fn rejects_cost_outside_bcrypt_range() {
        assert!(
            cli()
                .try_get_matches_from(["bcrypt", "--cost", "3", "secret"])
                .is_err()
        );
    }
}
