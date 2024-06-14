#![windows_subsystem = "windows"]

// We need the ctor macro for the replay test on macOS.
#[cfg(all(test, target_os="macos"))]
#[allow(unused_imports)]
#[macro_use]
extern crate ctor;

use gtk::prelude::*;
use gtk::gio::ApplicationFlags;

use packetry::ui::{
    activate,
    display_error,
    stop_cynthion
};

fn main() {
    if std::env::args().any(|arg| arg == "--test-cynthion") {
        test_cynthion::run_test();
    } else {
        let application = gtk::Application::new(
            Some("com.greatscottgadgets.packetry"),
            ApplicationFlags::NON_UNIQUE
        );
        application.connect_activate(|app| display_error(activate(app)));
        application.run_with_args::<&str>(&[]);
        display_error(stop_cynthion());
    }
}

mod test_cynthion;
#[cfg(test)]
mod test_replay;
