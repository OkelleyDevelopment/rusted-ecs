use proto_ecs::ecs::{components::{Health, Name}, world::World};

fn main() {
    // the world of the "game"
    let mut world = World::new();
    
    // A player entity
    world.new_entity(Some(Name("Player")), Some(Health(20)));

    // World Creatures
    world.new_entity(Some(Name("Pig")), Some(Health(15)));
    world.new_entity(Some(Name("Sheep")), Some(Health(10)));
    world.new_entity(Some(Name("Cow")), Some(Health(12)));

    // Enemies
    world.new_entity(Some(Name("Zombie")), Some(Health(-5)));

    // Tools don't have a health, like in Terraria
    world.new_entity(Some(Name("Axe")), None);

    let world_entities = world
        .name_components
        .iter()
        .zip(world.health_components.iter())
        .filter_map(|(name, health): (&Option<Name>, &Option<Health>)| {
            Some((name.as_ref()?, health.as_ref()?))
        });

    for (name, health) in world_entities {
        if health.0 < 0 {
            println!("{} has passed", name.0);
        } else if health.0 == 10 && name.0 == "Sheep" {
            println!("{} baaaaaaaas", name.0);
        }
        else {
            println!("{} hasn't passed", name.0);
        }
    }
}
