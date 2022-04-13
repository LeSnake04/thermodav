use serde::Deserialize;
use std::fs::{read_to_string, write};
use std::path::Path;
use toml::from_str as toml_str;

const EXAMPLE_CONFIG: &'static str = include_str!("../thermodav.example.toml");

pub struct ConfigReader {
	path: String,
}

/// # Configuration of thermodav
/// ## loading  the configuration
///
#[derive(Deserialize)]
pub struct Config {
	pub delay: u16,
	pub dav: DavConfig,
	pub mqtt: MqttConfig,
	pub thermo: Vec<ThermoConfig>,
}

#[derive(Deserialize)]
pub struct DavConfig {
	pub server: String,
	pub user: String,
	pub password: String,
}

#[derive(Deserialize)]
pub struct MqttConfig {
	pub server: String,
}

#[derive(Deserialize)]
pub struct ThermoConfig {
	pub topic: String,
	pub calname: String,
}
impl ConfigReader {
	pub fn new<S: Into<String>>(path: S) -> Self {
		Self { path: path.into() }
	}
	pub fn read_or_create_config(&self) -> Result<Config, ()> {
		// Create a config file if it doesn't already exist
		if !Path::new(&self.path).exists() {
			println!("Config does not exist!, Creating example config ...");
			self.create_config_from_example();
			println!("Please fill out the config file and run again");
			Err(())
		} else {
			Ok(self.read_config())
		}
	}
	pub fn read_config(&self) -> Config {
		/*&fs::read_to_string(CONFIG_PATH)
		.expect("Couldn't read config file")
		.parse::<Config<S>>()
		.expect("Failed to parse config:")*/
		toml_str(&read_to_string(&self.path).unwrap()).unwrap()
	}

	pub fn create_config_from_example(&self) {
		write(&self.path, EXAMPLE_CONFIG).expect("Failed to write example config")
	}
}
