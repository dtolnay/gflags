use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};

use ref_cast::RefCast;

use crate::atomic::StaticAtomicPtr;

/// The state associated with a single flag.
///
/// An invocation of [`gflags::define!`] with flag long name `--the_name` and
/// flag type `T` declares an item `static the_name: Flag<T>` through which the
/// state of the flag can be accessed.
///
/// After [`gflags::parse()`] has been called, the value of a flag is available
/// through its `.FLAG` field which is of type `T`.
///
/// [`gflags::define!`]: macro.define.html
/// [`gflags::parse()`]: fn.parse.html
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// gflags::define! {
///     /// Search for patterns from the given file, with one pattern per line.
///     -f, --file: &Path
/// }
///
/// fn main() {
///     let patterns = gflags::parse();
///
///     if file.is_present() {
///         let path = file.FLAG;
///         println!("searching for patterns from file: {}", path.display());
///     } else {
///         println!("searching for patterns given on command line: {:?}", patterns);
///     }
/// }
/// ```
pub struct Flag<T> {
    atomic: StaticAtomicPtr<T>,
    present: AtomicBool,
}

impl<T: 'static> Flag<T> {
    /// Whether this flag was provided on the command line.
    ///
    /// When using flags for which a default value is not provided, be sure to
    /// check `.is_present()` because accessing `.FLAG` when not present will
    /// cause a panic.
    ///
    /// When a flag has a default value and is not passed on the command line,
    /// `is_present()` will be false and `.FLAG` will refer to the default
    /// value.
    pub fn is_present(&self) -> bool {
        self.present.load(Ordering::SeqCst)
    }
}

#[allow(non_snake_case)]
#[derive(RefCast)]
#[repr(transparent)]
pub struct Accessor<T> {
    /// Value of the flag.
    pub FLAG: T,
}

impl<T: 'static> Flag<T> {
    // Not public API. Called from generated code.
    #[doc(hidden)]
    pub const fn new(default: &'static T) -> Self {
        Flag {
            atomic: StaticAtomicPtr::new(default),
            present: AtomicBool::new(false),
        }
    }

    // Not public API. Called from generated code.
    #[doc(hidden)]
    pub const fn null() -> Self {
        Flag {
            atomic: StaticAtomicPtr::null(),
            present: AtomicBool::new(false),
        }
    }

    pub(crate) fn set(&self, value: T) {
        let ptr = Box::leak(Box::new(value));
        self.atomic.store(ptr);
        self.present.store(true, Ordering::SeqCst);
    }
}

impl Flag<bool> {
    pub(crate) fn set_bool(&self, value: &'static bool) {
        self.atomic.store(value);
        self.present.store(true, Ordering::SeqCst);
    }
}

impl<T: 'static> Deref for Flag<T> {
    type Target = Accessor<T>;

    fn deref(&self) -> &Self::Target {
        Accessor::ref_cast(self.atomic.load())
    }
}
