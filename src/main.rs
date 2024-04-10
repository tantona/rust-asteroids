use raylib::prelude::*;
mod player;
mod utilities;

const WIDTH: f32 = 640f32;

const HEIGHT: f32 = 480f32;

struct Game {
    player: player::Player,
}

impl Game {
    pub fn new() -> Self {
        return Game {
            player: player::Player::new(WIDTH, HEIGHT),
        };
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        self.player.update(d);
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.player.draw(d);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Hello, World")
        .build();
    rl.set_target_fps(60);

    let mut game = Game::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        // let dt = d.get_frame_time();
        d.clear_background(Color::BLACK);

        game.update(&mut d);
        game.draw(&mut d);
    }
}
