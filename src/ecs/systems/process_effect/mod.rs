use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use super::super::components::*;
use super::super::event_channels::*;
use super::super::resources::*;
use crate::effect::*;

pub struct ProcessEffect {
  pub reader_id: ReaderId<EffectEvent>,
}

impl ProcessEffect {}

#[derive(SystemData)]
pub struct ProcessEffectData<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Read<'a, PlayerResource>,
  pub camera_resource: Read<'a, CameraResource>,
  pub tile_map_resource: Write<'a, TileMapResource>,
  pub effect_event_channel: Write<'a, EventChannel<EffectEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
  pub has_description: ReadStorage<'a, HasDescription>,
  pub has_name: ReadStorage<'a, HasName>,
  pub has_passages: ReadStorage<'a, HasPassages>,
  pub is_a_player: ReadStorage<'a, IsAPlayer>,
  pub is_an_actor: ReadStorage<'a, IsAnActor>,
  pub is_an_object: ReadStorage<'a, IsAnObject>,
  pub is_in_room: WriteStorage<'a, IsInRoom>,
}

impl<'a> System<'a> for ProcessEffect {
  type SystemData = ProcessEffectData<'a>;

  /// Run the system.
  fn run(&mut self, mut data: Self::SystemData) {
    let events = data
      .effect_event_channel
      .read(&mut self.reader_id)
      .into_iter()
      .cloned()
      .collect::<Vec<EffectEvent>>();
    let event_count = events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} effect event(s)...", event_count);
    for event in events.iter() {
      debug!("Processing next effect event, {:?}", event);
      let EffectEvent { effect } = event;
      use Effect::*;
      match effect {
        EntityWalksIntoRoom(object) => object.process(&mut data),
        EntityWalksOutOfRoom(object) => object.process(&mut data),
      }
    }
  }
}
