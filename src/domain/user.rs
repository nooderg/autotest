use uuid::Uuid;
use std::time::SystemTime;

use crate::schema::users;
use crate::application::event::registered_user_event::RegisteredUserEvent;
use crate::application::event::update_user_event::UpdateUserEvent;
use crate::application::event::Event;
use crate::infrastructure::event_bus::EVENT_BUS;

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
pub struct User {
    id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    #[serde(skip_serializing)] password: Option<String>,
    created_at: SystemTime,
}

impl User {
    pub fn new(
        id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        password: Option<String>,
        created_at: SystemTime,
    ) -> Self {
        Self {
            id: id,
            first_name: first_name,
            last_name: last_name,
            email: email,
            password: password,
            created_at: created_at,
        }
    }

    pub fn register_user(&self) -> () {
        self.apply(Event::RegisteredUser);
    }
    
    pub fn update_user(&self) -> () {
        self.apply(Event::UpdateUser);
    }

    fn apply(&self, event: Event) {
        match event {
            Event::RegisteredUser => {
                let mut data = RegisteredUserEvent {
                    first_name: self.first_name.clone().unwrap(),
                    last_name: self.last_name.clone().unwrap(),
                    email: self.email.clone().unwrap(),
                };
                post_event!(&EVENT_BUS, &mut data, RegisteredUserEvent);
            },
            Event::UpdateUser => {
                let mut data = UpdateUserEvent {
                    id: self.id.clone(),
                    first_name: self.first_name.clone().unwrap(),
                    last_name: self.last_name.clone().unwrap(),
                    email: self.email.clone().unwrap(),
                };
                post_event!(&EVENT_BUS, &mut data, UpdateUserEvent);
            },
        }
    }
}
