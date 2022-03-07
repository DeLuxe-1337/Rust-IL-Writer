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
    pub rtype: String,
    pub indention: Indention,
    pub entrypoint: bool,
    pub managed: bool,
    pub is_static: bool,
    pub maxstack: usize,
    pub path: String,
}

impl Type for MethodType {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
    fn emit(&mut self, result: &mut qwriter, indention: &mut Indention) {
        let mut staticinsert = String::new();

        if self.is_static {
            staticinsert.push_str("static ");
        }

        match self.vis {
            MethodVisType::Private => result.push(
                format!(
                    "{}.method private {}{} {}({}) ",
                    indention.get(),
                    staticinsert,
                    self.rtype,
                    self.name,
                    self.arguments.join(", ")
                )
                .as_str(),
            ),
            MethodVisType::Public => result.push(
                format!(
                    "{}.method public {}{} {}({}) ",
                    indention.get(),
                    staticinsert,
                    self.rtype,
                    self.name,
                    self.arguments.join(", ")
                )
                .as_str(),
            ),
        }

        if self.managed {
            result.push("cil managed ");
        }

        result.push_str("{");

        indention.inc();

        if self.entrypoint {
            result.push_str(format!("{}.entrypoint", indention.get()).as_str());
        }

        result.push_str(format!("{}.maxstack {}", indention.get(), self.maxstack).as_str());

        let mut il_current: usize = 0;

        for i in self.body.iter_mut() {
            result.push(format!("{}IL_{}:    ", indention.get(), il_current).as_str());
            i.emit(result, indention);

            il_current += 1;
        }

        indention.dec();

        result.push_str(format!("{}}}", indention.get()).as_str());
    }
}

pub fn mscorlibfn(name: &str, returntype: &str, args: Vec<&str>) -> String {
    return format!("{1} [mscorlib]{0}({2})", name, returntype, args.join(", ")).to_string();
}