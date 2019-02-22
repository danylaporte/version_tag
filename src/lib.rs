//! A VersionTag struct for Rust to detect changes
//! 
//! # Example
//! Suppose you want to do a heavy computation based on something else.
//! 
//! You might want to cache the heavy computation result and perform it again
//! only when a dependency changes.
//! 
//! ```
//! use version_tag::{combine, VersionTag};
//! 
//! /// A dependency
//! struct Dep {
//!     v: u32,
//!     tag: VersionTag,
//! }
//! 
//! struct MyStruct {
//!     v: u32,
//!     tag: VersionTag,
//! }
//! 
//! impl MyStruct {
//!     fn update_if_necessary(&mut self, x: &Dep, y: &Dep) {
//! 
//!         // compute the actual tag
//!         let actual = combine(&[x.tag, y.tag]);
//! 
//!         // if the tag has changed, we need to recalculate
//!         if actual != self.tag {
//! 
//!             // perform the heavy computation
//!             self.v = x.v + y.v;
//! 
//!             // update the tag
//!             self.tag = actual;
//!         }
//!     }
//! }
//! ```
use std::hash::Hasher;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VersionTag(u64);

impl VersionTag {
    /// Creates an initialized new VersionTag.
    pub fn new() -> Self {
        VersionTag(COUNTER.fetch_add(1, Ordering::SeqCst) as u64)
    }

    /// Internally increment the counter of the tag to signal a change.
    pub fn notify(&mut self) {
        self.0 = COUNTER.fetch_add(1, Ordering::SeqCst) as u64;
    }
}

impl Default for VersionTag {
    /// Creates an initialized new VersionTag. This is the same as calling VersionTag::new.
    fn default() -> Self {
        VersionTag::new()
    }
}

/// compute a new VersionTag by hashing all other tags together
///
/// # Example
/// ```
/// use version_tag::{VersionTag, combine};
///
/// let t1 = VersionTag::new();
/// let t2 = VersionTag::new();
///
/// let t3 = combine(&[t1, t2]);
///
/// assert!(t1 != t3);
/// assert!(t2 != t3);
///
/// assert_eq!(t3, combine(&[t1, t2]));
///
/// let mut t1 = t1;
/// t1.notify();
///
/// assert!(t3 != combine(&[t1, t2]));
/// ```
pub fn combine(tags: &[VersionTag]) -> VersionTag {
    let mut hasher = std::collections::hash_map::DefaultHasher::default();

    for tag in tags {
        hasher.write_u64(tag.0);
    }

    VersionTag(hasher.finish())
}
