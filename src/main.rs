use std::process;
use std::io::{self, BufRead, Write};
use crate::interpreter::Token;
mod builtins;
mod interpreter;

fn moonsh_launch(command: &str, args: Vec<&str>) -> Result<i32, &'static str> {
  // Filter for ^D and give exit_failure
  // Treat ^C like bash does
  // Handle help, cd, and exit / logout gracefully

  if builtins::is_builtin(command) {
    match builtins::execute_builtin(command, args) {
      Ok(status) => {
        return Ok(status);
      }
      Err(e) => {
        match e {
          "exit" => {
            return Err(e);
          }
          _ => {
            println!("{}", e);
            return Ok(0);
          }
        }
      }
    }
  }

  else {
    match process::Command::new(command).args(args).status() {
      Ok(_) => {}
      Err(e) => {
        println!("{}: {}", command, e);
      }
    }
  }
  Ok(0)
}

fn moonsh_read_line() -> io::Result<String> {
  let mut buf = String::new();
  let stdin = io::stdin();
  let mut handle = stdin.lock();

  handle.read_line(&mut buf)?;

  Ok(buf)
}

fn moonsh_loop(prompt: &str) -> Result<i32, String> {
  loop {
    print!("{}", prompt);
    io::stdout().flush().expect("Could not flush stdout.");

    // Read line and check for read errors
    let line;
    match moonsh_read_line() {
      Ok(l) => line = l,
      Err(e) => {
        break Err(e.to_string());
      }
    }

    // Parse command into args for sys call
    let mut args: Vec<&str> = line.split(' ').collect();

    // Trim leading and trailing whitespace
    args = args.iter().map(|arg| arg.trim()).collect();
    
    // Interpret moonsh wildcards and other control constructs (TODO: semicolons...)
    // in all but the first element of args
    let mut arg_tokens: Vec<Vec<Token>> = Vec::new();
    let mut common_args: Vec<&str> = Vec::new();

    common_args.push(args[0]); // Common args are those that do not need tokenized

    for arg in &args[1..] {

      // Don't tokenize options
      match arg.chars().nth(0) {
        Some('-') => {
          common_args.push(arg);
          continue;
        }
        _ => {}
      }

      match interpreter::parse_arg(arg) {
        Ok(tokens) => {
          arg_tokens.push(tokens);
        }
        Err(e) => {
          println!("{}", e);
        }
      }
    }

    // Build regex from tokens
    let re_vec: Vec<String> = interpreter::build_regex(arg_tokens);

    //println!("{:?}", re_vec);

    // Enumerate combinations with regex-matching paths/files
    // Add combinations to common args in a list of lists
    // (command) [all regex matches in fs] [all regex matches in fs]
    // iterate over this list running moonsh_launch for each entry
    //
    let paths: Vec<Vec<String>>; 

    match interpreter::valid_paths(re_vec) {
      Err(e) => {
        println!("{}", e);
        continue;
      }
      Ok(p) => {
        paths = p;
      }
    }

    
    let mut flat_paths: Vec<String> = paths.into_iter().flatten().collect();
    let borrowed_flat: Vec<&str> = flat_paths.iter().map(AsRef::as_ref).collect();
    common_args.extend(borrowed_flat.iter().cloned());
    //common_args.append(flat_paths.iter().map(AsRef::as_ref).collect());
    //common_args.append(flat_paths.iter().map(AsRef::as_ref).collect());


     // let mut final_args: Vec<&str> = Vec::new();

     // for arg in common_args { final_args.push(arg); }
     // for v in paths {
     //   for path in v {
     //     final_args.push(&path);
     //   }
     // }

    //match moonsh_launch(common_args[0], common_args[1..].to_vec()) {
    //  Ok(_) => {} // Nothing to see here
    //  Err(e) => { // Exiting gracefully
    //    break Err(e.to_string());
    //  }
    //}
    match moonsh_launch(args[0], args[1..].to_vec()) {
      Ok(_) => {}
      Err(e) => {
        break Err(e.to_string());
      }
    }
  }
}

fn main() {
  // config
  // Just needs basic pwd text (with limit?)
  let prompt: &str = "> ";
  
  // loop
  let code: Result<i32, String> = moonsh_loop(prompt);

  // clean up
  match code {
    Ok(code) => {
      process::exit(code);
    }
    Err(e) => {
      println!("{}", e);
    }
  }
}
