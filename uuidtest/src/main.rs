
extern crate uuid;

use uuid::Uuid;

fn main() {
    let my_uuid = Uuid::new_v5(&uuid::NAMESPACE_DNS, "foo");
    println!("{}", my_uuid);
}