use ggez::graphics;
use nalgebra::Vector2;
use schema::{Move, PhysicsObject};

mod schema;

use ggez::event;
use ggez::glam::*;
use ggez::graphics::Color;
use ggez::{Context, GameResult};

struct MainState {
    objects: Vec<PhysicsObject<f32>>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            objects: vec![
                PhysicsObject::new(Vector2::from([1.2, 2.2]), Vector2::from([1.2, 2.2]), 10.2),
                PhysicsObject::new(Vector2::from([1.2, 3.2]), Vector2::from([5.2, 2.2]), 9.2),
            ],
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta();

        for object in &mut self.objects {
            object.apply_force(Vector2::from([0.0, 5.0]), dt.as_secs_f32());
            object.apply_velocity(dt.as_secs_f32());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let mut text = graphics::Text::new("Hello");
        text.set_scale(36.0);

        canvas.draw(&text, Vec2::new(3.0, 3.0));

        for object in &mut self.objects {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                object.mass * 2.0,
                0.1,
                Color::WHITE,
            )?;
            canvas.draw(&circle, Vec2::new(object.position[0], object.position[1]));
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}
pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
