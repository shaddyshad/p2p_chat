use super::Result;
pub mod memory_storage;

pub use memory_storage::MemoryStorage;

/// A trait defining types that can provide predicate capabilities 
/// for storage queries 
pub trait QueryPredicate<T> {
    fn matches(&self, other: &T) -> bool;    
}

/// A trait for types that provide storage capabilities. Examples would include  
/// 
/// + Saving records to a form of storage 
/// + Retrieving all records 
/// + Removing a record 
/// + Removing all records 
/// + Retrieving based on a predicate 
pub trait Storage {
    /// data type 
    type Item;

    /// add a record to storage and return an error if not succesful 
    fn save(&mut self, item: Self::Item) -> Result<()>;

    /// Retieve all records in the storage 
    fn list(&self) -> Vec<Self::Item>;

    /// Delete all records 
    /// Returns a count of all deleted records or an error if any occured
    fn remove_all(&mut self) -> Result<usize>;

    /// Predicated version of list 
    fn find<Q: QueryPredicate<Self::Item>>(&self, query: Q) -> Vec<Self::Item>;

    /// Predicated remove 
    /// Returns a Result with count of the removed elements or an error 
    fn remove_if<Q:QueryPredicate<Self::Item>>(&mut self, query: Q) -> Result<usize>;


    /// Find one items given a predicate 
    fn find_one<Q:QueryPredicate<Self::Item>>(&self, query: Q) -> Option<Self::Item> {
        let items = self.find(query);

        if let Some(item) = items.into_iter().next(){
            return Some(item)
        }

        None 
    }


}


