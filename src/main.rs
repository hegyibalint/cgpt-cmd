use std::{env, io};
use std::io::{Error, Read, Write};
use ::config::{Config, Environment, File, FileFormat, FileSourceFile};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::config::{get_config_file_path, load_config};

mod config;

fn read_prompt(subject: &str, role: Option<&str>) -> Result<String, Error> {

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(stdout, "You")?;

    match role {
        Some(role) => {
            write!(stdout, " / ")?;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            write!(stdout, "{}", role)?;
        },
        None => {}
    }

    stdout.reset()?;
    write!(stdout, " > ")?;
    stdout.flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    return Ok(buffer);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config_path = get_config_file_path().unwrap();
    let config = load_config(&config_path);


    let system_prompt = read_prompt("You", Some("System")).unwrap();

    loop {
        let system_prompt = read_prompt("You", None).unwrap();
    }
}
