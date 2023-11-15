//
//
use gdk::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};
use gtk::{CssProvider, StyleContext};

fn main() {
    if gtk::init().is_err() {
        println!("ERROR: Failed to initialize GTK.");
        return;
    }

    let application =
        gtk::Application::new(Some("com.github.jean-turgeon.rusted"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("./rusted.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder
        .object("mainWindow")
        .expect("ERROR: Couldn't get window!");

    let provider = CssProvider::new();
    provider.load_from_path("./rusted.css").unwrap();

    let _context = window.style_context();
    StyleContext::add_provider_for_screen(
        &gdk::Screen::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.set_application(Some(application));

    //window.connect_delete_event(|_, _| {
    //    gtk::main_quit();
    //    gtk::signal::Inhibit(false)
    //});

    // fetch manifest information
    //let authors: &str = env!("CARGO_PKG_AUTHORS");
    //let description: &str = env!("CARGO_PKG_DESCRIPTION");
    //let name: &str = env!("CARGO_PKG_NAME");
    //let version: &str = env!("CARGO_PKG_VERSION");

    window.maximize();
    window.show_all();

    gtk::main();
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
