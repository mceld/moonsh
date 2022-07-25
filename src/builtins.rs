//use lazy_static::lazy_static;
//use std::collections::HashMap;
//
//lazy_static! {
//    static ref BUILTINS: HashMap<&'static str, fn> = HashMap::from([
//        ("cd", cd)
//        , ("help", help)
//        , ("exit", exit)
//    ]);
//}

use std::env;
use std::path::Path;

const BUILTINS: &'static [&'static str] = &[
    "cd"
    , "help"
    , "exit"
];

pub fn is_builtin(name: &str) -> bool {
    BUILTINS.contains(&name)
}

pub fn execute_builtin(command: &str, args: Vec<&str>) -> Result<i32, &'static str>  {
    match command {
        "cd" => {
            return cd(args[0]);
        }
        "help" => {
            return help();
        }
        "exit" => {
            return exit();
        }
        _ => {
            println!("No implementation given for {} yet", command);
            return Ok(0);
        }
    }
}

fn cd(dir: &str) -> Result<i32, &'static str> {
    if dir.is_empty() {
        println!("Expected an argument to cd");
        return Ok(0);
    }

    let new_dir: &Path = Path::new(dir);

    assert!(env::set_current_dir(&new_dir).is_ok());
    return Ok(0);
}

fn help() -> Result<i32, &'static str> {
    Err("moonsh: A basic unix shell")
}

fn exit() -> Result<i32, &'static str> {
    Err("exit")
}
