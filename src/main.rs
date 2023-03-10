use std::{env, io};
use std::io::{Error, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::chat::{Session, Role};
use crate::config::{load_config};

mod config;
mod chat;

fn main() {
    let _args: Vec<String> = env::args().collect();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    let config = load_config().unwrap();
    let mut chat_session = Session::new(config.api_key, config.model);

    print_prompt(&mut stdout, Role::System).unwrap();
    let system_prompt = read_prompt().unwrap();
    let response = chat_session.say(Role::System, system_prompt).unwrap();

    print_prompt(&mut stdout, Role::Assistant).unwrap();
    print_message(&mut stdout, &response.content).unwrap();
    writeln!(stdout, "").unwrap();

    loop {
        print_prompt(&mut stdout, Role::User).unwrap();
        let prompt = read_prompt().unwrap();
        let response = chat_session.say(Role::User, prompt).unwrap();

        print_message(&mut stdout, &response.content).unwrap();
        writeln!(stdout, "").unwrap();
    }
}

fn print_prompt(stdout: &mut StandardStream, role: Role) -> Result<(), Error> {
    match role {
        Role::System | Role::User => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            write!(stdout, "You")?;
            stdout.reset()?;

            match role {
                Role::System => {
                    write!(stdout, " / ")?;
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                    write!(stdout, "System")?;
                },
                _ => {}
            }
        },
        Role::Assistant => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            write!(stdout, "ChatGPT")?;
        }
    }

    stdout.reset()?;
    write!(stdout, " > ")?;
    stdout.flush()?;

    Ok(())
}

fn print_message(stdout: &mut StandardStream, message: &String) -> Result<(), Error> {
    stdout.write_all(message.trim().as_bytes())?;
    stdout.flush()?;
    Ok(())
}


fn read_prompt() -> Result<String, Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    return Ok(buffer);
}