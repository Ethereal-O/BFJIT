mod bfparser;
mod bftype;
mod bfvm;

fn test(str: &str) {
    let a = crate::bfparser::frontend::parser::parse(str);
    println!("{:?}", a);
    if a.is_err() {
        println!("{:?}", a.as_ref().unwrap_err());
        return;
    }
    let b = crate::bfparser::frontend::ir::transfer_to_ir(&a.unwrap());
    if b.is_err() {
        println!("{:?}", b.as_ref().unwrap_err());
        return;
    }
    println!("{:?}", b.as_ref().unwrap());
    let c = crate::bfparser::backend::codegen::gen_code(
        &b.unwrap(),
        crate::bftype::bfcate::bfcate::VMArchType::X64,
    );
    if c.is_err() {
        println!("{:?}", c.as_ref().unwrap_err());
        return;
    }
    println!("{:?}", c.as_ref().unwrap());
    let d= crate::bfvm::bfjit::vm::VMStruct::new(c.unwrap(), Box::new(std::io::stdin()), Box::new(std::io::stdout()));
    if d.is_err() {
        println!("{:?}", "VMStruct::new error");
        return;
    }
    d.unwrap().run();
}

fn main() {
    test("+++-,.[++[--][,.]]");
    // test("+++-,.[++[--],.]");
}
