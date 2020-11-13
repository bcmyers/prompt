#![cfg(target_family = "unix")]

mod git;
mod k8;

use std::env;
use std::path::Path;

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

    colored::control::set_override(true);

    let mut s = String::new();

    if let Some(ref k) = k8::k8() {
        s.push_str(k);
        s.push('\n');
    }

    match virtual_env() {
        Some(ve) => {
            s.push_str(&format!(" venv:      {}\n", ve));
        }
        None => (),
    };

    s.push_str(&base(&opt.color));

    if let Some(ref g) = git::git() {
        s.push(' ');
        s.push_str(g);
    }

    println!("{}", s);
}

fn base(bg_color: &str) -> String {
    let fg_color = match bg_color.as_ref() {
        "black" => "white",
        "red" => "white",
        "green" => "black",
        "yellow" => "white",
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
    format!(
        " {} {}@{} {} ",
        Local::now().format("%H:%M"),
        &whoami::username(),
        whoami::hostname(),
        current_dir(),
    )
    .bold()
    .color(fg_color)
    .on_color(bg_color)
    .to_string()
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

fn virtual_env() -> Option<String> {
    let var = env::var("VIRTUAL_ENV").ok()?;
    let env = Path::new(&var).file_name()?.to_str()?;
    Some(format!("{}", env))
}
