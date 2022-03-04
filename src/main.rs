mod Indention;

mod Type;

mod Class;
mod Instruction;
mod Method;

mod Writer;

use Class::{ClassType, ClassVisType};
use Instruction::*;

fn main() {
    let mut writer = Writer::IL::new();

    //start of il program
    writer.comment("This is my IL writer for my new language!");
    writer.newln();

    writer.assemblymanifest(
        "extern mscorlib",
        vec![
            (".publickeytoken", "(B7 7A 5C 56 19 34 E0 89 )", true),
            (".ver", "4:0:0:0", false),
        ],
    );

    writer.newln();

    writer.assemblymanifest(
        "Base",
        vec![
            (".hash algorithm", "0x00008004", false),
            (".ver", "1:0:0:0", false),
        ],
    );

    writer.newln();

    let mut base = writer.class("Base.Program", Class::ClassVisType::Public);

    let mut mainb = writer.method("Main", Method::MethodVisType::Public);
    mainb.is_entrypoint = true;

    writer.op("ldstr", vec!["\"Hello, World!\"".to_string()], &mut mainb);

    writer.op(
        "call",
        vec!["void [mscorlib]System.Console::WriteLine(string)".to_string()],
        &mut mainb,
    );

    writer.op("ret", vec![], &mut mainb);

    base.body.push(Box::new(mainb));

    base.emit(&mut writer.result);

    //end

    println!("{}", writer.result.result);

    writer.compile();
}
