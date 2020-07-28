//! automatically generated

use std::collections::HashMap;
use super::Process;
use super::fio::{Script, load_native};
use super::util::{warning, log};

pub(crate) fn compile_auto_gen(mnemonic: &str) -> u8 {
  match mnemonic {
		"ncall" => 14,
		"eq1" => 58,
		"ushr1" => 63,
		"add0" => 32,
		"ifeq" => 74,
		"shr1" => 61,
		"div1" => 51,
		"not1" => 54,
		"nop" => 0,
		"add1" => 48,
		"and1" => 52,
		"xor1" => 55,
		"ifgt" => 72,
		"imm" => 15,
		"xor0" => 39,
		"new" => 80,
		"ushr0" => 47,
		"exit" => 1,
		"and0" => 36,
		"ge0" => 41,
		"ushl1" => 62,
		"ushl0" => 46,
		"sub1" => 49,
		"pop" => 17,
		"shr0" => 45,
		"mul0" => 34,
		"div0" => 35,
		"shl0" => 44,
		"or1" => 53,
		"not0" => 38,
		"mul1" => 50,
		"eq0" => 42,
		"gt1" => 56,
		"gt0" => 40,
		"ge1" => 57,
		"goto" => 64,
		"ifge" => 73,
		"set" => 81,
		"call" => 66,
		"get" => 82,
		"ret" => 67,
		"sub0" => 33,
		"shl1" => 60,
		"debug" => 2,
		"load" => 19,
		"push" => 16,
		"or0" => 37,
		"store" => 18,
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
				81 => self.heap[self.registers[instruction.1 as usize] as usize + instruction.2 as usize] = self.registers[instruction.3 as usize],
				15 => self.registers[instruction.1 as usize] = ((instruction.2 as i32) << 8) + (instruction.3 as i32),
				1 => { println!("{} {}", log("exit status"), instruction.1); break; },
				14 => natives[&(pc as u32)](),
				80 => { for _ in 0..instruction.2 { self.heap.push(0); } self.registers[instruction.1 as usize] = self.heap.len() as i32 - instruction.2 as i32; },
				16 => self.stack.push(self.registers[instruction.1 as usize]),
				17 => self.registers[instruction.1 as usize] = self.stack.pop().expect("STACK EMPTY"),
				18 => self.stack[(self.base_pointer + instruction.1 as i32) as usize] = self.registers[instruction.2 as usize],
				0 => println!("{}", warning("nop")),
				2 => { println!("{} {:?}", log("DATA:      "), self.data); println!("{} {:?}", log("HEAP:      "), self.heap); println!("{} {:?}", log("STACK:     "), self.stack); println!("{} {:?}", log("REGISTERS: "), self.registers); println!("{} {}",   log("BP:        "), self.base_pointer); println!("{} {}",   log("PC:        "), pc); loop { if let Some(Ok(_)) = lines.next() { break; } } },
				19 => self.registers[instruction.1 as usize] = self.stack[(self.base_pointer + instruction.2 as i32) as usize],
				82 => self.registers[instruction.1 as usize] = self.heap[self.registers[instruction.2 as usize] as usize + instruction.3 as usize],
				67 => { pc = self.stack.pop().expect("STACK EMPTY"); while self.stack.len() - 1 > self.base_pointer as usize { self.stack.pop().expect("STACK EMPTY"); } self.base_pointer = self.stack.pop().expect("STACK EMPTY"); continue; },
				66 => { self.stack.push(self.base_pointer); self.base_pointer = (self.stack.len() - 1) as i32; for _ in 0..instruction.2 { self.stack.push(0); } self.stack.push(pc + 1); pc += instruction.1 as i32; continue; },
				35 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] / self.registers[instruction.3 as usize],
				39 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] ^ self.registers[instruction.3 as usize],
				37 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] | self.registers[instruction.3 as usize],
				33 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] - self.registers[instruction.3 as usize],
				40 => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] > self.registers[instruction.3 as usize] { 1 } else { 0 },
				56 => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] > (instruction.3 as i32) { 1 } else { 0 },
				51 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] / (instruction.3 as i32),
				49 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] - (instruction.3 as i32),
				52 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] & (instruction.3 as i32),
				55 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] ^ (instruction.3 as i32),
				48 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] + (instruction.3 as i32),
				46 => *(self.registers.get_mut(instruction.1 as usize).unwrap() as *mut i32 as *mut u32) = (self.registers[instruction.2 as usize] as u32) << self.registers[instruction.3 as usize],
				61 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] >> (instruction.3 as i32),
				62 => *(self.registers.get_mut(instruction.1 as usize).unwrap() as *mut i32 as *mut u32) = (self.registers[instruction.2 as usize] as u32) << (instruction.3 as i32),
				58 => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] == (instruction.3 as i32) { 1 } else { 0 },
				34 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] * self.registers[instruction.3 as usize],
				45 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] >> self.registers[instruction.3 as usize],
				41 => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] >= self.registers[instruction.3 as usize] { 1 } else { 0 },
				47 => *(self.registers.get_mut(instruction.1 as usize).unwrap() as *mut i32 as *mut u32) = (self.registers[instruction.2 as usize] as u32) >> self.registers[instruction.3 as usize],
				63 => *(self.registers.get_mut(instruction.1 as usize).unwrap() as *mut i32 as *mut u32) = (self.registers[instruction.2 as usize] as u32) >> (instruction.3 as i32),
				60 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] << (instruction.3 as i32),
				42 => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] == self.registers[instruction.3 as usize] { 1 } else { 0 },
				50 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] * (instruction.3 as i32),
				53 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] | (instruction.3 as i32),
				32 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] + self.registers[instruction.3 as usize],
				57 => self.registers[instruction.1 as usize] = if self.registers[instruction.2 as usize] >= (instruction.3 as i32) { 1 } else { 0 },
				36 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] & self.registers[instruction.3 as usize],
				44 => self.registers[instruction.1 as usize] = self.registers[instruction.2 as usize] << self.registers[instruction.3 as usize],
				72 => if self.registers[instruction.2 as usize] > self.registers[instruction.3 as usize] { pc += instruction.1 as i32; continue; },
				64 => { pc += instruction.1 as i32; continue; },
				74 => if self.registers[instruction.2 as usize] == self.registers[instruction.3 as usize] { pc += instruction.1 as i32; continue; },
				73 => if self.registers[instruction.2 as usize] >= self.registers[instruction.3 as usize] { pc += instruction.1 as i32; continue; },
				_ => println!("{}", warning(&format!("no such instruction: {}", instruction.1))),
      }
    pc += 1;
    }
  }
}