//! Archived versions of standard library containers.

pub mod btree_map;
pub mod btree_set;
pub mod hash_map;
pub mod hash_set;

pub use self::hash_map::ArchivedHashMap;
pub use self::hash_set::ArchivedHashSet;