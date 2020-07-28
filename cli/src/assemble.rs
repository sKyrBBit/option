// TODO to wc
// pub(crate) fn symbol(body: &str) {
//   body.split(" ");
// }
// pub(crate) fn label(body: &str) {
//   body.split(" ");
// }
// pub(crate) fn goto(body: &str) {
//   body.split(" ");
// }

pub fn assemble(path: &str) -> std::io::Result<()> {
  use super::fio::{Script, read_ScriptM, write_Script};
  use super::instruction::Instruction;
  use super::compile::compile_auto_gen;

  let scriptm = read_ScriptM(path)?;
  let body: Vec<Instruction> = scriptm.body
    .into_iter()
    .map(|(mnemonic, operand0, operand1, operand2)| {
      let opcode = compile_auto_gen(&mnemonic);
      (opcode, operand0, operand1, operand2)
    })
    .collect();
  
  write_Script(path, &Script {
    symbols: scriptm.symbols,
    includes: scriptm.includes,
    references: scriptm.references,
    natives: scriptm.natives,
    body: body
  })?;
  Ok(())
}