//! Convenience traits to help clarify a common pattern in Rust code.
//!
//! The traits are seperated as `use existent::Existent` doesn't really
//! convey what is being imported, and by importing only a single method
//! at a time, the potential for clashes is reduced.

pub trait When where Self: Sized {
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

    /// Returns `Some(closure_result)` if the predicate is `true`, otherwise `None`.
    ///
    /// Invokes the closure when iff the predicate is `true`.
    #[inline]
    fn do_when<O>(self, pred: bool) -> Option<O> where Self: FnOnce() -> O {
        if pred { Some(self()) } else { None }
    }
}

pub trait Unless : When {
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

    /// Returns `None` if the predicate is `true`, otherwise `Some(closure_result)`.
    ///
    /// Invokes the closure when iff the predicate is `false`.
    #[inline]
    fn do_unless<O>(self, pred: bool) -> Option<O> where Self: FnOnce() -> O {
        self.do_when(!pred)
    }
}

impl<T> When for T {}
impl<T> Unless for T {}
