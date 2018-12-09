use std::error::Error;

use clap::{load_yaml, App};

mod claim;
mod fabric;

mod loader;

use crate::claim::Claim;
use crate::fabric::Fabric;

const FABRIC_SIZE: usize = 2000;

fn apply_claims<'a>(claims: impl Iterator<Item = &'a Claim>) -> Fabric {
    let mut fabric = Fabric::new(FABRIC_SIZE);

    for claim in claims {
        fabric.process_claim(claim);
    }

    fabric
}

fn main() -> Result<(), Box<Error>> {
    let yaml = load_yaml!("args.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let claims = loader::from_file(matches.value_of("input").unwrap())?;
    let fabric = apply_claims(claims.iter());

    match matches.value_of("part").unwrap() {
        "one" => {
            println!("Number of elven conflicts: {}", fabric.count_conflicts());
        }
        "two" => println!(
            "Valid claims are: {:?}",
            fabric.valid_claims().collect::<Vec<_>>()
        ),
        _ => unreachable!(),
    };

    Ok(())
}
