mod main_pane;
mod sidebar;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Paned, glib};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.mimvoid.Dorippe")
        .build();

    app.connect_activate(init_window);
    app.run()
}

fn init_window(app: &Application) {
    let sidebar_pane = sidebar::build_sidebar();
    let main_pane = main_pane::build_main_pane();

    let content = Paned::builder()
        .start_child(&sidebar_pane)
        .end_child(&main_pane)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dorippe")
        .child(&content)
        .build();

    window.present();
}
