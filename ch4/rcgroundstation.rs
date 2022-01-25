use std::rc::Rc;

#[derive(Debug)]
struct GroundStation {}

fn main() {
    let base = Rc::new(GroundStation {});   // wrapped instance
    println!("{:?}", base);
}