use factorio_blueprint::{blueprint_string_to_model, model_to_blueprint_string, blueprint_string_to_pretty_json};
use clap::{App, Arg};

enum Command {
    Decode(String),
    ReEncode(String),
}

fn parse_arguments() -> Option<Command> {
    let matches = App::new("JP Factorio Blueprint CLI")
        .version("0.1.0")
        .about("Factorio Blueprint Toolbox")
        .subcommand(App::new("reencode")
            .about("Decodes a blueprint string and then reencodes it")
            .arg(Arg::new("blueprint-string")
                .help("Factorio blueprint string as exported from the game")
                .required(true)
                .index(1)))
        .subcommand(App::new("decode")
            .about("Decodes a blueprint string and outputs the json")
            .arg(Arg::new("blueprint-string")
                .help("Factorio blueprint string as exported from the game")
                .required(true)
                .index(1)))
        .get_matches();

    match matches.subcommand() {
        Some(("decode", args)) => args
            .value_of("blueprint-string")
            .map(|b| Command::Decode(b.into())),
        Some(("reencode", args)) => args
            .value_of("blueprint-string")
            .map(|b| Command::ReEncode(b.into())),
        _ => None,
    }
}

fn reencode(blueprint: &str) -> factorio_blueprint::Result<()> {
    let model = blueprint_string_to_model(blueprint)?;
    let reencoded = model_to_blueprint_string(model)?;

    println!("{}", reencoded);

    Ok(())
}

fn decode(blueprint: &str) -> factorio_blueprint::Result<()> {
    let json = blueprint_string_to_pretty_json(blueprint)?;
    println!("{}", json);

    Ok(())
}

fn main() -> factorio_blueprint::Result<()> {
    match parse_arguments() {
        Some(Command::ReEncode(b)) => reencode(&b),
        Some(Command::Decode(b)) => decode(&b),
        _ => {
            eprintln!("Unknown command");
            Ok(())
        },
    }
}
