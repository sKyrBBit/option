extern crate serde;
extern crate serde_yaml;

use serde::{Deserialize, Serialize};

pub mod fio;
pub mod util;
pub mod instruction;
pub mod compile;
pub mod assemble;
pub mod ll;

// TODO impl thread

#[derive(Serialize, Deserialize)]
pub struct ProcessSerde {
  data:         Vec<i32>,
  heap:         Vec<i32>,
  stack:        Vec<i32>,
  registers:    Vec<i32>,
  base_pointer: i32,
}
impl ProcessSerde {
  pub(crate) fn convert(self) -> Process { // TODO optional reservation
    Process {
      data:         Box::from(self.data.as_slice()),
      heap:         self.heap,
      stack:        self.stack,
      registers:    Box::from(self.registers.as_slice()),
      base_pointer: self.base_pointer,
    }
  }
}

pub struct Process {
  data:         Box<[i32]>,
  heap:         Vec<i32>,
  stack:        Vec<i32>,
  registers:    Box<[i32]>,
  base_pointer: i32,
}

impl Process {
  pub fn new() -> Self {
    Self {
      data:         Box::from([0; 0x100]),
      heap:         Vec::new(),
      stack:        Vec::new(),
      registers:    Box::from([0; 0x100]),
      base_pointer: 0,
    }
  }
  pub(crate) fn convert(self) -> ProcessSerde {
    ProcessSerde {
      data:         Vec::from(self.data),
      heap:         self.heap,
      stack:        self.stack,
      registers:    Vec::from(self.registers),
      base_pointer: self.base_pointer
    }
  }
  pub fn prompt(&mut self) -> std::io::Result<()> {
    use self::instruction::check_instruction_format;
    use self::util::print;

    self::util::prompt_mut(">", |line|
      match &check_instruction_format(line) {
        Ok(instruction) => self.execute(instruction),
        Err(e) => print(e).unwrap(),
      }
    )
  }
  
  pub fn run_script(&mut self, path: &str) -> std::io::Result<()> {
    let script = self::fio::read_Script(path)?;
    unsafe { self.execute_auto_gen(&script) }
    Ok(())
  }
}