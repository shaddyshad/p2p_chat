use super::{Storage, QueryPredicate};

/// Listing type provides an iterator on types in the messaging modules 
/// Used to retrieve records from the storage for a particular user noted by the peer_id 
#[derive(Clone)]
pub struct Listing<T, S:Storage<Item=T>, P: QueryPredicate<T>> {
    storage: S,
    pred: P,
    next: usize
}


impl<T: Clone,S:Storage<Item=T>, P: QueryPredicate<T>> Iterator for Listing<T, S, P>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let items = self.storage.find(self.pred.clone());

        while self.next < items.len(){
            self.next += 1;

            return Some(items[self.next-1].clone())
         
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use crate::storage::{MemoryStorage, QueryPredicate};

    use super::*;

    // stub items with peer id 
    #[derive(Debug, Clone, PartialEq)]
    struct StubItem (String, u32);
    
    fn setup() -> MemoryStorage<StubItem>{
        // create a memory storage on u32 items and add a few
        let mut storage: MemoryStorage<StubItem> = MemoryStorage::new();

        storage.save(StubItem("001".to_string(), 10)).expect("can add");
        storage.save(StubItem("001".to_string(), 10)).expect("can add");
        storage.save(StubItem("001".to_string(), 19)).expect("can add");
        storage.save(StubItem("001".to_string(), 16)).expect("can add");
        storage.save(StubItem("002".to_string(), 12)).expect("can add");
        storage.save(StubItem("003".to_string(),13)).expect("can add");
        storage.save(StubItem("004".to_string(), 14)).expect("can add");

        storage
    }

    // predicate to find by peer id 
    #[derive(Clone)]
    struct FindById<'a>(&'a str);

    impl<'a> QueryPredicate<StubItem> for FindById<'a>{
        fn matches(&self, other: &StubItem) -> bool {
            self.0 == other.0
        }
    }

    #[test]
    fn listing(){
        let storage = setup();
        let peer_id = "001".to_string();
        let pred = FindById(&peer_id);

        let mut list = Listing {
            storage,
            pred,
            next: 0
        };
 


        list.next();
        list.next();

        let val = list.next().expect("can find third item");

        assert_eq!(val, StubItem("001".to_string(), 19));
    }

}