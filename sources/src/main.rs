mod platform;
mod ui;

use ui::{UserChoice, clear_screen, menu};

fn main() {
    platform::enable_utf8_terminal();
    // test for menu
    loop {
        let choice = menu();

        clear_screen();

        match choice {
            UserChoice::SpoofRandom => {
                println!("→ Spoofing Random MAC address...");
            }
            UserChoice::SetCustom => {
                println!("→ Setting Custom MAC address...");
            }
            UserChoice::SpoofVendor => {
                println!("→ Spoofing Vendor Specific MAC address...");
            }
            UserChoice::Exit => {
                println!("Goodbye!");
                break;
            }
        }
    }
}
