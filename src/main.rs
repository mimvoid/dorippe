mod files;
mod ui;

use gtk::prelude::*;
use gtk::{Application, Paned, glib};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.mimvoid.Dorippe")
        .build();

    app.connect_activate(init_window);
    app.run()
}

fn init_window(app: &Application) {
    let window = ui::DorippeWindow::new_for_home(app);

    let sidebar_pane = ui::build_sidebar();
    let main_pane = ui::MainPane::new(window.file_browser());

    let content = Paned::builder()
        .start_child(&sidebar_pane)
        .end_child(&main_pane)
        .shrink_start_child(true)
        .shrink_end_child(true)
        .position(128)
        .build();

    window.set_child(Some(&content));
    window.present();
}
