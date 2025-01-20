use crate::models::user::User;
use crate::dto::user_dto::UserDto;

pub fn to_user_dto(user: &User) -> UserDto {
    UserDto {
        id: user.id.clone(),
        name: user.name.clone(),
        email: user.email.clone(),
    }
}

pub fn from_user_dto(user_dto: &UserDto) -> User {
    User {
        id: user_dto.id.clone(),
        name: user_dto.name.clone(),
        email: user_dto.email.clone(),
    }
}