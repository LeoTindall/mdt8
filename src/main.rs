extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate chrono;

extern crate clap;

extern crate xdg;

mod state;
mod cli;
use cli::{get_cli_args, Command};

fn main() {
    let (config_path, command) = get_cli_args();

    let mut config = match state::State::load_from(&config_path) {
        Ok(s) => s,
        Err(e) => {
            println!("Could not load config:{}\nCreating new config at '{}'.", e, config_path.display());
            state::State::default()
        }
    };

    config.set_today_tracked();

    let result = match command {
        Command::Status => {
            println!("You plan to spend {} per day on mindfulness.", config.goal_string());
            println!("So far, you've spent {}.", config.completed_today_string());
            Ok(())
        }
        Command::Start => {
            config.start_session()
        }
        Command::Stop => {
            config.stop_session()
        }
        Command::Cancel => {
            config.cancel_session()
        }
        Command::Mod(v) => {
            config.modify(v);
            Ok(())
        }
    };

    match result {
        Ok(_) => {
            match command {
                Command::Start => println!("Started timer.\nRemember to breathe deeply and relax."),
                Command::Stop => println!("Stopped timer."),
                Command::Cancel => println!("Cancelled ongoing session."),
                Command::Mod(n) => println!("Modified today's total time by {} minutes.", n),
                Command::Status => ()
            }
        }
        Err(e) => {
            match command {
                Command::Start => println!("Could not start session: {}", e),
                Command::Stop => println!("Could not stop session: {}", e),
                Command::Cancel => println!("Could not cancel session: {}", e),
                _ => println!("Unknown error: {}", e)
            }
        }
    }

    config.write_to(&config_path);
}
