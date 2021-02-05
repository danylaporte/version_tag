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
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

/// Allow to share this tag between process reload.
/// This tag can be serialized and deseralize.
#[cfg(feature = "shared-tag")]
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct SharedTag(u128);

#[cfg(feature = "shared-tag")]
impl SharedTag {
    pub fn new(tag: VersionTag) -> Self {
        Self(shared(Self::instance(), tag.0))
    }

    fn instance() -> u64 {
        static INSTANCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let val = INSTANCE.load(Relaxed);

        if val == 0 {
            let new = rand::random();
            INSTANCE.compare_and_swap(0, new, Relaxed)
        } else {
            val
        }
    }
}

#[cfg(feature = "shared-tag")]
impl Eq for SharedTag {}

#[cfg(feature = "shared-tag")]
impl From<VersionTag> for SharedTag {
    fn from(v: VersionTag) -> Self {
        Self::new(v)
    }
}

#[cfg(feature = "shared-tag")]
impl PartialEq for SharedTag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(feature = "shared-tag")]
impl PartialEq<Option<SharedTag>> for SharedTag {
    fn eq(&self, other: &Option<SharedTag>) -> bool {
        other.as_ref().map_or(false, |t| self == t)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VersionTag(u64);

impl VersionTag {
    /// Creates an initialized new VersionTag.
    pub fn new() -> Self {
        VersionTag(COUNTER.fetch_add(1, Relaxed) as u64)
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
        self.0 = COUNTER.fetch_add(1, Relaxed) as u64;
    }
}

impl Default for VersionTag {
    /// Creates an initialized new VersionTag. This is the same as calling VersionTag::new.
    fn default() -> Self {
        VersionTag::new()
    }
}

impl From<VersionTag> for u64 {
    fn from(t: VersionTag) -> Self {
        t.0
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

#[cfg(any(test, feature = "shared-tag"))]
fn shared(instance: u64, tag: u64) -> u128 {
    let i = (instance as u128) << 8;
    i + tag as u128
}

#[test]
fn test_shared_overflow() {
    shared(u64::MAX, u64::MAX);
}
