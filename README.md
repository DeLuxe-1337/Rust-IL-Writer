# Rust-IL-Writer

I am just writing an IL writer for Rust so I can use it on my future programming languages.

I do plan to make it even easier to use, right now it's on github so I don't lose source or anything like that.

This does use 'Microsoft (R) .NET Framework IL Assembler' to assemble the il output.

example: 

```let mut writer = Writer::IL::new();

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

    base.emit(&mut writer.result);```
    
----------------------------------------------------------------------------------------------------------------------------------------------
output result below
----------------------------------------------------------------------------------------------------------------------------------------------

// This is my IL writer for my new language!

.assembly extern mscorlib {
    .publickeytoken = (B7 7A 5C 56 19 34 E0 89 )
    .ver 4:0:0:0
}

.assembly Base {
    .hash algorithm 0x00008004
    .ver 1:0:0:0
}

.class public Base.Program {
    .method public static void Main() cil managed {
        .entrypoint
        .maxstack 8
        IL_0:    ldstr "Hello, World!"
        IL_1:    call void [mscorlib]System.Console::WriteLine(string)
        IL_2:    ret 
    }
}
