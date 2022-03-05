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
