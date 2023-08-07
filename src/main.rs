mod bfexception;
mod bfparser;
mod bfvm;

fn main() {
    let a = crate::bfparser::frontend::parser::parse("+++-,.[++[--][,.]]");
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
    println!("{:?}", b.unwrap());
}
