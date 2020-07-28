extern crate serde;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum Destination {
  IR, // Registers as integer
  UR, // Registers as unsigned integer
  FR, // Registers as float
  BR, // Registers as boolean
}

impl Destination {
  pub(crate) fn to_string(&self, s: &str) -> String {
    use self::Destination::*;
    match self {
      IR => format!("self.registers[instruction.1 as usize] = {}", s),
      UR => format!("*(self.registers.get_mut(instruction.1 as usize).unwrap() as *mut i32 as *mut u32) = {}", s),
      BR => format!("self.registers[instruction.1 as usize] = if {} {{ 1 }} else {{ 0 }}", s),
      _ => String::new(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum Source {
  IR, // Registers as integer
  UR, // Registers as unsigned integer
  II, // Immediate as integer
  UI, // Immediate as unsigned integer
}

impl Source {
  pub(crate) fn to_string(&self, n: u8) -> String {
    use self::Source::*;
    match self {
      IR => format!("self.registers[instruction.{} as usize]", n),
      UR => format!("(self.registers[instruction.{} as usize] as u32)", n),
      II => format!("(instruction.{} as i32)", n),
      UI => format!("(instruction.{} as u32)", n),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
  instructions: HashMap<String, u8>,
  sys: HashMap<u8, String>,
  calc: HashMap<u8, (String, Destination, Source, Source)>,
  goto: HashMap<u8, Option<String>>,
}

pub(crate) fn read_config() -> std::io::Result<Config> {
  let file = File::open("builder/config.yml")?;
  let reader = BufReader::new(file);
  let programs = serde_yaml::from_reader(reader).expect("error | InvalidYamlFile");
  Ok(programs)
}

fn main() -> std::io::Result<()> {
  let config = read_config()?;
  let writer = BufWriter::new(File::create("cli/src/compile.rs")?);
  let mut indenter = Indenter::new(writer);
  indenter.writeln(r##"
//! automatically generated

use std::collections::HashMap;
use super::Process;
use super::fio::{Script, load_native};
use super::util::{warning, log};

pub(crate) fn compile_auto_gen(mnemonic: &str) -> u8 {
  match mnemonic {
"##.trim())?.indent().indent();
  for (mnemonic, opcode) in config.instructions {
    indenter.writeln(&format!("\"{}\" => {},", mnemonic, opcode))?;
  }
  indenter.writeln(r##"
    _ => compile_auto_gen("nop"),
  }
}
impl Process {
  pub(crate) unsafe fn execute_auto_gen(&mut self, script: &Script) {
    use std::io::{stdin, BufRead, BufReader};

    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    let mut pc = 0;
    let mut modules: HashMap<String, libloading::Library> = HashMap::with_capacity(32);
    let mut natives: HashMap<u32, Box<unsafe extern "C" fn()>> = HashMap::with_capacity(32);

    // load
    for native in &script.natives {
      load_native(&mut modules, &mut natives, native.native_id, &native.symbol_name);
    }

    // execute
    loop {
      let instruction = script.body[pc as usize];
      match instruction.0 {
"##.trim())?.indent().indent();
  for (opcode, action) in config.sys {
    indenter.writeln(&format!(
      "{} => {},",
      opcode,
      decode(&action),
    ))?;
  }
  for (opcode, (operator, dst, src1, src2)) in config.calc {
    let rhs = dst.to_string(&format!("{} {} {}",
      src1.to_string(2),
      operator,
      src2.to_string(3)
    ));
    indenter.writeln(&format!(
      "{} => {},",
      opcode,
      rhs,
    ))?;
  }
  for (opcode, condition) in config.goto {
    indenter.writeln(
      &if let Some(condition) = condition {
        format!("{} => if self.registers[instruction.2 as usize] {} self.registers[instruction.3 as usize] {{ pc += instruction.1 as i32; continue; }},", opcode, condition)
      } else {
        format!("{} => {{ pc += instruction.1 as i32; continue; }},", opcode)
      },
    )?;
  }
  indenter.write(r##"
        _ => println!("{}", warning(&format!("no such instruction: {}", instruction.1))),
      }
    pc += 1;
    }
  }
}
"##.trim())?;
  Ok(())
}

pub struct Indenter<T: std::io::Write> {
  writer: BufWriter<T>,
  tab: usize,
}

impl <T: std::io::Write> Indenter<T> {
  pub fn new(writer: BufWriter<T>) -> Self {
    Self {
      writer: writer,
      tab: 0,
    }
  } 
  pub fn indent(&mut self) -> &mut Self {
    self.tab += 1;
    self
  }
  pub fn write(&mut self, message: &str) -> std::io::Result<&mut Self> {
    self.writer.write(format!("{}{}", "\t".repeat(self.tab), message).as_bytes())?;
    Ok(self)
  }
  pub fn writeln(&mut self, message: &str) -> std::io::Result<&mut Self> {
    self.writer.write(format!("{}{}\r\n", "\t".repeat(self.tab), message).as_bytes())?;
    Ok(self)
  }
  pub fn outdent(&mut self) -> &mut Self {
    self.tab -= 1;
    self
  }
}

pub(crate) fn decode(s: &str) -> String {
  s.replace("bp",        "self.base_pointer")
   .replace("operand0",  "instruction.1")
   .replace("dst",       "instruction.1")
   .replace("operand1",  "instruction.2")
   .replace("src1",      "instruction.2")
   .replace("operand2",  "instruction.3")
   .replace("src2",      "instruction.3")
   .replace("data",      "self.data")
   .replace("stack",     "self.stack")
   .replace("heap",      "self.heap")
   .replace("registers", "self.registers")
}