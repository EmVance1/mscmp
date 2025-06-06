use crate::{
    repr::Config,
    error::Error,
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    New{ name: String, library: bool },
    Clean,
    Build{ config: Config, mingw: bool },
    Run  { config: Config, mingw: bool, args: Vec<String> },
    Test { config: Config, mingw: bool },
}

pub fn parse_input(mut args: Vec<String>) -> Result<Action, Error> {
    args.remove(0);
    if args.is_empty() { return Err(Error::MissingAction) }

    match args[0].as_str() {
        "new"|"n" => {
            if args.len() == 2 {
                Ok(Action::New{ name: args[1].clone(), library: false })
            } else if args.len() == 3 {
                if let Some(pos) = args.iter().position(|s| *s == "-lib") {
                    args.remove(pos);
                    Ok(Action::New{ name: args[1].clone(), library: true })
                } else {
                    Err(Error::BadAction(args[1].clone()))
                }
            } else {
                Err(Error::BadAction(args[2].clone()))
            }
        }
        "clean"|"c" => {
            if args.len() == 1 {
                Ok(Action::Clean)
            } else {
                Err(Error::BadAction(args[2].clone()))
            }
        }
        "build"|"b" => {
            let config = match args.get(1).map(|a| a.as_str()) {
                Some("-debug"  |"-d") => Config::Debug,
                Some("-release"|"-r") => Config::Release,
                _ => Config::Debug
            };
            Ok(Action::Build{ config, mingw: false })
        }
        "run"|"r" => {
            let (config, count) = match args.get(1).map(|a| a.as_str()) {
                Some("-debug"  |"-d") => (Config::Debug, 1),
                Some("-release"|"-r") => (Config::Release, 1),
                _ => (Config::Debug, 0)
            };
            args.remove(0);
            if count == 1 {
                args.remove(0);
            }
            Ok(Action::Run{ config, mingw: false, args })
        }
        "test"|"t" => {
            let config = match args.get(1).map(|a| a.as_str()) {
                Some("-debug"  |"-d") => Config::Debug,
                Some("-release"|"-r") => Config::Release,
                _ => Config::Debug
            };
            Ok(Action::Test{ config, mingw: false })
        }
        _ => Err(Error::BadAction(args[1].clone())),
    }
}

