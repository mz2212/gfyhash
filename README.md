# gfyhash

A crate to create [gfycat](https://gfycat.com/) style `AdjectiveAdjectiveAnimal` "hashes".  
They probably can't be considered real hashes as it's probably quite easy to create collisions, and they're almost certainly not cryptographically secure, so it's probably a (really) bad idea to use them in that way.

Some basic usage is like so:

```rust
use gfyhash::gfyhash;
let bar = String::from("foo");
// Yeah, the function signature isn't super pretty, but Rust doesn't allow for default arguments
println!("{}" gfyhash(&bar, None, None, None).unwrap());
```
