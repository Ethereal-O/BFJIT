mod bfexception;
mod bfparser;

fn main() {
    let a = crate::bfparser::frontend::parser::parse("+++-,.[++[--][,.]]");
    println!("{:?}", a);
    if a.is_err() {
        println!("{:?}", a.as_ref().unwrap_err());
        return;
    }
    let b = crate::bfparser::frontend::ir::transfer_to_ir(&a.unwrap());
    println!("{:?}", b.unwrap());
}
