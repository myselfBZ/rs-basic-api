use std::collections::HashMap;

use crate::response::User;

pub struct MyState {
    pub users: HashMap<String, User>
}
