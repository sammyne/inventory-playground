use hello::Hello;
use world::*;

fn main() {
    for hello in inventory::iter::<Hello> {
        println!("hello from '{}'", hello.src);
    }

    println!("done");
}
