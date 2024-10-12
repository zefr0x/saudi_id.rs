use saudi_id::{Id, IdType};

fn main() {
    let id = Id::try_from(1_564_437_091);

    match id {
        Ok(id) => match id.get_type() {
            IdType::Citizen => {
                println!("Valid Citizen ID");
            }
            IdType::Resident => {
                println!("Valid Resident ID");
            }
        },
        Err(_) => {
            println!("Invalid ID");
        }
    }
}
