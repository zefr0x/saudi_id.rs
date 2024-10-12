use saudi_id::{Id, IdType};

fn main() {
    let id = Id::new(&IdType::Citizen);

    println!("{id}");
}
