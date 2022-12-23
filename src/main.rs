use app::App;

mod app;
mod tui;
mod renderer;

fn main() {
    App::new().start();
}
