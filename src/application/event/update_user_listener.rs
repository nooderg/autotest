use crate::application::event::update_user_event::UpdateUserEvent;

pub fn execute(event: &mut UpdateUserEvent) -> () {
    println!("event -> {:?}", event);
}
