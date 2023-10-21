pub enum Role {
    Creator,
    Collaborator,
    Moderator,
    Guest,
}

pub enum UserAction {
    CreateTask,
    EditTask,
    MoveTask,
    DeleteTask,
    ReadTask,
}

pub struct User {
    pub username: String,
    pub role: Role,
}

fn has_permission(user: &User, action: &UserAction) -> bool {
    match user.role {
        Role::Creator => true,
        Role::Moderator | Role::Collaborator => match action {
            UserAction::DeleteTask => false,
            _ => true,
        },
        Role::Guest => match action {
            UserAction::ReadTask => true,
            _ => false,
        },
    }
}

pub fn authorize(user: &User, action: &UserAction) -> Result<(), String> {
    match has_permission(user, action) {
        true => Ok(()),
        false => Err("Permission denied".to_string()),
    }
}
