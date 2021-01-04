use super::{Storage, QueryPredicate, Result};

/// Memory storage implementation over a generic typeT
pub struct MemoryStorage<T> {
    items: Vec<T>
}

impl<T> MemoryStorage<T>{
    /// Create a new memory storage 
    pub fn new() -> Self {
        Self {
            items: vec![]
        }
    }
}



/// Storage interface 
impl<T: Clone> Storage for MemoryStorage<T>{
    type Item = T;


    fn save(&mut self, item: Self::Item) -> Result<()> {
        Ok(self.items.push(item))
    }

    fn list(&self) ->  Vec<Self::Item> {
        self.items.to_vec()
    }

    fn find<Q: QueryPredicate<Self::Item>>(&self, query: Q) ->  Vec<Self::Item> {
        let items = self.items.to_vec();

        let items: Vec<T> = items.into_iter()
            .filter(|q| query.matches(q))
            .collect();

        items

    }

    fn remove_all(&mut self) -> Result<usize> {
        let len = self.items.len();
        self.items.clear();

        Ok(len)
    }

    fn remove_if<Q:QueryPredicate<Self::Item>>(&mut self, query: Q) -> Result<usize> {
        let initial_len = self.items.len();

        self.items.retain(|q| !query.matches(q));

        Ok(initial_len - self.items.len())
    }

}


#[cfg(test)]
mod tests {
    /// create a memory 
    use super::*;

    struct FindNum(u32);

    impl QueryPredicate<u32> for FindNum {
        fn matches(&self, other: &u32) -> bool {
            &self.0 == other
        }
    }


    #[test]
    fn can_save(){
        let mut storage: MemoryStorage<u32> = MemoryStorage::new();
        storage.save(2).expect("Can save a value");
        storage.save(3).expect("Can save a value");

        assert_eq!(storage.list().len(), 2);
    }

    #[test]
    fn can_list(){
        let mut storage: MemoryStorage<u32> = MemoryStorage::new();
        storage.save(2).expect("Can save a value");
        storage.save(3).expect("Can save a value");

        assert_eq!(storage.list(), vec![2, 3]);
    }

    #[test]
    fn can_find_num(){
        let values = setup();

        

        let num_56 = FindNum(56);
        let found = values.find(num_56);

        assert!(!found.is_empty());
        let other_num = FindNum(1223);
        let f = values.find(other_num);
        assert!(f.is_empty());
    }

    #[test]
    fn can_delete(){
        let mut values = setup();
        assert_eq!(values.list().len(), values.remove_all().unwrap());
    }

    #[test]
    fn can_conditionally_remove(){
        let mut values = setup();
        let val = FindNum(384);

        // remove all 384's 
        let res = values.remove_if(val).expect("can remove conditionally");

        assert_eq!(res, 3);
    }

    #[test]
    fn can_find_one(){
        let values = setup();
        let query = FindNum(3);

        assert_eq!(values.find_one(query), Some(3));
    }

    fn setup() -> MemoryStorage<u32>{
        let mut storage: MemoryStorage<u32> = MemoryStorage::new();
        storage.save(2).expect("Can save a value");
        storage.save(3).expect("Can save a value");
        storage.save(56).expect("Can save a value");
        storage.save(33).expect("Can save a value");
        storage.save(22).expect("Can save a value");
        storage.save(384).expect("Can save a value");
        storage.save(384).expect("Can save a value");
        storage.save(384).expect("Can save a value");

        storage 
    }

}