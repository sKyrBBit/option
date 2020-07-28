use super::Process;
use super::util::{log, warning, error, from_str};

pub(crate) type InstructionM = (String, i8, i8, i8);
pub(crate) type Instruction = (u8, i8, i8, i8);

pub(crate) fn check_instruction_format(line: &str) -> Result<Instruction, String> {
  let words: Vec<&str> = line.split(' ').collect();
  if words.len() != 4 { return Err(error("invalid format")); }
  let opcode:   u8 = from_str(words[0])?;
  let operand0: i8 = from_str(words[1])?;
  let operand1: i8 = from_str(words[2])?;
  let operand2: i8 = from_str(words[3])?;
  Ok((opcode, operand0, operand1, operand2))
}

impl Process {
  pub(crate) fn execute(&mut self, instruction: &Instruction) {
    match instruction.0 { // TODO generate automatically
      0x00 /* nop   */ => println!("{}", warning("nop")),
      0x01 /* exit  */ => { println!("{} {}", log("exit status"), instruction.1); },
      0x02 /* debug */ => {
        println!("{} {:?}", log("data:      "), self.data);
        println!("{} {:?}", log("heap:      "), self.heap);
        println!("{} {:?}", log("stack:     "), self.stack);
        println!("{} {:?}", log("registers: "), self.registers)
      },
      0x0f /* imm   */ => self.registers[instruction.1 as usize] = (instruction.2 as i32) << 8 + instruction.3 as i32,
      0x10 /* push  */ => self.stack.push(self.registers[instruction.1 as usize]),
      0x11 /* pop   */ => self.registers[instruction.1 as usize] = self.stack.pop().expect("stack empty"),

      0x20 /* add   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] + self.registers[instruction.3 as usize],
      0x21 /* sub   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] - self.registers[instruction.3 as usize],
      0x22 /* mul   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] * self.registers[instruction.3 as usize],
      0x23 /* div   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] / self.registers[instruction.3 as usize],
      0x24 /* and   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] & self.registers[instruction.3 as usize],
      0x25 /* or    */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] | self.registers[instruction.3 as usize],
      0x26 /* not   */ => self.registers[instruction.1 as usize] = !self.registers[instruction.2 as usize],
      0x27 /* xor   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] ^ self.registers[instruction.3 as usize],
      0x28 /* gt    */ => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] >  self.registers[instruction.3 as usize] {1}
                                                                     else {0},
      0x29 /* ge    */ => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] >= self.registers[instruction.3 as usize] {1}
                                                                     else {0},
      0x2a /* eq    */ => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] == self.registers[instruction.3 as usize] {1}
                                                                     else {0},
      0x2c /* shl   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] << self.registers[instruction.3 as usize],
      0x2d /* shr   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] >> self.registers[instruction.3 as usize],

      0x30 /* add   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] + instruction.3 as i32,
      0x31 /* sub   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] - instruction.3 as i32,
      0x32 /* mul   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] * instruction.3 as i32,
      0x33 /* div   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] / instruction.3 as i32,
      0x34 /* and   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] & instruction.3 as i32,
      0x35 /* or    */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] | instruction.3 as i32,

      0x37 /* xor   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] ^ instruction.3 as i32,
      0x38 /* gt    */ => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] >  instruction.3 as i32 {1}
                                                                     else {0},
      0x39 /* ge    */ => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] >= instruction.3 as i32 {1}
                                                                     else {0},
      0x3a /* eq    */ => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] == instruction.3 as i32 {1}
                                                                     else {0},
      0x3c /* shl   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] << instruction.3 as i32,
      0x3d /* shr   */ => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] >> instruction.3 as i32,
      
      0xff /* show  */ => println!("{}", self.registers[instruction.1 as usize]),
      _ => println!("{}", warning("no such instruction")),
    }
  }
}