use minicaldav::{get_calendars, get_events, Calendar, Event};
use ureq::Agent;
use url::Url;

use crate::Deserialize;

pub struct CalDav {
	agent: Agent,
	config: CalDavConfig,
	url: Url,
	calendars: Vec<Calendar>,
	//calendar: Option<Calendar>,
}

#[derive(Deserialize, Clone)]
pub struct CalDavConfig {
	pub server: String,
	pub user: String,
	pub password: String,
}

impl CalDav {
	pub fn init(config: CalDavConfig) -> Self {
		let agent: Agent = Agent::new();
		let url: Url = Url::parse(&config.server).expect("Failed to parse CalDav Server URL");
		Self {
			calendars: get_calendars(agent.clone(), &config.user, &config.password, &url)
				.expect("Failed to get Calendars"),
			agent,
			config,
			url,
		}
	}

	pub fn list_calenders(&self) -> &Self {
		let calendars: &Vec<Calendar> = &self.calendars;
		println!("Calendars:");
		for i in calendars {
			println!("{}", i.name());
		}
		self
	}

	pub fn list_events<S: Into<String> + Copy>(&self, calendar: S) -> &Self {
		fn get_calendar<'a>(calendars: &'a Vec<Calendar>, term: &'a str) -> Option<&'a Calendar> {
			for calendar in calendars {
				if calendar.name() == term {
					return Some(calendar);
				}
			}
			return None;
		}
		let calendars: &Vec<Calendar> = &self.calendars;
		let calendar_name: String = calendar.into();
		let calendar: &Calendar = get_calendar(&calendars, &calendar_name).expect(&format!(
			"Couldn't find calendar with name '{}'",
			&calendar_name
		));

		let events: Vec<Event> = get_events(
			self.agent.clone(),
			&self.config.user,
			&self.config.password,
			calendar,
		)
		.expect("Failed to get events");
		println!("Events");
		for event in events {
			println!("{:?}", event.properties());
		}
		self
	}
}
