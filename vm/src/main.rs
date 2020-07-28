//!
use std::collections::HashMap;

use cli::util::{prompt_mut, error, warning};
use cli::Program;

fn program_found(id: isize) -> String {
  warning(&format!("program with id {} found", id))
}

fn program_not_found(id: isize) -> String {
  warning(&format!("program with id {} not found", id))
}

fn command_not_found(command: &str) -> String {
  warning(&format!("command '{}' nod found", command))
}

struct MyResult<T, E> (Result<T, E>);

impl <T, E> From<Result<T, E>> for MyResult<T, E> {
  fn from(result: Result<T, E>) -> Self {
    MyResult(result)
  }
}

impl <T, E> MyResult<T, E> {
  fn unwrap_or_print(&self, message: &str) -> Option<&T> {
    let result = &self.0;
    match result {
      Ok(o) => Some(o),
      Err(_) => { println!("{}", message); None }
    }
  }
}

fn get<F: std::str::FromStr>(args: &Vec<&str>, n: usize) -> Result<F, String> {
  if args.len() > n {
    args[n].parse().map_err(|_| format!("{} th element is invalid format", n))
  } else {
    Err(format!("{} th element is required, but not provided", n))
  }
}

fn main() -> std::io::Result<()> {
  let mut programs: HashMap<isize, Program> = HashMap::with_capacity(32);
  prompt_mut("$", |line: &str| {
    let args: Vec<&str> = line.split(' ').collect();
    let command = args[0];
    match command {
      "start" => {
        match get::<isize>(&args, 1) {
          Ok(id) => { if let Some(_) = programs.insert(id, Program::new()) {
            println!("{}", program_found(id))
          }},
          Err(e) => println!("{}", error(&e)),
        }
      },
      "restart" => {
        match get::<isize>(&args, 1) {
          Ok(id) => { programs.insert(id, Program::new()); },
          Err(e) => println!("{}", error(&e)),
        }
      },
      "stop" => {
        match get::<isize>(&args, 1) {
          Ok(id) => { if let None = programs.remove(&id) {
            println!("{}", program_not_found(id))
          }},
          Err(e) => println!("{}", error(&e)),
        }
      },
      "run" => {
        match get::<isize>(&args, 1) {
          Ok(id) => {
            if let Some(program) = programs.get_mut(&id) {
              match get::<String>(&args, 2) {
                Ok(path) => {
                  program.run_script(&path).unwrap();
                },
                Err(_) => {
                  program.prompt().unwrap();
                },
              }
            } else {
              println!("{}", program_not_found(id))
            }
          },
          Err(e) => println!("{}", error(&e)),
        }
      },
      "test" => (),
      _ => println!("{}", command_not_found(command)),
    }
  })
}
