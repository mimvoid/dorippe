use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Box, Label, Paned};

fn main() -> glib::ExitCode {
  let app = Application::builder()
    .application_id("org.mimvoid.Dorippe")
    .build();

  app.connect_activate(init_window);
  app.run()
}

fn init_window(app: &Application) {
  let sidebar = Box::new(gtk::Orientation::Vertical, 0);
  {
    let recent = Button::with_label("Recent");
    let home = Button::with_label("Home");
    sidebar.append(&recent);
    sidebar.append(&home);
  }

  let main_pane = Box::new(gtk::Orientation::Vertical, 0);
  {
    let header = Box::new(gtk::Orientation::Horizontal, 2);

    let go_back = Button::with_label("<");
    let go_forward = Button::with_label(">");
    header.append(&go_back);
    header.append(&go_forward);

    let file_listings = Box::builder()
      .orientation(gtk::Orientation::Horizontal)
      .spacing(2)
      .halign(gtk::Align::Start)
      .valign(gtk::Align::Start)
      .hexpand(true)
      .vexpand(true)
      .build();

    let button1 = Button::with_label("mock-file-1.txt");
    let button2 = Button::with_label("mock-file-2.txt");
    file_listings.append(&button1);
    file_listings.append(&button2);

    let status = Box::new(gtk::Orientation::Horizontal, 2);
    let mock_count = Label::new(Some("2 files"));
    status.append(&mock_count);

    main_pane.append(&header);
    main_pane.append(&file_listings);
    main_pane.append(&status);
  }

  let content = Paned::builder()
    .start_child(&sidebar)
    .end_child(&main_pane)
    .build();

  let window = ApplicationWindow::builder()
    .application(app)
    .title("Dorippe")
    .child(&content)
    .build();

  window.present();
}
