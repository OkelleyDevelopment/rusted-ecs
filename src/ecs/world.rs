use super::components::Name;
use super::components::Health;
use super::traits::ComponentVector;

pub struct World {
    pub name_components: Vec<Option<Name>>,
    pub health_components: Vec<Option<Health>>,

}

impl<T> ComponentVector for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None)
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            name_components: Vec::new(),
            health_components: Vec::new(),
        }
    }

    pub fn new_entity(&mut self, name: Option<Name>, health: Option<Health>) {
        self.name_components.push(name);
        self.health_components.push(health);

    }
}
