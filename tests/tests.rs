#![feature(macro_rules)]
extern crate existent;
use existent::{When, Unless};

macro_rules! basic_tests(
    ($($name:ident - $exp:expr),+) => (
        mod test_unless {
            use existent::Unless;
            $(
                #[test]
                fn $name() {
                    assert_eq!(Some($exp), $exp.unless(false));
                    assert_eq!(None, $exp.unless(true));
                }
            )+
        }

        mod test_when {
            use existent::When;
            $(
                #[test]
                fn $name() {
                    assert_eq!(Some($exp), $exp.when(true));
                    assert_eq!(None, $exp.when(false));
                }
            )+
        }
    )
)

basic_tests!(
    ints - 4u,
    vecs - vec![false, true, false],
    tuples - ("Hello", 42i32, 79.4f64),
    strings - "World".to_string(),
    static_slices - "!!!!!one1on1"
)

#[test]
fn filter_map() {
    let xs = vec!["", "This", "", "Has", "Blanks"];

    let filtered = xs.into_iter()
                     .filter_map(|s| s.unless(s.is_empty()))
                     .collect::<Vec<&str>>();

    assert_eq!(filtered, vec!["This", "Has", "Blanks"])
}

#[test]
fn lifetimes_work() {
    fn unless<'a>(s: &'a str) -> Option<&'a str> {
        s.unless(false)
    }

    fn when<'a>(s: &'a str) -> Option<&'a str> {
        s.when(true)
    }

    let x = "Abandoning".to_string();

    let ys = {
        let y = x.as_slice();
        unless(y)
    };

    assert_eq!(Some(x.as_slice()), ys);

    let ys = {
        let y = x.as_slice();
        when(y)
    };
    assert_eq!(Some(x.as_slice()), ys);
}
