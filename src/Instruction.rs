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
    fn get_name(&self) -> String {
        return self.name.clone();
    }
    fn emit(&mut self, result: &mut qwriter, _indention: &mut Indention) {
        result.push_str(format!("{} {}", self.name, self.operands.join(" ")).as_str());
    }
}
