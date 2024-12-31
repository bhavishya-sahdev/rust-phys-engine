use std::collections::HashMap;

use nalgebra::Vector2;

pub struct RigidBody {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub mass: f32,
    pub forces: HashMap<String, Vector2<f32>>,
}

impl RigidBody {
    const MIN_MASS_FOR_GRAVITY: f32 = 10.0; // Only objects above this mass create significant gravity
    const MIN_DISTANCE_FOR_GRAVITY: f32 = 0.1; // Minimum distance to prevent extreme forces
    const MAX_DISTANCE_FOR_GRAVITY: f32 = 1000.0; // Beyond this distance, gravity is negligible
    const GRAVITY_CONSTANT: f32 = 9.81; // Simplified gravity constant

    /// Create a new RigidBody object
    pub fn new(position: (f32, f32), mass: f32) -> Self {
        RigidBody {
            position: Vector2::from([position.0, position.1]),
            velocity: Vector2::from([0.0, 0.0]),
            acceleration: Vector2::from([0.0, 0.0]),
            mass,
            forces: HashMap::new(),
        }
    }

    pub fn calculate_gravity(&self, other: &RigidBody) -> Vector2<f32> {
        // Early exit if either mass is too small to matter
        if self.mass < Self::MIN_MASS_FOR_GRAVITY || other.mass < Self::MIN_MASS_FOR_GRAVITY {
            return Vector2::from([0.0, 0.0]);
        }

        let direction = other.position - self.position;
        let distance_sq = direction.magnitude_squared();

        // Early exit if objects are too close or too far
        if !(Self::MIN_DISTANCE_FOR_GRAVITY * Self::MIN_DISTANCE_FOR_GRAVITY
            ..=Self::MAX_DISTANCE_FOR_GRAVITY * Self::MAX_DISTANCE_FOR_GRAVITY)
            .contains(&distance_sq)
        {
            return Vector2::from([0.0, 0.0]);
        }

        let force_magnitude = Self::GRAVITY_CONSTANT * (self.mass * other.mass) / distance_sq;
        direction.normalize() * force_magnitude
    }

    // Add a named force that will persist between updates
    pub fn add_persistent_force(&mut self, name: &str, force: (f32, f32)) -> &Self {
        self.forces
            .insert(name.to_string(), Vector2::from([force.0, force.1]));
        self
    }

    // Remove a named force
    pub fn remove_force(&mut self, name: &str) -> &Self {
        self.forces.remove(name);
        self
    }

    /// Apply an impulse vector to the body.
    /// Directly updates velocity using J=F*dt
    pub fn apply_impulse(&mut self, impulse: (f32, f32)) -> &Self {
        let impulse_vec = Vector2::from([impulse.0, impulse.1]);
        self.velocity += impulse_vec / self.mass;
        self
    }

    /// Get the kinetic energy of the body.
    /// K = 1/2 * m * v^2
    pub fn get_kinetic_energy(&self) -> f32 {
        0.5 * self.mass * (self.velocity.magnitude_squared())
    }

    /// Get the current momentum of a body.
    /// p = mv
    pub fn get_momentum(&self) -> Vector2<f32> {
        self.velocity * self.mass
    }

    /// Reset acceleration to zero
    /// Important to call this at the end of each physics step
    pub fn reset_acceleration(&mut self) -> &Self {
        self.acceleration = Vector2::from([0.0, 0.0]);
        self
    }

    /// Update position and velocity using semi-implicit Euler integration
    pub fn update(&mut self, dt: f32) -> &Self {
        // Reset acceleration
        self.acceleration = Vector2::from([0.0, 0.0]);

        // Sum all forces and convert to acceleration
        for force in self.forces.values() {
            self.acceleration += *force / self.mass;
        }

        // Update velocity first (semi-implicit Euler)
        self.velocity += self.acceleration * dt;
        // Then update position using new velocity
        self.position += self.velocity * dt;

        self.forces.clear();

        self
    }

    /// Directly set velocity
    pub fn set_velocity(&mut self, velocity: (f32, f32)) -> &Self {
        self.velocity = Vector2::from([velocity.0, velocity.1]);
        self
    }

    /// Directly apply acceleration
    pub fn apply_acceleration(&mut self, acceleration: (f32, f32)) -> &Self {
        self.acceleration = Vector2::from([acceleration.0, acceleration.1]);
        self
    }
}
