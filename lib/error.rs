use std::{
    boxed::Box,
    error::Error,
    marker::{Send, Sync},
};

// Implement Error to satisfy ?
// Send and sync for threads
pub type BoxError = Box<dyn Error + Send + Sync>;
