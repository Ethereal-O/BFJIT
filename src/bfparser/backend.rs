pub mod codegen {
    use dynasmrt::relocations::Relocation;
    use dynasmrt::Assembler;

    use crate::bfparser::frontend::ir::BFIR;
    use crate::bftype::bfcate::bfcate::VMArchType;
    use crate::bftype::bferror;

    fn gen_x64_code(
        irs: &Vec<BFIR>,
    ) -> Result<Assembler<impl Relocation + std::fmt::Debug>, bferror::error::RuntimeError> {
        let mut ops = dynasmrt::x64::Assembler::new();
        return Ok(ops.unwrap());
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
