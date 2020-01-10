#![cfg(target_family = "unix")]

mod git;

use std::env;

use chrono::Local;
use colored::Colorize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Background color
    #[structopt(short = "c", long = "color", default_value = "blue", possible_values = &[
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "white",
        "bright black",
        "bright red",
        "bright green",
        "bright yellow",
        "bright blue",
        "bright magenta",
        "bright cyan",
        "bright white",
    ])]
    color: String,
}

fn main() {
    let opt = Opt::from_args();

    let bg_color = opt.color;
    let fg_color = match bg_color.as_ref() {
        "black" => "white",
        "red" => "white",
        "green" => "black",
        "yellow" => "black",
        "blue" => "white",
        "magenta" => "white",
        "cyan" => "white",
        "white" => "black",
        "bright black" => "bright white",
        "bright red" => "bright white",
        "bright green" => "right black",
        "bright yellow" => "bright black",
        "bright blue" => "bright white",
        "bright magenta" => "bright white",
        "bright cyan" => "bright white",
        "bright white" => "bright black",
        _ => unreachable!(),
    };

    let username = whoami::username();

    let s = format!(
        " {} {}@{} {} ",
        Local::now().format("%H:%M"),
        &username,
        whoami::hostname(),
        current_dir(),
    )
    .bold()
    .color(fg_color)
    .on_color(bg_color);

    colored::control::set_override(true);

    match git::git() {
        Some(t) => println!("{} {}", s, t),
        None => println!("{}", s),
    }
}

fn current_dir() -> String {
    let dir = env::current_dir().unwrap();
    let mut output = dir.into_os_string().into_string().unwrap();

    let f = || {
        let home = dirs_sys::home_dir()?
            .canonicalize()
            .ok()?
            .into_os_string()
            .into_string()
            .ok()?;
        Some(home)
    };

    if let Some(home) = f() {
        if output.starts_with(&home) {
            output = output.replacen(&home, "~", 1);
        }
    }

    output
}
