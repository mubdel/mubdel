use poem::{FromRequest, Request, RequestBody, Result};

#[derive(Clone)]
pub enum Role {
    Guest,
    User(UserRole),
    System, // TODO
}

#[derive(Clone)]
pub struct UserRole {
    pub id: String,
    pub username: String,
}

impl Role {
    pub fn is_guest(&self) -> bool {
        matches!(self, Role::Guest)
    }

    pub fn is_user(&self) -> (bool, Option<UserRole>) {
        match self {
            Self::User(u) => (true, Some(u.clone())),
            _ => (false, None),
        }
    }

    pub fn is_system(&self) -> bool {
        matches!(self, Role::System)
    }
}

impl<'a> FromRequest<'a> for Role {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let role = req.extensions().get::<Role>();
        Ok(role.unwrap_or(&Role::Guest).clone())
    }
}
