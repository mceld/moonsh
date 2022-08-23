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
            return cd(args);
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

fn cd(args: Vec<&str>) -> Result<i32, &'static str> {
    if args.len() == 0 {
        return Err("Expected an argument to cd");
    }

    let dir: &str = args[0];
    if dir.is_empty() {
        return Err("Expected an argument to cd");
    }

    let new_dir: &Path = Path::new(dir);
    match env::set_current_dir(&new_dir) {
        Ok(_) => { return Ok(0); }
        Err(_) => { return Err("No such directory"); }
    }
}

fn help() -> Result<i32, &'static str> {
    Err(
"moonsh

Builtins:
cd <dest>
exit
help
slice [dir]
    Define a new slice with 'dir' as the first directory in the stack.
    'dir' is optional and will use $HOME as a default.
slices
    Display currently allocated slices graphically.
pop <n>
    Pop n directories from the current slice's stack.
    'pop a' will remove all directories on the stack and fill the stack with the directory at $HOME.
reload <config_file>
    Tear down all current slices and read the given config file as a sequence of moonsh commands."
    )
}

fn exit() -> Result<i32, &'static str> {
    Err("exit")
}
