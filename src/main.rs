use playground::{algorithms, structures};

fn main() {
    println!("Hello Playground!");
    println!("{:?}", structures::vector::Vec::new());
    println!("{:?}", algorithms::search::binary_while(vec!(1, 2, 3, 4), 3).unwrap());
}
