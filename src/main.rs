use clap::Parser;
use rcon_rs::Client;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    host: Option<String>,

    #[arg(long)]
    passwd: Option<String>,

    #[arg(index = 1)]
    command: Vec<String>,
}

fn main() {
    let args = Args::parse();

    match Client::from_str(args.host.as_deref().unwrap_or("localhost:25566")) {
        Ok(mut client) => match client.login(args.passwd.as_deref().unwrap_or("")) {
            Ok(success) => {
                if success {
                    match client.exec(
                        args.command
                            .iter()
                            .map(|string| string.as_str())
                            .collect::<Vec<_>>()
                            .as_slice(),
                    ) {
                        Ok(response) => println!("{}", response),
                        Err(error) => eprintln!("{}", error),
                    }
                } else {
                    eprintln!("Login failed.");
                }
            }
            Err(error) => eprintln!("{}", error),
        },
        Err(error) => eprintln!("{}", error),
    }
}
