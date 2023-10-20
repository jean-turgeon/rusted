

use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};

fn main() {
    let application = gtk::Application::new(
        Some("com.github.jean-turgeon.rusted"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("rusted.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.object("mainWindow").expect("Couldn't get window1");
    window.set_application(Some(application));

    window.show_all();
}