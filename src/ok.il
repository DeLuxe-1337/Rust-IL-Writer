.assembly extern mscorlib
{
  .publickeytoken = (B7 7A 5C 56 19 34 E0 89 )
  .ver 4:0:0:0
}

.module BaseCompiled.exe

.class private auto ansi beforefieldinit BaseCompiled.Program
       extends [mscorlib]System.Object
{
  .method private hidebysig static void  Main(string[] args) cil managed
  // SIG: 00 01 01 1D 0E
  {
    .entrypoint
    // Method begins at RVA 0x2050
    // Code size       24 (0x18)
    .maxstack  8
    IL_0000:  /* 00   |                  */ nop
    IL_0001:  /* 72   | (70)000001       */ ldstr      "Hello world"
    IL_0006:  /* 28   | (0A)00000F       */ call       void [mscorlib]System.Console::WriteLine(string)
    IL_000b:  /* 00   |                  */ nop
    IL_000c:  /* 72   | (70)000019       */ ldstr      "hahaa"
    IL_0011:  /* 28   | (0A)00000F       */ call       void [mscorlib]System.Console::WriteLine(string)
    IL_0016:  /* 00   |                  */ nop
    IL_0017:  /* 2A   |                  */ ret
  } // end of method Program::Main

  .method public hidebysig specialname rtspecialname 
          instance void  .ctor() cil managed
  // SIG: 20 00 01
  {
    // Method begins at RVA 0x2069
    // Code size       8 (0x8)
    .maxstack  8
    IL_0000:  /* 02   |                  */ ldarg.0
    IL_0001:  /* 28   | (0A)000010       */ call       instance void [mscorlib]System.Object::.ctor()
    IL_0006:  /* 00   |                  */ nop
    IL_0007:  /* 2A   |                  */ ret
  } // end of method Program::.ctor

} // end of class BaseCompiled.Program


// =============================================================

// *********** DISASSEMBLY COMPLETE ***********************
// WARNING: Created Win32 resource file C:\Users\rragt\Downloads\dmp.res
