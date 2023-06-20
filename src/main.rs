use rcon_rs::rcon;

fn main() {
    match rcon(
        "srv.richard-neumann.de:5000",
        "ZXYRdwrXkLD38hGt",
        &["list"],
        None,
    ) {
        Ok(response) => println!("Server replied: {}", response),
        Err(error) => eprint!("Error: {}", error),
    }
}
