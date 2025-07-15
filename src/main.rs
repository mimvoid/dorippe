mod files;
mod ui;

use gtk::prelude::*;
use gtk::{gdk, Application, Paned, glib};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.mimvoid.Dorippe")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(init_window);

    app.run()
}

fn load_css() {
    let display = gdk::Display::default().expect("Could not connect to display");
    let settings = gtk::Settings::for_display(&display);

    let fallback = gtk::CssProvider::new();
    fallback.load_named(
        "Adwaita",
        if settings.is_gtk_application_prefer_dark_theme() {
            Some("dark")
        } else {
            None
        },
    );

    gtk::style_context_add_provider_for_display(
        &display,
        &fallback,
        gtk::STYLE_PROVIDER_PRIORITY_THEME
    );
}

fn init_window(app: &Application) {
    let window = ui::DorippeWindow::new_for_home(app);

    let sidebar_pane = ui::Sidebar::new();
    let main_pane = ui::MainPane::new(window.file_browser());

    let panes = Paned::builder()
        .start_child(&sidebar_pane)
        .end_child(&main_pane)
        .shrink_start_child(true)
        .shrink_end_child(true)
        .position(128)
        .build();

    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.append(&ui::Toolbar::new());
    content.append(&panes);

    window.set_child(Some(&content));
    window.present();
}
