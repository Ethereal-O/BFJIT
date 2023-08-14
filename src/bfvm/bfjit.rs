pub mod vm {
    use dynasmrt::relocations::Relocation;
    use dynasmrt::{Assembler, AssemblyOffset};
    use std::io::{Read, Write};
    use std::ptr;

    use crate::bftype::bfcate;
    use crate::bftype::bfcate::bfcate::VMArchType;
    use crate::bftype::bferror;

    const MEMORY_SIZE: usize = 30000;

    type RawFnX64 = unsafe extern "sysv64" fn(
        this: *mut VMStruct,
        memory_start: *mut u8,
        memory_end: *const u8,
    ) -> *mut bferror::error::RuntimeError;

    pub struct VMStruct {
        code: dynasmrt::ExecutableBuffer,
        pc: dynasmrt::AssemblyOffset,
        memory: Box<[u8]>,
        input: Box<dyn Read>,
        output: Box<dyn Write>,
        vm_arch_type: bfcate::bfcate::VMArchType,
        add_48: bool,
    }

    fn to_raw<R, T>(ptr: T) -> *mut R {
        Box::into_raw(Box::new(ptr)) as *mut R
    }

    impl VMStruct {
        pub unsafe extern "sysv64" fn put_x64_byte(
            this: *mut Self,
            byte_ptr: *mut u8,
        ) -> *mut bferror::error::RuntimeError {
            let mut buf = [0_u8];
            let this = &mut *this;
            match this.input.read(&mut buf) {
                Ok(1) => {
                    let byte = if this.add_48 { buf[0] - 48 } else { buf[0] };
                    *byte_ptr = byte;
                    return ptr::null_mut();
                }
                _ => {
                    return to_raw::<bferror::error::RuntimeError, _>(
                        bferror::error::RuntimeError {
                            index: 1,
                            kind: bferror::error::RuntimeErrorKind::IO,
                        },
                    )
                }
            }
        }

        pub unsafe extern "sysv64" fn get_x64_byte(
            this: *mut Self,
            byte_ptr: *const u8,
        ) -> *mut bferror::error::RuntimeError {
            let this = &mut *this;
            let byte = if this.add_48 {
                *byte_ptr + 48
            } else {
                *byte_ptr
            };
            match this.output.write(&[byte]) {
                Ok(1) => return ptr::null_mut(),
                _ => {
                    return to_raw::<bferror::error::RuntimeError, _>(
                        bferror::error::RuntimeError {
                            index: 1,
                            kind: bferror::error::RuntimeErrorKind::IO,
                        },
                    )
                }
            }
        }

        pub unsafe fn overflow_error() -> *mut bferror::error::RuntimeError {
            to_raw(bferror::error::RuntimeError {
                index: 1,
                kind: bferror::error::RuntimeErrorKind::Memory,
            })
        }

        pub fn new<T: Relocation + std::fmt::Debug>(
            ops: Assembler<T>,
            input: Box<dyn Read>,
            output: Box<dyn Write>,
            vm_arch_type: bfcate::bfcate::VMArchType,
            add_48: bool,
        ) -> Result<Self, bferror::error::RuntimeError> {
            let pc = AssemblyOffset(0);
            let memory = vec![0; MEMORY_SIZE].into_boxed_slice();
            Ok(Self {
                code: ops.finalize().unwrap(),
                pc,
                memory,
                input,
                output,
                vm_arch_type,
                add_48,
            })
        }

        fn run_x64(&mut self) -> Result<(), bferror::error::RuntimeError> {
            let raw_fn: RawFnX64 = unsafe { std::mem::transmute(self.code.ptr(self.pc)) };

            let this: *mut Self = self;
            let memory_start = self.memory.as_mut_ptr();
            let memory_end = unsafe { memory_start.add(MEMORY_SIZE) };

            let ret: *mut bferror::error::RuntimeError =
                unsafe { raw_fn(this, memory_start, memory_end) };

            if ret.is_null() {
                Ok(())
            } else {
                Err(*unsafe { Box::from_raw(ret) })
            }
        }

        pub fn run(&mut self) -> Result<(), bferror::error::RuntimeError> {
            match self.vm_arch_type {
                VMArchType::X64 => {
                    return self.run_x64();
                }
                _ => {
                    return Err(bferror::error::RuntimeError {
                        index: 1,
                        kind: bferror::error::RuntimeErrorKind::Unknown,
                    })
                }
            }
        }
    }
}
