mod git;

use std::env;
use std::ffi::OsStr;
use std::path::Component;

use chrono::Local;
use colored::Colorize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Flag indicating that previous command failed
    #[structopt(long = "failure")]
    failure: bool,
}

fn main() {
    let opt = Opt::from_args();

    let (bg_color, fg_color) = if opt.failure {
        ("red", "white")
    } else {
        ("blue", "white")
    };

    let username = whoami::username();

    let s = format!(
        " {} {}@{} {} ",
        Local::now().format("%H:%M"),
        &username,
        whoami::hostname(),
        current_dir(&username),
    )
    .bold()
    .color(fg_color)
    .on_color(bg_color);

    match git::git() {
        Some(t) => println!("{} {}", s, t),
        None => println!("{}", s),
    }
}

fn current_dir(username: &str) -> String {
    let dir = env::current_dir().unwrap();

    let mut iter = dir.components();
    if let Some(component) = iter.next() {
        if component == Component::RootDir {
            if let Some(component) = iter.next() {
                if component == Component::Normal(OsStr::new("home")) {
                    if let Some(component) = iter.next() {
                        if component == Component::Normal(OsStr::new(username)) {
                            let mut output = String::from("~");
                            let rest = iter.as_path().as_os_str().to_str().unwrap();
                            if rest == "" {
                                return output;
                            } else {
                                output.push('/');
                                output.push_str(rest);
                                return output;
                            }
                        }
                    }
                }
            }
        }
    }

    dir.into_os_string().into_string().unwrap()
}
