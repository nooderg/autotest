pub mod registered_user_event;
pub mod registered_user_listener;
pub mod deleted_user_event;
pub mod deleted_user_listener;

#[derive(PartialEq)]
pub enum Event {
    RegisteredUser,
    DeletedUser
}