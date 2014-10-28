//! Convenience traits to help clarify a common pattern in Rust code.
//!
//! The traits are seperated as `use existent::Existent` doesn't really
//! convey what is being imported, and by importing only a single method
//! at a time, the potential for clashes is reduced.
//!
//! There is a blanket `impl for T` for each trait so you should not `use` the
//! trait if you are working with types containing methods named `when` or `unless`.
//!
//! This should hopefully be less of a problem when/if UFCS lands.

pub trait When {
    /// Returns `Some(self)` if the predicate is `true`, otherwise `None`.
    ///
    /// # Example
    /// ```
    /// use existent::When;
    /// let x = 100u32;
    ///
    /// assert_eq!(None, x.when(x > 200));
    /// assert_eq!(Some(x), x.when(x < 200));
    /// ```
    #[inline]
    fn when(self, pred: bool) -> Option<Self> {
        if pred { Some(self) } else { None }
    }
}

pub trait Unless {
    /// Returns `None` if the predicate is `true`, otherwise `Some(self)`.
    ///
    /// # Example
    /// ```
    /// use existent::Unless;
    /// let xs = vec!["", "Three", "", "Two", "One"];
    ///
    /// let filtered = xs.into_iter()
    ///                  .filter_map(|s| s.unless(s.is_empty()))
    ///                  .collect::<Vec<&str>>();
    ///
    /// assert_eq!(filtered, vec!["Three", "Two", "One"])
    /// ```
    #[inline]
    fn unless(self, pred: bool) -> Option<Self> {
        self.when(!pred)
    }
}

impl<T> When for T {}
impl<T> Unless for T {}
