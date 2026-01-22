//! SlideGrid - Multi-monitor slideshow application

mod ui;
mod services;

fn main() {
    let window = ui::setup();
    ui::run(window);
}
