//! src/domain/new_subscriber.rs

use crate::domain::subscriber_name::SubscriberName;

use super::SubscriberEmail;

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
