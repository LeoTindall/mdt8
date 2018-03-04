use clap::{Arg, App, SubCommand};
use xdg::BaseDirectories;

use std::process::exit;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Command {
    Status,
    Start,
    Stop,
    Cancel,
    Mod(i32)
}

pub fn get_cli_args() -> (PathBuf, Command) {
    let app = App::new("MDT8 Meditation Aid")
                    .version(env!("CARGO_PKG_VERSION"))
                    .author("Leo Tindall <lfstindall@gmail.com>")
                    .about("Aids in the cultivation of a regular mindfulness meditation practice. Use 'mdt8 start' and 'mdt8 stop' to log meditation time, and 'mdt8 status' to view your meditation time today.")
                    .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .takes_value(true)
                            .value_name("FILE")
                            .help("The config file to use. By default, '.config/mdt8.json'. Set 'goal_minutes' in that file to set your daily goal."))
                    .subcommand(SubCommand::with_name("status")
                                .about("Prints the day's meditation stats."))
                    .subcommand(SubCommand::with_name("start")
                                .about("Starts the session timer"))
                    .subcommand(SubCommand::with_name("stop")
                                .about("Stops the session timer, adding the time measured to the day's tally."))
                    .subcommand(SubCommand::with_name("cancel")
                                .about("Stops the session timer, discarding the time."))
                    .subcommand(SubCommand::with_name("mod")
                                .about("Manually add or subtract time from the day's tally.")
                                .arg(Arg::with_name("VALUE")
                                        .help("The value by which to change the tally, in minutes. May be negative.")
                                        .required(true)
                                        .index(1)));
    let mut helpapp = app.clone();

    let matches = app.get_matches();
    let config = match matches.value_of_os("config") {
        Some(s) => PathBuf::from(s),
        None => BaseDirectories::new()
                .expect("Could not determine XDG config directory")
                .place_config_file("mdt8.json")
                .expect("Could not place config file")
    };

    if let Some(_) = matches.subcommand_matches("status") {
        return (config, Command::Status);
    }
    if let Some(_) = matches.subcommand_matches("start") {
        return (config, Command::Start);
    }
    if let Some(_) = matches.subcommand_matches("stop") {
        return (config, Command::Stop);
    }
    if let Some(_) = matches.subcommand_matches("cancel") {
        return (config, Command::Cancel);
    }
    if let Some(matches) = matches.subcommand_matches("mod") {
        let v_str = matches.value_of("VALUE").unwrap();
        let v = i32::from_str_radix(v_str, 10).expect("Invalid value provided; must be a negative or positive integer");
        return (config, Command::Mod(v));
    }
    helpapp.print_help().expect("Could not print help");
    exit(1);
}
