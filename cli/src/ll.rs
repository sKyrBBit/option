//! link and load

use std::collections::HashMap;

struct Scanner {
  paths:      Vec<String>,
  symbols:    HashMap<String, u32>,
  references: HashMap<u32, String>,
  natives:    HashMap<u32, String>,
  text:       Vec<super::instruction::Instruction>,
}

impl Scanner {
  fn new() -> Self {
    Self {
      paths:      Vec::with_capacity(32),
      symbols:    HashMap::with_capacity(32),
      references: HashMap::with_capacity(32),
      natives:    HashMap::with_capacity(32),
      text:       Vec::with_capacity(32),
    }
  }
  fn scan(&mut self, path: &str) -> std::io::Result<()> {
    use super::fio::read_Script;

    let offset = self.text.len();
    // read
    let mut script = read_Script(path)?;
    for symbol in script.symbols {
      if self.symbols.contains_key(&symbol.name) {
        panic!(format!("{} is defined twice or more", &symbol.name));
      } else {
        self.symbols.insert(symbol.name, offset as u32 + symbol.base_address);
      }
    }
    for native in script.natives {
      self.natives.insert(offset as u32 + native.native_id, native.symbol_name);
    }
    self.text.append(&mut script.body);
    // scan
    for include in script.includes {
      if !self.paths.contains(&include) {
        self.scan(&include)?;
      }
    }
    // resolve
    for reference in script.references {
      let absolute_from_address = offset + reference.base_address as usize / 4;
      if let Some(address) = self.symbols.get(&reference.symbol_name) {
        let absolute_to_address = *address as usize;
        let relative_to_address =
          if absolute_to_address >= absolute_from_address { (absolute_to_address - absolute_from_address) as i8 }
          else { -((absolute_from_address - absolute_to_address) as i8) };
        match reference.base_address % 4 {
          1 => self.text[absolute_from_address].1 = relative_to_address,
          2 => self.text[absolute_from_address].2 = relative_to_address,
          3 => self.text[absolute_from_address].3 = relative_to_address,
          _ => unreachable!(),
        };
      } else {
        panic!(format!("{} is undefined", &reference.symbol_name));
      }
    }
    
    Ok(())
  }
}

pub fn link(path: &str) -> std::io::Result<()> {
  let mut scanner = Scanner::new();
  scanner.scan(path)?;

  use super::fio::{read_Script, write_Script};
  let mut script = read_Script(path)?;
  script.symbols = scanner.symbols
    .into_iter()
    .map(|(name, base_address)| super::fio::Symbol {
      name: name,
      base_address: base_address,
    })
    .collect();
  script.includes.clear();
  script.references = scanner.references
    .into_iter()
    .map(|(base_address, symbol_name)| super::fio::Reference {
      base_address: base_address,
      symbol_name: symbol_name,
    })
    .collect();
  script.natives = scanner.natives
    .into_iter()
    .map(|(native_id, symbol_name)| super::fio::Native {
      native_id: native_id,
      symbol_name: symbol_name,
    })
    .collect();
  script.body = scanner.text;
  write_Script(path, &script)?;
  Ok(())
}