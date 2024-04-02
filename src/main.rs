use config::Config;

mod config;
mod error;

fn main() {
    match Config::load_config() {
        Ok(config) => println!("{:?}", config),
        Err(e) => eprintln!("{}", e),
    }
}
