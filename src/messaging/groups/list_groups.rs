use crate::{Group, storage::Storage};
use super::QueryPredicate;

/// A type to list groups created by a peer 
pub struct ListGroups<S: Storage<Item=Group>> {
    storage: S,
    peer_id: String,
    next: usize 
}

impl<S: Storage<Item=Group>> ListGroups<S>{
    pub fn new(peer_id: String, storage: S) -> Self {
        ListGroups {
            storage,
            peer_id,
            next: 0
        }
    }
}

/// Queriy to retrieve groups registered by a peer 
#[derive(Clone)]
pub struct FindGroup(pub String);

impl QueryPredicate<Group> for FindGroup{
    fn matches(&self, other: &Group) -> bool {
        self.0 == other.peer_id()
    }
}


impl <S: Storage<Item=Group>> Iterator for ListGroups<S>{
    type Item = Group;

    fn next(&mut self) -> Option<Self::Item> {
        let items = self.storage.find(FindGroup(self.peer_id.clone()));

        while self.next < items.len(){
            self.next += 1;

            return Some(items[self.next - 1].clone());
        }

        None
    }
}