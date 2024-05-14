pub mod gameapp;
mod screens;
mod common;

use onion_engine::render::run;

fn main() {
    let game_app = Box::new(gameapp::GameApp::init());
    pollster::block_on(run(game_app));
}
