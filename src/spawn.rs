use rand::{self, Rng};
use gfx;

use {Particle, System};

pub struct Cone<F> {
    pub spawn_time: f32,
    pub life_time: f32,
    pub pos: [f32; 2],
    pub orientation: f32,
    pub spread: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub angle: f32,
    pub friction: f32,
    pub size: [f32; 2],
    pub color: F,
}

impl<F: Fn(f32, f32) -> [f32; 3]> Cone<F> {
    /// Spawn a bunch of particles in `orientation` (radians), spread out by `spread` (radians).
    /// The speed is chosen uniformly between `min_speed` and `max_speed`. The color of the particle
    /// can be calculated depending on the angle and the speed with the `color` function.
    pub fn spawn<R: gfx::Resources>(&self, system: &mut System<R>, n: usize) {
        let mut rng = rand::thread_rng();

        for _ in 0..n {
            let angle = rng.gen_range(
                self.orientation - self.spread,
                self.orientation + self.spread,
            );
            let speed = rng.gen_range(self.min_speed, self.max_speed);
            let vel = [angle.cos() * speed, angle.sin() * speed];
            let color = (self.color)(angle, speed);
            let particle = Particle {
                spawn_time: self.spawn_time,
                life_time: speed / self.friction,
                pos: self.pos,
                vel,
                angle: 0.0,
                angular_vel: 0.0,
                friction: self.friction,
                color,
                alpha_exp: 1.0,
                size: self.size,
            };
            system.spawn(&particle);
        }
    }
}
