use self::file_accessor::{FileMutations, FileQueries};

pub mod file_accessor;

pub trait FileStorage: Sync + Send + FileQueries + FileMutations {}
