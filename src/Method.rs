use super::Indention::*;
use super::Writer::qwriter;

use super::Type::*;

#[derive(Clone)]
pub enum MethodVisType {
    Private,
    Public,
}

#[derive(Clone)]
pub struct MethodType {
    pub vis: MethodVisType,
    pub name: String,
    pub body: Vec<Box<dyn Type>>,
    pub arguments: Vec<String>,
    pub rettype: String,
    pub indention: Indention,
    pub is_entrypoint: bool,
    pub is_managed: bool,
    pub is_static: bool,
    pub maxstack: usize,
}

impl Type for MethodType {
    fn emit(&mut self, result: &mut qwriter) {
        self.indention.inc();

        let mut staticinsert = String::new();

        if self.is_static {
            staticinsert.push_str("static ");
        }

        match self.vis {
            MethodVisType::Private => result.push(
                format!(
                    "{}.method private {}{} {}({}) ",
                    self.indention.get(),
                    staticinsert,
                    self.rettype,
                    self.name,
                    self.arguments.join(", ")
                )
                .as_str(),
            ),
            MethodVisType::Public => result.push(
                format!(
                    "{}.method public {}{} {}({}) ",
                    self.indention.get(),
                    staticinsert,
                    self.rettype,
                    self.name,
                    self.arguments.join(", ")
                )
                .as_str(),
            ),
        }

        if self.is_managed {
            result.push("cil managed ");
        }

        result.push_str("{");

        self.indention.inc();

        if self.is_entrypoint {
            result.push_str(format!("{}.entrypoint", self.indention.get()).as_str());
        }

        result.push_str(format!("{}.maxstack {}", self.indention.get(), self.maxstack).as_str());

        let mut il_current: usize = 0;

        for i in self.body.iter_mut() {
            result.push(format!("{}IL_{}:    ", self.indention.get(), il_current).as_str());
            i.emit(result);

            il_current += 1;
        }

        self.indention.dec();

        result.push_str(format!("{}}}", self.indention.get()).as_str());
    }
}
