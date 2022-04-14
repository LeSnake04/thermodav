use crate::caldav::CalDav;
pub use serde::Deserialize;
use std::env::args;
use std::thread::{sleep, spawn};
use std::time::Duration;

use crate::config::{Config, ConfigReader};

mod caldav;
mod config;

const FORCE_OVERRIDE_CONFIG: bool = false;
const LOOPING: bool = false;

fn main() {
	loop {
		let args: Vec<String> = args().collect();
		//
		let config_path: &str = if args.len() > 1 {
			&args[1]
		} else {
			"./thermodav.toml"
		};

		let config: Config = if FORCE_OVERRIDE_CONFIG {
			ConfigReader::new(config_path).create_config_from_example()
		} else {
			match ConfigReader::new(config_path).read_or_create_config() {
				Ok(c) => c,
				Err(_) => return,
			}
		};
		let _ = spawn(move || {
			println!("Note: To reset the configuration delete or rename the file 'thermodav.toml'");
			let caldav: &CalDav = CalDav::init(config.dav).list_events("thermodav_test");
		})
		.join();

		if !LOOPING {
			break;
		}
		println!("Applied, waiting {} minutes", config.delay);
		sleep(Duration::from_secs(config.delay as u64 * 100));
	}
}
