
extern crate uuid;

use uuid::Uuid;

fn main() {
    let my_uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
    println!("Parsed a version {} UUID.", my_uuid.get_version_num());
    println!("{}", my_uuid);
}