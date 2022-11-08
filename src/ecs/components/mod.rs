use specs::prelude::*;

pub mod has_description;
pub use has_description::HasDescription;
pub mod has_name;
pub use has_name::HasName;
pub mod has_passages;
pub use has_passages::HasPassages;
pub mod is_a_player;
pub use is_a_player::IsAPlayer;
pub mod is_a_room;
pub use is_a_room::IsARoom;
pub mod is_a_spawn_room;
pub use is_a_spawn_room::IsASpawnRoom;
pub mod is_an_object;
pub use is_an_object::IsAnObject;
pub mod is_in_room;
pub use is_in_room::IsInRoom;

pub fn register_components(ecs: &mut World) {
  ecs.register::<HasDescription>();
  ecs.register::<HasName>();
  ecs.register::<HasPassages>();
  ecs.register::<IsAPlayer>();
  ecs.register::<IsARoom>();
  ecs.register::<IsASpawnRoom>();
  ecs.register::<IsAnObject>();
  ecs.register::<IsInRoom>();
}
