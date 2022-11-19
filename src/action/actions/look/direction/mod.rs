use crate::ecs::entity::EntityId;
use crate::ecs::system::action_processor::Data as ActionProcessorData;
use crate::effect::*;
use crate::map::Direction;
use crate::map::PassageDestination;
use anyhow::Error;

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
          PassageDestination::Room(_) => {
            write_effect_event!(
              data,
              Effect::EntityLooksDirection(EntityLooksDirection {
                entity_id: self.entity_id,
                direction: self.direction,
              })
            );
            write_effect_event!(
              data,
              Effect::EntitySetInitiative(EntitySetInitiative {
                entity_id: self.entity_id,
                value: 0,
              })
            );
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
