#[derive(Clone, Copy, Default)]
pub struct UserState<'a> {
    pub first_name: &'a str,
    pub picture: &'a str,
}

#[derive(Clone, Copy, Default)]
pub struct AuthenticatedState<'a> {
    pub is_authenticated: bool,
    pub user: Option<UserState<'a>>,
}

impl<'a> AuthenticatedState<'a> {
    pub fn new() -> Self {
        Self {
            is_authenticated: false,
            user: None,
        }
    }

    pub fn set_user(&mut self, user: UserState<'a>) {
        self.is_authenticated = true;
        self.user = Some(user);
    }
}
