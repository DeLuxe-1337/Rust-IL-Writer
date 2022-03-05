use super::Indention::*;
use super::Writer::qwriter;

use super::Type::*;

#[derive(Clone)]
pub struct Instruction {
    pub name: String,
    pub operands: Vec<String>,
    pub indention: Indention,
}

impl Type for Instruction {
    fn emit(&mut self, result: &mut qwriter, indention: &mut Indention) {
        result.push_str(format!("{} {}", self.name, self.operands.join(" ")).as_str());
    }
}
