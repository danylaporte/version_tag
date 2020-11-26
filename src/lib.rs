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
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct VersionTag(u64);

impl VersionTag {
    /// Creates an initialized new VersionTag.
    pub fn new() -> Self {
        VersionTag(COUNTER.fetch_add(1, Ordering::SeqCst) as u64)
    }

    /// Creates a version 0 which could indicate that the computation
    /// has not been done.
    ///
    /// # Example
    /// ```
    /// use version_tag::VersionTag;
    ///
    /// // The zero version will always be the lowest version.
    /// // Calling new will start at version 1
    /// assert!(VersionTag::zero() < VersionTag::new());
    /// ```
    pub fn zero() -> Self {
        VersionTag(0)
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

/// compute a new VersionTag by using the max value of other tags
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
/// assert!(t2 == t3);
///
/// assert_eq!(t3, combine(&[t1, t2]));
///
/// let mut t1 = t1;
/// t1.notify();
///
/// let t4 = combine(&[t1, t2]);
/// assert!(t3 != t4);
/// assert!(t4 == t1);
/// ```
pub fn combine(tags: &[VersionTag]) -> VersionTag {
    VersionTag(tags.iter().map(|t| t.0).max().unwrap_or_default())
}
