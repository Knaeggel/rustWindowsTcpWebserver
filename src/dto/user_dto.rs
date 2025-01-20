use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDto {
    pub id: u32,
    pub name: String,
    pub email: String,
}
