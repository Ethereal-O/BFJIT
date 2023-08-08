pub mod vm {
    use std::io::{Read, Write};
    use std::path::Path;
    use std::ptr;

    use dynasm::dynasm;
    use dynasmrt::{DynasmApi, DynasmLabelApi};

    use crate::bfexception::bferror;

    const MEMORY_SIZE: usize = 30000;

    pub struct VMStruct {
        code: dynasmrt::ExecutableBuffer,
        pc: dynasmrt::AssemblyOffset,
        memory: Box<[u8]>,
        input: Box<dyn Read>,
        output: Box<dyn Write>,
    }

    trait VMInterface {
        type VMType;
        fn new(
            code: dynasmrt::ExecutableBuffer,
            pc: dynasmrt::AssemblyOffset,
            input: Box<dyn Read>,
            output: Box<dyn Write>,
        ) -> Result<Self::VMType, bferror::error::RuntimeError>;
        unsafe fn get_byte(&mut self, ptr: *mut u8) -> Result<(), bferror::error::RuntimeError>;
        unsafe fn put_byte(&mut self, ptr: *const u8) -> Result<(), bferror::error::RuntimeError>;
        fn run(&mut self);
    }

    impl VMInterface for VMStruct {
        type VMType = VMStruct;
        fn new(
            code: dynasmrt::ExecutableBuffer,
            pc: dynasmrt::AssemblyOffset,
            input: Box<dyn Read>,
            output: Box<dyn Write>,
        ) -> Result<Self, bferror::error::RuntimeError> {
            let memory = vec![0; MEMORY_SIZE].into_boxed_slice();
            Ok(Self {
                code,
                pc,
                memory,
                input,
                output,
            })
        }

        unsafe fn get_byte(&mut self, ptr: *mut u8) -> Result<(), bferror::error::RuntimeError> {
            let mut buf = [0_u8];
            match self.input.read(&mut buf) {
                Ok(1) => {
                    *ptr = buf[0];
                    return Ok(());
                }
                _ => {
                    return Err(bferror::error::RuntimeError {
                        index: 1,
                        kind: bferror::error::RuntimeErrorKind::IO,
                    })
                }
            }
        }

        unsafe fn put_byte(&mut self, ptr: *const u8) -> Result<(), bferror::error::RuntimeError> {
            match self.output.write(&[*ptr]) {
                Ok(1) => return Ok(()),
                _ => {
                    return Err(bferror::error::RuntimeError {
                        index: 1,
                        kind: bferror::error::RuntimeErrorKind::IO,
                    })
                }
            }
        }

        fn run(&mut self) {
            let mut ops = dynasmrt::x64::Assembler::new();
            // dynasm!(ops;push rax
            //     ;ops
            //     ;ops);
            // std::mem::transmute
            todo!()
        }
    }
}
