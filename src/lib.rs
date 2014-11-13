//! Convenience traits to help clarify a common pattern in Rust code.
//!
//! The traits are seperated as `use existent::Existent` doesn't really
//! convey what is being imported, and by importing only a single method
//! at a time, the potential for clashes is reduced.

pub trait When<T> {
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
    fn when(self, pred: bool) -> Option<T>;
}

pub trait Unless<T> {
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
    fn unless(self, pred: bool) -> Option<T>;
}

impl<T> When<T> for T {
    #[inline]
    fn when(self, pred: bool) -> Option<T> {
        if pred { Some(self) } else { None }
    }
}

impl<T> Unless<T> for T {
    #[inline]
    fn unless(self, pred: bool) -> Option<T> {
        self.when(!pred)
    }
}

impl<'a, T> When<T> for ||:'a -> T {
    #[inline]
    fn when(self, pred: bool) -> Option<T> {
        if pred { Some(self()) } else { None }
    }
}

impl<'a, T> Unless<T> for ||:'a -> T {
    #[inline]
    fn unless(self, pred: bool) -> Option<T> {
        self.when(!pred)
    }
}
