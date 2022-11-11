#[macro_export]
macro_rules! create_player {
  ($system_data: expr) => {{
    use $crate::ecs::components::*;
    use $crate::initiative::Initiative;
    let player = $system_data.entities.create();
    $system_data
      .is_an_actor
      .insert(player, IsAnActor)
      .expect("Unable to insert is-an-actor for player!");
    $system_data
      .is_a_player
      .insert(player, IsAPlayer)
      .expect("Unable to insert is-a-player for player!");
    $system_data
      .has_initiative
      .insert(
        player,
        HasInitiative(Initiative {
          current: 0,
          increment: 8,
        }),
      )
      .expect("Unable to insert has-initiative for entity!");
    $system_data
      .has_name
      .insert(player, HasName("Player".into()))
      .expect("Unable to insert name for player!");
    $system_data
      .has_description
      .insert(
        player,
        HasDescription {
          initial: Some("You're an absolutely unexceptional specimen of whatever species you are.".to_string()),
          brief: "It's you, you idiot!".to_string(),
        },
      )
      .expect("Unable to insert description for player!");
    player
  }};
  ($system_data: expr, $in_room: expr) => {{
    use $crate::ecs::components::*;
    let player = create_player!($system_data);
    $system_data
      .is_in_room
      .insert(player, IsInRoom($in_room))
      .expect("Unable to insert is-in-room for entity!");
    player
  }};
}
