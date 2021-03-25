mod foo;

fn main() {
    let x = 3;
    unsafe {
        println!("{}", foo::hello(x));
    }
}
