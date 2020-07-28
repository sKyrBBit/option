pub fn print(s: &str) -> std::io::Result<()> {
  use std::io::{stdout, Write};
  let stdout = stdout();
  let mut stdout = stdout.lock();
  stdout.write(s.as_bytes())?;
  stdout.flush()
}

pub fn prompt(shell: &str, f: impl Fn(&str)) -> std::io::Result<()> {
  use std::io::{stdin, BufRead, BufReader};

  let stdin = stdin();
  let stdin = stdin.lock();
  let stdin = BufReader::new(stdin);
  let mut lines = stdin.lines();

  loop {
    print(&format!("{} ", shell))?;
    if let Some(Ok(line)) = lines.next() {
      if line == "exit" { break; }
      f(&line);
    } else {
      break;
    }
  }
  Ok(())
}

pub fn prompt_mut(shell: &str, mut f: impl FnMut(&str)) -> std::io::Result<()> {
  use std::io::{stdin, BufRead, BufReader};

  let stdin = stdin();
  let stdin = stdin.lock();
  let stdin = BufReader::new(stdin);
  let mut lines = stdin.lines();

  loop {
    print(&format!("{} ", shell))?;
    if let Some(Ok(line)) = lines.next() {
      if line == "exit" { break; }
      f(&line);
    } else {
      break;
    }
  }
  Ok(())
}

pub fn log(message: &str) -> String {
  format!("[*] {}", message)
}

pub fn warning(message: &str) -> String {
  format!("warning | {}", message)
}

pub fn error(message: &str) -> String {
  format!("error | {}", message)
}

pub(crate) fn from_str<F: std::str::FromStr>(s: &str) -> Result<F, String> {
  s.parse::<F>().map_err(|_| error("invalid format"))
}

#[macro_export]
macro_rules! test {
  ($s: stmt) => {{
    use std::time::Instant;
    use cli::util::log;

    let timer = Instant::now();
    { $s }
    let time = timer.elapsed();
    println!("{} {}.{:03} sec", log("execution time: "), time.as_secs(), time.subsec_nanos() / 1_000_000);
  }};
}