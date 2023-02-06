use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Store, Default, Clone, PartialEq)]
pub struct UserStore {
    pub username: String,
    pub password: String,
}

#[derive(Store, Default, Clone, PartialEq)]
pub struct BoardStore {
    pub data: String,
    pub is_login: bool,
}
