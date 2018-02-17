extern crate frenzy;
extern crate gfx_device_gl;
extern crate ggez;

use ggez::{conf, event, graphics, timer, Context, GameResult};

struct MainState {
    system: frenzy::ParticleSystem<gfx_device_gl::Resources>,
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let time = timer::duration_to_f64(timer::get_time_since_start(ctx)) as f32;

        let particle = frenzy::Particle {
            spawn_time: time,
            life_time: 2.0,
            pos: [0.0, 0.0],
            vel: [1.0, 0.0],
            angle: 0.0,
            angular_vel: 1.0,
            color: [1.0, 0.0, 0.0],
            size: [10.0, 10.0],
        };

        for _ in 0..10 {
            self.system.spawn(&particle);
        }

        {
            let transform = graphics::get_transform(ctx).clone();
            let (factory, device, encoder, _depthview, _colorview) = graphics::get_gfx_objects(ctx);
            self.system
                .render(factory, encoder, time, &transform.into());
            encoder.flush(device);
        }

        graphics::present(ctx);

        Ok(())
    }
}

pub fn main() {
    let conf = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("ggez", "frenzy", conf).unwrap();

    let system = {
        let target = graphics::get_screen_render_target(ctx);
        let factory = graphics::get_factory(ctx);
        frenzy::ParticleSystem::new(factory, target, 1000, 10)
    };

    let state = &mut MainState { system };
    event::run(ctx, state).unwrap();
}
