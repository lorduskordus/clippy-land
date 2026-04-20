mod app;
mod i18n;
mod ipc;
mod services;

fn main() -> cosmic::iced::Result {
    for arg in std::env::args().skip(1) {
        if arg == "--toggle" || arg == "-t" {
            if let Err(e) = ipc::send_toggle_signal() {
                eprintln!("Failed to toggle clippy-land: {e}");
                std::process::exit(1);
            }
            return Ok(());
        }

        if arg == "-h" || arg == "--help" {
            println!("Clippy Land - Clipboard history applet for COSMIC");
            println!();
            println!("USAGE:");
            println!("    cosmic-applet-clippy-land [OPTIONS]");
            println!();
            println!("OPTIONS:");
            println!("    -t, --toggle    Toggle the clipboard popup via keyboard shortcut");
            println!("    -h, --help      Print this help message");
            println!();
            println!("KEYBOARD SHORTCUT SETUP:");
            println!("    1. Open COSMIC Settings > Keyboard > Custom Shortcuts");
            println!("    2. Click 'Add Custom Shortcut'");
            println!("    3. Name:    Clipboard History");
            println!("    4. Command: cosmic-applet-clippy-land --toggle");
            println!("    5. Shortcut: Press Super+V (or your preferred shortcut)");
            return Ok(());
        }
    }

    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    i18n::init(&requested_languages);
    cosmic::applet::run::<app::AppModel>(())
}
