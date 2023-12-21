# Entro Hash
## Description
Entro Hash is a 32-bit, non-cryptographic hashing algorithm.

## Code Example
The following code demonstrates an example implementation included in this package.

``` rust
extern crate entro_hash;

use entro_hash::entro_hash::entro_hash;

fn main() {
    let input1 = "message1";
    let input2 = "message2";
    let mut digest = entro_hash(input1.as_bytes(), 0);

    digest = entro_hash(input2.as_bytes(), digest);
    println!("{:08x}", digest);
}
```

To test the Cargo package, execute the following command.

``` console
cargo test
```

## Purchase
[EntroCraft](https://entrocraft.com/) maintains this open-source package with the permissive MIT license.

It's provided as a convenient code evaluation tool for the [premium Entro Hash library written in C](https://entrocraft.com/dungeon/hashing-algorithms/entro-hash/).

Converting code in this package from Rust to another programming language is discouraged and may be subject to either [purchasing a license](https://entrocraft.com/dungeon/hashing-algorithms/entro-hash/#license) for the premium library in C or attributing other OSI licenses.

Developers who don't use the C programming language can still [purchase credits](https://entrocraft.com/pricing/), learn C and support package maintenance.
