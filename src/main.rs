use ggez::graphics;

mod system;

use ggez::event;
use ggez::glam::*;
use ggez::graphics::Color;
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use system::RigidBody;

struct MainState {
    objects: Vec<RigidBody>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            objects: vec![RigidBody::new((20.0, 10.0), 10.0)],
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta();

        for object in &mut self.objects {
            if !object.forces.contains_key("force1") {
                object.add_persistent_force("force1", (0.0, 10.0));
            }
            if !object.forces.contains_key("force3") {
                object.add_persistent_force("force3", (14.0, 10.0));
            }
            object.update(dt.as_secs_f32());
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
            let circle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect {
                    h: object.mass * 20.0,
                    w: object.mass * 20.0,
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
