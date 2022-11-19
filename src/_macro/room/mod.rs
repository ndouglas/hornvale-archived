#[macro_export]
macro_rules! create_room {
  ($data: expr, $name: expr, $brief_description: expr) => {{
    let room = $data.entities.create();
    has_name!($data, room, $name);
    has_brief_description!($data, room, $brief_description);
    has_passages!($data, room);
    is_a_room!($data, room);
    room
  }};
}

#[macro_export]
macro_rules! format_room {
  ($data: expr, $room: expr) => {{
    use specs::prelude::*;
    use $crate::entity::RoomId;
    let mut string = String::new();
    if let Some(name) = get_name!($data, $room) {
      string.push_str(format!("<bold>{}<reset>\n", name).as_str());
    }
    if let Some(description) = get_brief_description!($data, $room) {
      string.push_str(format!("{}\n", description.0).as_str());
    }
    {
      let room_id = RoomId($room.id());
      for (_entities, _is_in_room, _is_an_object, has_brief_description) in (
        &$data.entities,
        &$data.is_in_room,
        &$data.is_an_object,
        &$data.has_brief_description,
      )
        .join()
        .filter(|(_entity, is_in_room, _is_an_object, _has_brief_description)| is_in_room.0 == room_id)
      {
        string.push_str(format!("<fg_ext180>{}<reset>\n", has_brief_description.0).as_str());
      }
    }
    {
      let room_id = RoomId($room.id());
      for (_entities, _is_in_room, _is_an_actor, has_brief_description, _is_a_player) in (
        &$data.entities,
        &$data.is_in_room,
        &$data.is_an_actor,
        &$data.has_brief_description,
        !&$data.is_a_player,
      )
        .join()
        .filter(|(_entity, is_in_room, _is_an_actor, _has_brief_description, _)| is_in_room.0 == room_id)
      {
        string.push_str(format!("<fg_ext162>{}<reset>\n", has_brief_description.0).as_str());
      }
    }
    if let Some(passages) = get_passages!($data, $room) {
      string.push_str(format!("<green>{}<reset>\n", passages).as_str());
    }
    format!("{}", string)
  }};
}
