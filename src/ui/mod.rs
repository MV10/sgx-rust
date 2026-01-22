//! UI module - handles window setup and callback wiring

mod callbacks;

use slint::ComponentHandle;

slint::include_modules!();

/// Sets up the main window with all callbacks wired
pub fn setup() -> MainWindow {
    let ui = MainWindow::new().expect("Failed to create main window");

    callbacks::setup_grid_callbacks(&ui);
    callbacks::setup_content_callbacks(&ui);
    callbacks::setup_highlight_callbacks(&ui);
    callbacks::setup_toolbar_callbacks(&ui);

    ui
}

/// Runs the main window event loop
pub fn run(ui: MainWindow) {
    ui.run().expect("Failed to run main window");
}

// Re-export MainWindow for use by other modules
pub use self::MainWindow as AppWindow;
