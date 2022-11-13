use crate::entity::EntityId;
use crate::map::Direction;
use crate::map::PassageDestination;
use crate::system::action_processor::Data as ActionProcessorData;
use anyhow::Error;
use specs::prelude::*;

/// The `LookDirection` action.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookDirection {
  pub entity_id: EntityId,
  pub direction: Direction,
}

impl LookDirection {
  pub fn execute(&self, data: &mut ActionProcessorData) -> Result<(), Error> {
    let entity = get_entity!(data, self.entity_id);
    if let Some(room_id) = get_current_room_id!(data, entity) {
      let room = get_entity!(data, room_id);
      match get_passage_to!(data, room, &self.direction) {
        Some(passage) => match passage.to {
          PassageDestination::Room(destination_id) => {
            if entity_id_has_camera!(data, self.entity_id) {
              info!("Sending event (description of indicated room).");
              you!(
                data,
                entity,
                format!("look to the {}...", &self.direction.get_lowercase())
              );
              let destination_room = get_entity!(data, destination_id);
              write_output_event!(data, format_room!(data, destination_room));
            }
          },
          _ => {
            you!(data, entity, "are unable to look in that direction!");
          },
        },
        None => {
          you!(data, entity, "are unable to look in that direction!");
        },
      }
    }
    Ok(())
  }
}
