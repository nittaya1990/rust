//@ run-rustfix

fn main() {
    let x == 2; //~ ERROR unexpected `==`
    println!("x: {}", x)
}
