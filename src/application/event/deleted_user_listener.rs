use crate::application::event::deleted_user_event::DeletedUserEvent;

pub fn execute(event: &mut DeletedUserEvent) -> () {
    println!("event -> {:?}", event);
}
