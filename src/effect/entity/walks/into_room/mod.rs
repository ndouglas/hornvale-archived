use crate::ecs::entity::EntityId;
use crate::ecs::entity::RoomId;
use crate::ecs::systems::process_effect::ProcessEffectData;
use crate::map::Direction;

/// `EntityWalksIntoRoom`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntoRoom {
  /// The entity performing the action.
  pub entity_id: EntityId,
  /// The room the entity enters.
  pub room_id: RoomId,
  /// The direction from which the entity enters.
  pub direction: Direction,
}

impl IntoRoom {
  pub fn process(&self, data: &mut ProcessEffectData) {
    let entity = get_entity!(data, self.entity_id);
    let room = get_entity!(data, self.room_id);
    let name = get_name!(data, entity).unwrap();
    write_output_3rd!(
      data,
      entity,
      room,
      format!("{} walks in from the {}.", name, self.direction.get_lowercase())
    );
  }
}