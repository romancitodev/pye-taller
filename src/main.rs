mod app;
use iced::{Application, Settings};

fn main() {
    let _ = app::App::run(Settings::default());
}
