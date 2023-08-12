pub mod vm {
    use dynasmrt::relocations::Relocation;
    use dynasmrt::{Assembler, DynasmApi};
    use std::io::{Read, Write};

    use crate::bftype::bferror;

    const MEMORY_SIZE: usize = 30000;

    pub struct VMStruct {
        code: dynasmrt::ExecutableBuffer,
        pc: dynasmrt::AssemblyOffset,
        memory: Box<[u8]>,
        input: Box<dyn Read>,
        output: Box<dyn Write>,
    }

    pub trait VMInterface {
        type VMType;
    }

    impl VMInterface for VMStruct {
        type VMType = VMStruct;
    }

    impl VMStruct {
        pub unsafe fn get_byte(
            &mut self,
            ptr: *mut u8,
        ) -> Result<(), bferror::error::RuntimeError> {
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

        pub unsafe fn put_byte(
            &mut self,
            ptr: *const u8,
        ) -> Result<(), bferror::error::RuntimeError> {
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

        pub fn new<T: Relocation + std::fmt::Debug>(
            ops: Assembler<T>,
            input: Box<dyn Read>,
            output: Box<dyn Write>,
        ) -> Result<Self, bferror::error::RuntimeError> {
            let pc = ops.offset();
            let memory = vec![0; MEMORY_SIZE].into_boxed_slice();
            Ok(Self {
                code: ops.finalize().unwrap(),
                pc,
                memory,
                input,
                output,
            })
        }

        pub fn run(&mut self) {}
    }
}
