pub mod codegen {
    use dynasm::dynasm;
    use dynasmrt::relocations::Relocation;
    use dynasmrt::x64::X64Relocation;
    use dynasmrt::DynasmLabelApi;
    use dynasmrt::{Assembler, DynasmApi};

    use crate::bfparser::frontend::ir::BFIR;
    use crate::bftype::bfcate::bfcate::VMArchType;
    use crate::bftype::bferror;
    use crate::bfvm::bfjit::vm;

    fn gen_x64_code_normal(
        irs: &Vec<BFIR>,
        mut ops: Box<Assembler<X64Relocation>>,
    ) -> Box<Assembler<X64Relocation>> {
        dynasm!(ops
            ; push rax
            ; mov r12, rdi   // save this
            ; mov r13, rsi   // save memory_start
            ; mov r14, rdx   // save memory_end
            ; mov rcx, rsi   // ptr = memory_start
        );
        let mut index = 0;
        let len = irs.len();
        while index < len {
            match &irs[index] {
                BFIR::Add(x) => {
                    index += 1;
                    dynasm!(ops
                        ; add BYTE [rcx], *x as i8    // *ptr += x
                    );
                }
                BFIR::Sub(x) => {
                    index += 1;
                    dynasm!(ops
                        ; sub BYTE [rcx], *x as i8    // *ptr -= x
                    );
                }
                BFIR::MoveLeft(x) => {
                    index += 1;
                    dynasm!(ops
                        ; sub rcx, *x as i32     // ptr -= x
                        ; jc  ->overflow        // jmp if overflow
                        ; cmp rcx, r13          // ptr - memory_start
                        ; jb  ->overflow        // jmp if ptr < memory_start
                    );
                }
                BFIR::MoveRight(x) => {
                    index += 1;
                    dynasm!(ops
                        ; add rcx, *x as i32     // ptr += x
                        ; jc  ->overflow        // jmp if overflow
                        ; cmp rcx, r14          // ptr - memory_end
                        ; jnb ->overflow        // jmp if ptr >= memory_end
                    );
                }
                BFIR::Input => {
                    index += 1;
                    dynasm!(ops
                        ; mov  r15, rcx         // save ptr
                        ; mov  rdi, r12
                        ; mov  rsi, rcx         // arg0: this, arg1: ptr
                        ; mov  rax, QWORD vm::VMStruct::get_byte as _
                        ; call rax              // getbyte(this, ptr)
                        ; test rax, rax
                        ; jnz  ->io_error       // jmp if rax != 0
                        ; mov  rcx, r15         // recover ptr
                    )
                }
                BFIR::Output => {
                    index += 1;
                    dynasm!(ops
                        ; mov  r15, rcx         // save ptr
                        ; mov  rdi, r12
                        ; mov  rsi, rcx         // arg0: this, arg1: ptr
                        ; mov  rax, QWORD vm::VMStruct::put_byte as _
                        ; call rax              // putbyte(this, ptr)
                        ; test rax, rax
                        ; jnz  ->io_error       // jmp if rax != 0
                        ; mov  rcx, r15         // recover ptr
                    )
                }
                BFIR::Loop(x) => {
                    index += 1;
                    let left = ops.new_dynamic_label();
                    let right = ops.new_dynamic_label();
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jz => right       // jmp if *ptr == 0
                        ; => left
                    );
                    ops = gen_x64_code_normal(&x.borrow(), ops);
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jnz => left       // jmp if *ptr != 0
                        ; => right
                    );
                }
            }
        }
        return ops;
    }

    fn gen_x64_code(
        irs: &Vec<BFIR>,
    ) -> Result<Assembler<impl Relocation + std::fmt::Debug>, bferror::error::RuntimeError> {
        let ops = dynasmrt::x64::Assembler::new();
        if ops.is_err() {
            return Err(bferror::error::RuntimeError {
                index: 1,
                kind: bferror::error::RuntimeErrorKind::Memory,
            });
        }
        let mut ops_ptr = Box::new(ops.unwrap());
        ops_ptr = gen_x64_code_normal(&irs, ops_ptr);
        return Ok(*ops_ptr);
    }

    pub fn gen_code(
        irs: &Vec<BFIR>,
        vm_arch_type: VMArchType,
    ) -> Result<Assembler<impl Relocation + std::fmt::Debug>, bferror::error::RuntimeError> {
        match vm_arch_type {
            VMArchType::X64 => {
                return gen_x64_code(irs);
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
