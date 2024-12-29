use super::PhysicsObject;

#[derive(Debug)]
pub struct PhysicsSystem {
    members: Vec<PhysicsObject>,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        PhysicsSystem { members: vec![] }
    }

    pub fn add_member(&mut self, member: PhysicsObject) {
        self.members.push(member);
    }
}
