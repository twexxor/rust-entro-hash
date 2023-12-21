extern crate entro_hash;

use entro_hash::entro_hash::entro_hash;

fn main() {
    let input1 = "message1";
    let input2 = "message2";
    let mut digest = entro_hash(input1.as_bytes(), 0);

    digest = entro_hash(input2.as_bytes(), digest);
    println!("{:08x}", digest);
}
