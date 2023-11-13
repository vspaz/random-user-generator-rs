# random-user-generator-rs

```rust
use random_data_generator::user::Data;

fn main() {
    let random_data = Data::new("foobar", 10, true, false);
    let users = random_data.generate().unwrap();
    println!("{}", users[0].name.first);
}

```