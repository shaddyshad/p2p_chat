#[allow(dead_code)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A trait defining types that can provide predicate capabilities 
/// for storage queries 
pub trait QueryPredicate {
    type Item ;

    fn matches(&self, other: &Self::Item) -> bool;    
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
    fn list(&self) -> Box<dyn Iterator<Item=Self::Item>>;

    /// Delete all records 
    /// Returns a count of all deleted records or an error if any occured
    fn remove_all(&mut self) -> Result<usize>;

    /// Predicated version of list 
    fn find<Q: QueryPredicate>(&self, query: Q) -> Box<dyn Iterator<Item=Self::Item>>;

    /// Predicated remove 
    /// Returns a Result with count of the removed elements or an error 
    fn remove_if<Q:QueryPredicate>(&mut self, query: Q) -> Result<usize>;

}