mod system;

use ggez::{event, glam::Vec2, graphics::*, Context, GameResult};
use nalgebra::Vector2;
use parry2d::query::Contact;
use system::RigidBody;

struct MainState {
    objects: Vec<RigidBody>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            objects: vec![
                RigidBody::new((20.0, 10.0), 10.0),
                RigidBody::new((80.0, 80.0), 10.0),
            ],
        };
        Ok(s)
    }
}

fn resolve_collision(bodies: &mut Vec<RigidBody>, i: usize, j: usize, contact: &Contact) {
    // Basic impulse-based collision response
    let body1 = bodies[i].clone();
    let body2 = bodies[j].clone();

    let normal = Vector2::new(contact.normal1.x, contact.normal1.y);
    let relative_velocity = body2.velocity - body1.velocity;

    // Coefficient of restitution (bounciness)
    let restitution = 0.5;

    let impulse_magnitude = -(1.0 + restitution) * relative_velocity.dot(&normal)
        / (1.0 / body1.mass + 1.0 / body2.mass);

    let impulse = normal * impulse_magnitude;

    // Apply impulses
    bodies[i].velocity -= impulse / body1.mass;
    bodies[j].velocity += impulse / body2.mass;

    // Resolve penetration
    let percent = 0.2; // Penetration resolution percentage
    let slop = 0.01; // Penetration allowance
    let penetration = contact.dist.abs();

    if penetration > slop {
        let correction = normal * (penetration * percent) / (1.0 / body1.mass + 1.0 / body2.mass);
        bodies[i].position -= correction / body1.mass;
        bodies[j].position += correction / body2.mass;
    }
}

// fn check_window_collision(body: &mut RigidBody, ctx: &ggez::Context) -> Option<Contact> {
//     let (width, height) = ctx.gfx.size();

//     // Get bounding sphere of the collider for simple bounds check
//     let bounds = body.collider.compute_aabb(&body.position.into());

//     // Check each border
//     // Bottom border
//     if body.position[1] >= height {
//         let normal1 = Vector::new(0.0, -1.0);
//         return Some(Contact {
//             normal1,
//             dist: body.position[1] - height,
//             point: Point::new(body.position[0], height),
//         });
//     }

//     None
// }

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta();

        if !self.objects[0].forces.contains_key("gravity") {
            self.objects[0].add_persistent_force("gravity", (10.0, 20.0));
        }

        (0..self.objects.len()).for_each(|i| {
            (i + 1..self.objects.len()).for_each(|j| {
                if let Some(contact) = self.objects[i].check_collision(&self.objects[j]) {
                    resolve_collision(&mut self.objects, i, j, &contact);
                }
            });
        });

        // update velocity and position based on forces
        for object in &mut self.objects {
            object.update(dt.as_secs_f32());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        let mut text = Text::new("Hello");
        text.set_scale(36.0);

        canvas.draw(&text, Vec2::new(3.0, 3.0));

        for object in &mut self.objects {
            let circle = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect {
                    h: object.mass * 10.0,
                    w: object.mass * 10.0,
                    x: object.position.x,
                    y: object.position.y,
                },
                Color::GREEN,
            )?;
            canvas.draw(&circle, Vec2::new(object.position[0], object.position[1]));
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}
pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("engine", "rpe");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;

    event::run(ctx, event_loop, state)
}
