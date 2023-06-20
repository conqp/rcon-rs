use clap::Parser;
use rcon_rs::rcon;

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

    match rcon(
        args.host.unwrap_or("localhost:25566".to_string()).as_str(),
        args.passwd.unwrap_or("".to_string()).as_str(),
        args.command
            .iter()
            .map(|string| string.as_str())
            .collect::<Vec<_>>()
            .as_slice(),
        None,
    ) {
        Ok(response) => println!("Server replied: {}", response),
        Err(error) => eprint!("Error: {}", error),
    }
}
