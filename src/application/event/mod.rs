pub mod registered_user_event;
pub mod registered_user_listener;
pub mod update_user_event;
pub mod update_user_listener;

#[derive(PartialEq)]
pub enum Event {
    RegisteredUser,
    UpdateUser
}