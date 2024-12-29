use std::ops::{Add, AddAssign, Div, Mul, Sub};

use nalgebra::Vector2;

pub trait Move<T> {
    fn apply_force(&mut self, force: Vector2<T>, dt: T) -> &Self;
    fn apply_velocity(&mut self, dt: T) -> &Self;
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicsObject<T = f64> {
    pub position: Vector2<T>,
    pub velocity: Vector2<T>,
    pub mass: T,
}

impl<
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + AddAssign + Copy,
    > PhysicsObject<T>
{
    pub fn new(velocity: Vector2<T>, position: Vector2<T>, mass: T) -> Self {
        PhysicsObject {
            mass,
            position,
            velocity,
        }
    }
}

impl<
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + AddAssign + Copy,
    > Move<T> for PhysicsObject<T>
{
    fn apply_force(&mut self, force: Vector2<T>, dt: T) -> &Self {
        self.velocity[0] += (force[0] / self.mass) * dt;
        self.velocity[1] += (force[1] / self.mass) * dt;
        self
    }

    fn apply_velocity(&mut self, dt: T) -> &Self {
        self.position[0] += self.velocity[0] * dt;
        self.position[1] += self.velocity[1] * dt;
        self
    }
}
