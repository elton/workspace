use add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} puls one is {}",
        num,
        add_one::add_one(num)
    );
}
