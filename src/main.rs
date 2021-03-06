mod Indention;

mod Type;

mod Class;
mod Instruction;
mod Method;

mod Writer;

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

    let mut program_class = writer.class(vec!["Base", "Program"], Class::ClassVisType::Public);

    let mut main_method = writer.method("Main", Method::MethodVisType::Public, program_class.clone());
    main_method.entrypoint = true;

    writer.op(
        "ldstr",
        vec!["\"Hello, World!\"".to_string()],
        &mut main_method,
    );

    writer.op(
        "call",
        vec![Method::mscorlibfn("System.Console::WriteLine", "void", vec!["string"])],
        &mut main_method,
    );

    writer.op("ret", vec![], &mut main_method);

    Method::new_local("based".to_string(), Method::LocalType::Int32, &mut main_method);
    Method::new_local("absolute".to_string(), Method::LocalType::Int32, &mut main_method);

    program_class.body.push(Box::new(main_method.clone()));

    Class::emit_class(&mut program_class, &mut writer.result);

    //end

    println!("{}", writer.result.result);

    writer.compile();
}
