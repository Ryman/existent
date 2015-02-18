#![feature(core)]

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
);

basic_tests!(
    ints - 4usize,
    vecs - vec![false, true, false],
    tuples - ("Hello", 42i32, 79.4f64),
    strings - "World".to_string(),
    static_slices - "!!!!!one1on1"
);

#[test]
fn filter_map() {
    let xs = vec!["", "This", "", "Has", "Blanks"];

    let filtered = xs.into_iter()
                     .filter_map(|s| s.unless(s.is_empty()))
                     .collect::<Vec<&str>>();

    assert_eq!(filtered, vec!["This", "Has", "Blanks"]);
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

#[test]
fn ufcs() {
    #[derive(PartialEq, Debug)]
    struct Foo(usize);

    impl Foo {
        fn when(self, _: bool) -> Option<Foo> {
            None
        }

        fn unless(self, _: bool) -> Option<Foo> {
            Some(Foo(52))
        }
    }

    assert_eq!(Some(Foo(4)), When::when(Foo(4), true));
    assert_eq!(None, When::when(Foo(5), false));
    assert_eq!(None, Unless::unless(Foo(6), true));
    assert_eq!(Some(Foo(7)), Unless::unless(Foo(7), false));

    assert_eq!(None, Foo(8).when(true));
    assert_eq!(None, Foo(3).when(false));
    assert_eq!(Some(Foo(52)), Foo(2).unless(true));
    assert_eq!(Some(Foo(52)), Foo(1).unless(false));
}

// Pending on negative bounds/impls
// #[test]
// fn lazy() {
//     fn expensive_computation(bar: uint) -> uint {
//         42 * bar
//     }

//     let mut bar = 1;
//     assert_eq!(Some(42), (|| expensive_computation(bar)).unless(false));
//     assert_eq!(None::<uint>, (|| expensive_computation(bar)).unless(true));

//     bar = 2;
//     assert_eq!(Some(84), (|| expensive_computation(bar)).when(true));
//     assert_eq!(None::<uint>, (|| expensive_computation(bar)).when(false));
// }
