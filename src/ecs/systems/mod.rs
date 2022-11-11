use specs::prelude::*;
use specs::shrev::EventChannel;

use super::event_channels::*;

pub mod create_map;
pub use create_map::CreateMap as CreateMapSystem;
pub mod create_player;
pub use create_player::CreatePlayer as CreatePlayerSystem;
pub mod experiment;
pub use experiment::Experiment as ExperimentSystem;
pub mod action_processor;
pub use action_processor::ActionProcessor as ActionProcessorSystem;
pub mod command_processor;
pub use command_processor::CommandProcessor as CommandProcessorSystem;
pub mod effect_processor;
pub use effect_processor::EffectProcessor as EffectProcessorSystem;
pub mod initiative_dispenser;
pub use initiative_dispenser::InitiativeDispenser as InitiativeDispenserSystem;
pub mod input_processor;
pub use input_processor::InputProcessor as InputProcessorSystem;
pub mod output_processor;
pub use output_processor::OutputProcessor as OutputProcessorSystem;
pub mod tick;
pub use tick::Tick as TickSystem;

pub fn run_initial_systems(ecs: &mut World) {
  (CreatePlayerSystem {}).run_now(ecs);
  (CreateMapSystem {}).run_now(ecs);
}

pub fn get_tick_dispatcher(ecs: &mut World) -> Dispatcher<'static, 'static> {
  let output_processor_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<OutputEvent>>().register_reader();
    OutputProcessorSystem { reader_id }
  };
  let input_processor_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<InputEvent>>().register_reader();
    InputProcessorSystem { reader_id }
  };
  let command_processor_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<CommandEvent>>().register_reader();
    CommandProcessorSystem { reader_id }
  };
  let action_processor_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<ActionEvent>>().register_reader();
    ActionProcessorSystem { reader_id }
  };
  let effect_processor_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<EffectEvent>>().register_reader();
    EffectProcessorSystem { reader_id }
  };
  let experiment_system = ExperimentSystem {};
  let initiative_dispenser_system = InitiativeDispenserSystem {};
  let tick_system = TickSystem {};
  let dispatcher = DispatcherBuilder::new()
    .with(experiment_system, "experiment", &[])
    .with(initiative_dispenser_system, "initiative_dispenser", &[])
    .with(input_processor_system, "input_processor", &[])
    .with(command_processor_system, "command_processor", &[])
    .with(action_processor_system, "action_processor", &[])
    .with(effect_processor_system, "effect_processor", &[])
    .with(output_processor_system, "output_processor", &[])
    .with(tick_system, "tick", &[])
    .build();
  dispatcher
}

/// Every ten ticks.
pub fn get_deca_tick_dispatcher(_ecs: &mut World) -> Dispatcher<'static, 'static> {
  let dispatcher = DispatcherBuilder::new().build();
  dispatcher
}

/// Every hundred ticks.
pub fn get_hecto_tick_dispatcher(_ecs: &mut World) -> Dispatcher<'static, 'static> {
  let dispatcher = DispatcherBuilder::new().build();
  dispatcher
}

/// Every thousand ticks.
pub fn get_kilo_tick_dispatcher(_ecs: &mut World) -> Dispatcher<'static, 'static> {
  let dispatcher = DispatcherBuilder::new().build();
  dispatcher
}
