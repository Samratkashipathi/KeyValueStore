#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "CLI", about = "Trying CLI.")]
enum Command {
    #[structopt(name = "get")]
    Get { key: String },

    #[structopt(name = "set")]
    Set { key: String, value: String },

    #[structopt(name = "remove")]
    Remove { key: String },
}

fn main() {
    let cmd = Command::from_args();
    println!("{:?}", cmd);

    match cmd {
        Command::Get { key } => {
            println!("Get {:#?}", key);
        }
        Command::Set { key, value } => {
            println!("Set {:#?} {:#?}", key, value);
        }
        Command::Remove { key } => {
            println!("Remove {:#?}", key);
        }
    }
}
