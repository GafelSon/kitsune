use colored::*;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use dialoguer::{Select, theme::ColorfulTheme};
use std::io::stdout;

pub enum UserChoice {
    SpoofRandom,
    SetCustom,
    SpoofVendor,
    Exit,
}

pub fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

fn header() {
    let ascci = r#"
        ___  __    ___  _________  ________  ___  ___  ________   _______
       |\  \|\  \ |\  \|\___   ___\\   ____\|\  \|\  \|\   ___  \|\  ___ \
       \ \  \/  /|\ \  \|___ \  \_\ \  \___|\ \  \\\  \ \  \\ \  \ \   __/|
        \ \   ___  \ \  \   \ \  \ \ \_____  \ \  \\\  \ \  \\ \  \ \  \_|/__
         \ \  \\ \  \ \  \   \ \  \ \|____|\  \ \  \\\  \ \  \\ \  \ \  \_|\ \
          \ \__\\ \__\ \__\   \ \__\  ____\_\  \ \_______\ \__\\ \__\ \_______\
           \|__| \|__|\|__|    \|__| |\_________\|_______|\|__| \|__|\|_______|
                                     \|_________|
        "#;
    println!("{}", ascci.bright_cyan());
    println!(
        "{}",
        "\t\t\tWindows MAC Address Spoofer v2.0 <@gafelson>\n"
            .bold()
            .green()
    );
}

pub fn menu() -> UserChoice {
    header();

    let options = vec![
        "🀢 Spoof Random MAC",
        "🀢 Set Custom MAC",
        "🗘 Vendor Preset",
        "⏻ Exit",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an action")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();

    match selection {
        0 => UserChoice::SpoofRandom,
        1 => UserChoice::SetCustom,
        2 => UserChoice::SpoofVendor,
        _ => UserChoice::Exit,
    }
}
