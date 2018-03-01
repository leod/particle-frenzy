extern crate gfx_device_gl;
extern crate ggez;
extern crate hsl;
extern crate particle_frenzy;
extern crate rand;

use std::f32;

use rand::Rng;
use ggez::{conf, event, graphics, timer, Context, ContextBuilder, GameResult};

struct MainState {
    system: particle_frenzy::System<gfx_device_gl::Resources>,
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let time = timer::duration_to_f64(timer::get_time_since_start(ctx)) as f32;

        graphics::clear(ctx);

        {
            let transform = graphics::get_projection(ctx) * graphics::get_transform(ctx);
            let (factory, device, encoder, _depthview, _colorview) = graphics::get_gfx_objects(ctx);
            self.system
                .render(factory, encoder, time, &transform.into());
            encoder.flush(device);
        }

        graphics::present(ctx);

        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        _state: event::MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) {
        let time = timer::duration_to_f64(timer::get_time_since_start(ctx)) as f32;

        for _ in 0..2000 {
            let angle = rand::thread_rng().gen::<f32>() * f32::consts::PI * 2.0;
            let min_speed = 100.0;
            let max_speed = 200.0;
            let rel_speed = ((xrel as f32).powi(2) + (yrel as f32).powi(2)).sqrt() / 4.0;
            let fact = rand::thread_rng().gen_range(min_speed, max_speed);
            let speed = fact * rel_speed;
            let vel = [angle.cos() * speed, angle.sin() * speed];
            let friction = 1000.0;
            let life_time = speed / friction;

            let hsl = hsl::HSL {
                h: ((1.0 - (fact - min_speed) / (max_speed - min_speed)) * 360.0) as f64,
                s: 0.9,
                l: 0.4,
            };
            let (r, g, b) = hsl.to_rgb();

            let particle = particle_frenzy::Particle {
                spawn_time: time,
                life_time,

                pos: [x as f32, y as f32],
                vel,
                angle: 0.0,
                angular_vel: 1.0,
                friction,

                color: [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0],
                alpha_exp: 1.0,

                size: [2.0, 2.0],
            };

            self.system.spawn(&particle);
        }
    }
}

pub fn main() {
    let ctx = &mut ContextBuilder::new("particle-frenzy", "leod")
        .window_mode(conf::WindowMode::default().dimensions(1600, 900))
        .build()
        .unwrap();

    let system = {
        let target = graphics::get_screen_render_target(ctx);
        let factory = graphics::get_factory(ctx);
        particle_frenzy::System::new(factory, target, 100_000, 50)
    };

    let state = &mut MainState { system };
    event::run(ctx, state).unwrap();
}
