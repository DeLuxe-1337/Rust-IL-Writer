use super::Class::*;
use super::Indention::*;
use super::Instruction::*;
use super::Method::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

#[derive(Clone)]
pub struct qwriter {
    pub result: String,
}

impl qwriter {
    //Not necessary but I want to write to a file when done so i just made a little crappy writer
    pub fn new() -> qwriter {
        qwriter {
            result: String::new(),
        }
    }
    pub fn push_str(&mut self, s: &str) {
        self.result.push_str(format!("{}\n", s).as_str());
    }
    pub fn push(&mut self, s: &str) {
        self.result.push_str(s);
    }
}

pub struct IL {
    classes: HashMap<String, ClassType>,
    indention: Indention,
    pub result: qwriter,
}

impl IL {
    pub fn new() -> IL {
        IL {
            classes: HashMap::new(),
            indention: Indention::new(),
            result: qwriter::new(),
        }
    }
    pub fn newln(&mut self) {
        self.result.push_str("");
    }
    pub fn comment(&mut self, comment: &str) {
        self.result.push_str(format!("// {}", comment).as_str());
    }
    pub fn assemblymanifest(&mut self, manifest: &str, body: Vec<(&str, &str, bool)>) {
        self.result
            .push_str(format!("{}.assembly {} {{", self.indention.get(), manifest).as_str());

        self.indention.inc();

        for i in body {
            if i.2 {
                self.result
                    .push_str(format!("{}{} = {}", self.indention.get(), i.0, i.1).as_str());
            } else {
                self.result
                    .push_str(format!("{}{} {}", self.indention.get(), i.0, i.1).as_str());
            }
        }

        self.indention.dec();

        self.result
            .push_str(format!("{}}}", self.indention.get()).as_str());
    }
    pub fn class(&mut self, class_unjoined: Vec<&str>, class_type: ClassVisType) -> ClassType {
        let class = class_unjoined.join(".");
        let ctype = ClassType {
            vis: class_type,
            name: String::from(class.clone()),
            body: Vec::new(),
            indention: self.indention.clone(),
        };

        self.classes.insert(String::from(class), ctype.clone());

        return ctype;
    }
    pub fn method(&mut self, name: &str, visibility: MethodVisType, class: ClassType) -> MethodType {
        let ctype = MethodType {
            vis: visibility,
            name: String::from(name),
            body: Vec::new(),
            indention: self.indention.clone(),
            arguments: Vec::new(),
            rtype: String::from("void"),
            entrypoint: false,
            managed: true,
            is_static: true,
            maxstack: 8,
            path: format!("{}::{}", class.name, name),
            locals: HashMap::new(),
            localindex: 0,
        };

        return ctype;
    }
    pub fn op(&mut self, op: &str, arg: Vec<String>, meth: &mut MethodType) {
        meth.body.push(Box::new(Instruction {
            name: op.to_string(),
            operands: arg,
            indention: self.indention.clone(),
        }));
    }
    pub fn compile(&mut self) {
        if Path::new("out.il").exists() {
            std::fs::remove_file("out.il").expect("Could not remove file");
        }

        let mut file = File::create("out.il").expect("Unable to create file");
        file.write_all(self.result.result.as_bytes()).expect("Unable to write to file");

        Command::new("ilasm-win\\ilasm")
            .arg("out.il")
            .spawn()
            .expect("Failed to execute ilasm");

        println!("Ilasm executed.");
    }
}
