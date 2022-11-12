#[macro_export]
macro_rules! has_initiative {
  ($data: expr, $entity: expr, $increment: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::components::*;
    use $crate::initiative::Initiative;
    $data
      .has_initiative
      .insert(
        $entity,
        HasInitiative(Initiative {
          current: 0,
          increment: $increment,
        }),
      )
      .expect("Unable to insert has-initiative for entity!");
  }};
}