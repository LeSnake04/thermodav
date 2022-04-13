use crate::config::{Config, ConfigReader};

mod config;

pub const CONFIG_PATH: &'static str = "./thermodav.toml";

fn main() {
	let config: Config = ConfigReader::new(CONFIG_PATH)
		.read_or_create_config()
		.unwrap();
	println!("Note: To reset the configuration delete or rename the file thermodav.toml");
}
