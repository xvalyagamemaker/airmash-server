use specs::*;

use types::*;

use SystemInfo;

use systems::handlers::packet::LoginHandler;

use component::counter::*;
use component::event::*;
use utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct InitEarnings;

#[derive(SystemData)]
pub struct InitEarningsData<'a> {
	pub earnings: WriteStorage<'a, Earnings>,
}

impl EventHandlerTypeProvider for InitEarnings {
	type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for InitEarnings {
	type SystemData = InitEarningsData<'a>;

	fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
		data.earnings.insert(evt.id, Earnings(Score(0))).unwrap();
	}
}

impl SystemInfo for InitEarnings {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
