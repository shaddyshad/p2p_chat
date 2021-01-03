use super::Group;
use crate::{storage::{Storage, QueryPredicate}, Result};
use super::super::subscriptions::Subscriptions;

/// Query to check for matches in group name 
struct GroupNameQuery<'a>(&'a str);

impl<'a> QueryPredicate<Group> for GroupNameQuery<'a> {
    fn matches(&self, other: &Group) -> bool {
        self.0 == &other.name
    }
}

/// Provides facilities to build a new group chat 
pub struct NewGroup<T: Storage<Item=Group>, S: Subscriptions> {
    pub group_name: String,
    pub peer_id: String,
    pub storage: T,
    pub subscriptions: S,
}


impl <T:Storage<Item=Group>, S: Subscriptions> NewGroup<T, S>{
    ///  create a new group from the given data and save it 
    /// to the storage instance provided
    pub fn save(&mut self) -> Result<()>{
        let group = Group::new(self.group_name.clone(), self.peer_id.clone());

        self.storage.save(group)
    } 

    /// Subscribe to the new group 
    /// Subscriptions always save the group first 
    pub fn subscribe(&mut self) -> bool {
        if !self.exists(){
            // save it first 
            self.save().expect("can save group to storage");
        }

        self.subscriptions.subscribe(&self.peer_id, &self.group_name)
        
    }

    /// Check if a group has already been saved 
    /// Creates a group name query predicate and issues a find on the 
    /// storage. If the iterator returned is empty, then group is not created
    pub fn exists(&self) -> bool {
        let predicate = GroupNameQuery(&self.group_name);

        !self.storage.find(predicate).is_empty()
    }

    /// Unsubscribe from this group 
    pub fn unsubscribe(&mut self) -> bool {
        self.subscriptions.unsubscribe(&self.peer_id, &self.group_name)
    }

}