#![doc = include_str!("../README.md")]

#[allow(private_bounds, reason = "on purpose")]
#[rustfmt::skip]
pub struct Sudoku<
    _1_1, _1_2, _1_3, _1_4, _1_5, _1_6, _1_7, _1_8, _1_9,
    _2_1, _2_2, _2_3, _2_4, _2_5, _2_6, _2_7, _2_8, _2_9,
    _3_1, _3_2, _3_3, _3_4, _3_5, _3_6, _3_7, _3_8, _3_9,
    _4_1, _4_2, _4_3, _4_4, _4_5, _4_6, _4_7, _4_8, _4_9,
    _5_1, _5_2, _5_3, _5_4, _5_5, _5_6, _5_7, _5_8, _5_9,
    _6_1, _6_2, _6_3, _6_4, _6_5, _6_6, _6_7, _6_8, _6_9,
    _7_1, _7_2, _7_3, _7_4, _7_5, _7_6, _7_7, _7_8, _7_9,
    _8_1, _8_2, _8_3, _8_4, _8_5, _8_6, _8_7, _8_8, _8_9,
    _9_1, _9_2, _9_3, _9_4, _9_5, _9_6, _9_7, _9_8, _9_9,
>(
    pub _1_1, pub _1_2, pub _1_3, pub _1_4, pub _1_5, pub _1_6, pub _1_7, pub _1_8, pub _1_9,
    pub _2_1, pub _2_2, pub _2_3, pub _2_4, pub _2_5, pub _2_6, pub _2_7, pub _2_8, pub _2_9,
    pub _3_1, pub _3_2, pub _3_3, pub _3_4, pub _3_5, pub _3_6, pub _3_7, pub _3_8, pub _3_9,
    pub _4_1, pub _4_2, pub _4_3, pub _4_4, pub _4_5, pub _4_6, pub _4_7, pub _4_8, pub _4_9,
    pub _5_1, pub _5_2, pub _5_3, pub _5_4, pub _5_5, pub _5_6, pub _5_7, pub _5_8, pub _5_9,
    pub _6_1, pub _6_2, pub _6_3, pub _6_4, pub _6_5, pub _6_6, pub _6_7, pub _6_8, pub _6_9,
    pub _7_1, pub _7_2, pub _7_3, pub _7_4, pub _7_5, pub _7_6, pub _7_7, pub _7_8, pub _7_9,
    pub _8_1, pub _8_2, pub _8_3, pub _8_4, pub _8_5, pub _8_6, pub _8_7, pub _8_8, pub _8_9,
    pub _9_1, pub _9_2, pub _9_3, pub _9_4, pub _9_5, pub _9_6, pub _9_7, pub _9_8, pub _9_9,
)
where
    // Rows.
    _1_1: AllDifferent<_1_2, _1_3, _1_4, _1_5, _1_6, _1_7, _1_8, _1_9>,
    _2_1: AllDifferent<_2_2, _2_3, _2_4, _2_5, _2_6, _2_7, _2_8, _2_9>,
    _3_1: AllDifferent<_3_2, _3_3, _3_4, _3_5, _3_6, _3_7, _3_8, _3_9>,
    _4_1: AllDifferent<_4_2, _4_3, _4_4, _4_5, _4_6, _4_7, _4_8, _4_9>,
    _5_1: AllDifferent<_5_2, _5_3, _5_4, _5_5, _5_6, _5_7, _5_8, _5_9>,
    _6_1: AllDifferent<_6_2, _6_3, _6_4, _6_5, _6_6, _6_7, _6_8, _6_9>,
    _7_1: AllDifferent<_7_2, _7_3, _7_4, _7_5, _7_6, _7_7, _7_8, _7_9>,
    _8_1: AllDifferent<_8_2, _8_3, _8_4, _8_5, _8_6, _8_7, _8_8, _8_9>,
    _9_1: AllDifferent<_9_2, _9_3, _9_4, _9_5, _9_6, _9_7, _9_8, _9_9>,

    // Columns.
    _1_1: AllDifferent<_2_1, _3_1, _4_1, _5_1, _6_1, _7_1, _8_1, _9_1>,
    _2_1: AllDifferent<_2_2, _3_2, _4_2, _5_2, _6_2, _7_2, _8_2, _9_2>,
    _3_1: AllDifferent<_2_3, _3_3, _4_3, _5_3, _6_3, _7_3, _8_3, _9_3>,
    _4_1: AllDifferent<_2_4, _3_4, _4_4, _5_4, _6_4, _7_4, _8_4, _9_4>,
    _5_1: AllDifferent<_2_5, _3_5, _4_5, _5_5, _6_5, _7_5, _8_5, _9_5>,
    _6_1: AllDifferent<_2_6, _3_6, _4_6, _5_6, _6_6, _7_6, _8_6, _9_6>,
    _7_1: AllDifferent<_2_7, _3_7, _4_7, _5_7, _6_7, _7_7, _8_7, _9_7>,
    _8_1: AllDifferent<_2_8, _3_8, _4_8, _5_8, _6_8, _7_8, _8_8, _9_8>,
    _9_1: AllDifferent<_2_9, _3_9, _4_9, _5_9, _6_9, _7_9, _8_9, _9_9>,

    // Squares.
    _1_1: AllDifferent<_1_2, _1_3, _2_1, _2_2, _2_3, _3_1, _3_2, _3_3>,
    _1_4: AllDifferent<_1_5, _1_6, _2_4, _2_5, _2_6, _3_4, _3_5, _3_6>,
    _1_7: AllDifferent<_1_8, _1_9, _2_7, _2_8, _2_9, _3_7, _3_8, _3_9>,

    _4_1: AllDifferent<_4_2, _4_3, _5_1, _5_2, _5_3, _6_1, _6_2, _6_3>,
    _4_4: AllDifferent<_4_5, _4_6, _5_4, _5_5, _5_6, _6_4, _6_5, _6_6>,
    _4_7: AllDifferent<_4_8, _4_9, _5_7, _5_8, _5_9, _6_7, _6_8, _6_9>,

    _7_1: AllDifferent<_7_2, _7_3, _8_1, _8_2, _8_3, _9_1, _9_2, _9_3>,
    _7_4: AllDifferent<_7_5, _7_6, _8_4, _8_5, _8_6, _9_4, _9_5, _9_6>,
    _7_7: AllDifferent<_7_8, _7_9, _8_7, _8_8, _8_9, _9_7, _9_8, _9_9>,
;

/// Check a sudoku board at compile time using only the type system.
///
/// # Examples
///
/// Will compile.
///
/// ```
/// # use sudoku_checker::sudoku;
/// #
/// let sudoku = sudoku! {
///     1, 2, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, 8, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, 4,
/// };
/// ```
///
/// Will not compile.
///
/// ```compile_fail
/// # use sudoku_checker::sudoku;
/// #
/// let sudoku = sudoku! {
///     1, 2, 2, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, 8, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, 4,
/// };
/// ```
///
/// ```compile_fail
/// # use sudoku_checker::sudoku;
/// #
/// let sudoku = sudoku! {
///     1, 2, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, 8, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, _, _, _,
///     _, _, _, _, _, _, 4, _, _,
///     _, _, _, _, _, _, _, _, 4,
/// };
/// ```
#[macro_export]
macro_rules! sudoku {
    ($( $c:tt ),* $(,)?) => {
        ::sudoku_checker::Sudoku($( ::sudoku_checker::__private::paste!{ ::sudoku_checker::cell::[<_ $c>] } ),*)
    };
}

pub mod cell {
    pub struct _1;
    pub struct _2;
    pub struct _3;
    pub struct _4;
    pub struct _5;
    pub struct _6;
    pub struct _7;
    pub struct _8;
    pub struct _9;

    /// An empty cell.
    pub struct __;
}

use cell::*;

trait Different<Other> {}

macro_rules! differentiate {
    ($( $t:ident ),* $(,)? from $x:ident) => {
        $(
            impl Different<$t> for $x {}
        )*
    };
}

differentiate!(_1, _2, _3, _4, _5, _6, _7, _8, _9, __ from __);
differentiate!(    _2, _3, _4, _5, _6, _7, _8, _9, __ from _1);
differentiate!(_1,     _3, _4, _5, _6, _7, _8, _9, __ from _2);
differentiate!(_1, _2,     _4, _5, _6, _7, _8, _9, __ from _3);
differentiate!(_1, _2, _3,     _5, _6, _7, _8, _9, __ from _4);
differentiate!(_1, _2, _3, _4,     _6, _7, _8, _9, __ from _5);
differentiate!(_1, _2, _3, _4, _5,     _7, _8, _9, __ from _6);
differentiate!(_1, _2, _3, _4, _5, _6,     _8, _9, __ from _7);
differentiate!(_1, _2, _3, _4, _5, _6, _7,     _9, __ from _8);
differentiate!(_1, _2, _3, _4, _5, _6, _7, _8,     __ from _9);

trait AllDifferent<_2, _3, _4, _5, _6, _7, _8, _9> {}

#[rustfmt::skip]
impl<_1, _2, _3, _4, _5, _6, _7, _8, _9> AllDifferent<_2, _3, _4, _5, _6, _7, _8, _9> for _1
where
    _1: Different<_2> + Different<_3> + Different<_4> + Different<_5> + Different<_6> + Different<_7> + Different<_8> + Different<_9>,
    _2: Different<_3> + Different<_4> + Different<_5> + Different<_6> + Different<_7> + Different<_8> + Different<_9>,
    _3: Different<_4> + Different<_5> + Different<_6> + Different<_7> + Different<_8> + Different<_9>,
    _4: Different<_5> + Different<_6> + Different<_7> + Different<_8> + Different<_9>,
    _5: Different<_6> + Different<_7> + Different<_8> + Different<_9>,
    _6: Different<_7> + Different<_8> + Different<_9>,
    _7: Different<_8> + Different<_9>,
    _8: Different<_9>,
{ }

// We do this so the user of the `sudoku` macro will not
// have to add `paste` as a dependency for their crate.
#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use paste::paste;
}
