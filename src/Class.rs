use super::Indention::*;
pub use super::Type::Type;
use super::Writer::qwriter;

#[derive(Clone)]
pub enum ClassVisType {
    Private,
    Public,
}

#[derive(Clone)]
pub struct ClassType {
    pub vis: ClassVisType,
    pub name: String,
    pub body: Vec<Box<dyn Type>>,
    pub indention: Indention,
}

impl Type for ClassType {
    fn emit(&mut self, result: &mut qwriter) {
        match self.vis {
            ClassVisType::Private => result.push_str(
                format!("{}.class private {} {{", self.indention.get(), self.name).as_str(),
            ),
            ClassVisType::Public => result.push_str(
                format!("{}.class public {} {{", self.indention.get(), self.name).as_str(),
            ),
        }

        self.indention.inc();

        for i in self.body.iter_mut() {
            i.emit(result);
        }

        self.indention.dec();

        result.push_str(format!("{}}}", self.indention.get()).as_str());
    }
}

pub fn emit_class(classt: &mut ClassType, result: &mut qwriter) {
    classt.emit(result);
}