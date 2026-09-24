use std::time::Duration;

use ureq::{Agent, config::Config};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

pub fn agent(global_timeout: Duration) -> Agent {
	let config = Config::builder().timeout_connect(Some(CONNECT_TIMEOUT)).timeout_global(Some(global_timeout)).build();
	Agent::new_with_config(config)
}
