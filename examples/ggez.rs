extern crate frenzy;
extern crate ggez;

use ggez::{conf, event, graphics, Context, GameResult};

struct MainState {}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

impl MainState {
    fn new(ctx: &mut Context) -> Self {
        MainState {}
    }
}

pub fn main() {
    let conf = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("ggez", "frenzy", conf).unwrap();

    let state = &mut MainState::new(ctx);
    event::run(ctx, state).unwrap();
}
