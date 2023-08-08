pub mod vm {
    // use std::io::{Read, Write};
    // use std::path::Path;
    // use std::ptr;
    // use std::Lazy;

    // use dynasm::dynasm;
    // use dynasmrt::{DynasmApi, DynasmLabelApi};

    // pub struct VMStruct {
    //     code: dynasmrt::ExecutableBuffer,
    //     pc: dynasmrt::AssemblyOffset,
    //     memory: Box<[u8]>,
    //     input: Box<dyn Read>,
    //     output: Box<dyn Write>,
    // }

    // static VM_STRUCT:VMStruct = VMStruct {
    //     code: dynasmrt::ExecutableBuffer::new(4096).unwrap(),
    //     pc: dynasmrt::AssemblyOffset(0),
    //     memory: Box::new([0; 30000]),
    //     input: Lazy::new(|| Box::new(std::io::stdin())),
    //     output: Box::new(std::io::stdout()),
    // };

    // unsafe impl Sync for VMStruct {}


}
