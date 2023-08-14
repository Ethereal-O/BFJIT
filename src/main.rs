mod bfparser;
mod bftype;
mod bfvm;
mod start;

fn main() {
    let args = start::parse();
    if args.is_err() {
        println!("parse args error");
        return;
    }
    start::start_all(args.unwrap());
}
