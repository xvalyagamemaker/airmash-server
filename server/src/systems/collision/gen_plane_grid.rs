use specs::*;

use types::collision::*;
use types::systemdata::IsAlive;
use types::*;
use SystemInfo;

use component::collision::PlaneGrid;
use component::flag::IsPlayer;

use consts::config::PLANE_HIT_CIRCLES;

#[derive(Default)]
pub struct GenPlaneGrid;

#[derive(SystemData)]
pub struct GenPlaneGridData<'a> {
	grid: Write<'a, PlaneGrid>,

	ent: Entities<'a>,
	pos: ReadStorage<'a, Position>,
	rot: ReadStorage<'a, Rotation>,
	team: ReadStorage<'a, Team>,
	plane: ReadStorage<'a, Plane>,

	is_player: ReadStorage<'a, IsPlayer>,
	is_alive: IsAlive<'a>,
}

impl<'a> System<'a> for GenPlaneGrid {
	type SystemData = GenPlaneGridData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let it = (
			&*data.ent,
			&data.pos,
			&data.rot,
			&data.team,
			&data.plane,
			data.is_player.mask() & data.is_alive.mask(),
		)
			.join()
			.map(|(ent, &pos, &rot, &team, &plane, ..)| {
				PLANE_HIT_CIRCLES[&plane].iter().map(move |hc| {
					let offset = hc.offset.rotate(rot);

					HitCircle {
						pos: pos + offset,
						rad: hc.radius,
						layer: team.0,
						ent: ent,
					}
				})
			})
			.flatten();

		data.grid.0.rebuild_from(it);
	}
}

use systems::PositionUpdate;

impl SystemInfo for GenPlaneGrid {
	type Dependencies = PositionUpdate;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
