//! # wrapped-list
//! 
//! This crate provides the `wrapped_list!` macro which allows you to create a list of elements that are wrapped by an object, function, or macro at compile time.
//! 
//! ## Examples
//! 
//! ### Wrap with a tuple struct or enum
//! 
//! ```
//! use wrapped_list::*;
//! 
//! #[derive(Debug, PartialEq, Eq)]
//! struct Wrapper(i32);
//! 
//! let wrapped_items = wrapped_list!(Wrapper; 1, 2, 3, 4);
//! 
//! assert_eq!(wrapped_items, [Wrapper(1), Wrapper(2), Wrapper(3), Wrapper(4)]);
//! ```
//! 
//! ### Wrap with an object or function
//! 
//! ```
//! use wrapped_list::*;
//! 
//! let boxed_items = wrapped_list!(Box::new; 1, 2, 3);
//! 
//! assert_eq!(boxed_items, [Box::new(1), Box::new(2), Box::new(3)])
//! ```
//! 
//! ```
//! use wrapped_list::*;
//! 
//! let func = |x| x * 2;
//! 
//! let doubled = wrapped_list!(func; 1, 2, 3);
//! 
//! assert_eq!(doubled, [2, 4, 6]);
//! ```
//! 
//! ### Wrap with a macro
//! 
//! ```
//! use wrapped_list::*;
//! 
//! macro_rules! add_one {
//!     ($e:expr) => {
//!         $e + 1
//!     };
//! }
//! 
//! let one_more = wrapped_list!(add_one!; 1, 2, 3);
//! 
//! assert_eq!(one_more, [2, 3, 4]);
//! ```

#[macro_export]
macro_rules! wrapped_list {
    /* ---------------------------------- Paths --------------------------------- */
    ($wrapper:path ; $e:expr $(,)?) => {
        [$wrapper($e)]
    };
    ($wrapper:path ; $e1:expr, $e2:expr $(,)?) => {
        [$wrapper($e1), $wrapper($e2)]
    };
    ($wrapper:path ; $e1:expr, $e2:expr, $($es:expr),*) => {
        __wrapped_list_impl!($wrapper ; [$wrapper($e1)] ; $e2 ; $($es),*)
    };
    // Allow trailing commas
    ($wrapper:path ; $e1:expr, $e2:expr, $($es:expr,)*) => {
        wrapped_list!($wrapper ; $e1 , $e2 , $($es),*)
    };
    /* --------------------------------- Macros --------------------------------- */
    ($wrapper:ident! ; $e:expr $(,)?) => {
        [$wrapper!($e)]
    };
    ($wrapper:ident! ; $e1:expr, $e2:expr $(,)?) => {
        [$wrapper!($e1), $wrapper!($e2)]
    };
    ($wrapper:ident! ; $e1:expr, $e2:expr, $($es:expr),*) => {
        __wrapped_list_impl!($wrapper! ; [$wrapper!($e1)] ; $e2 ; $($es),*)
    };
    // Allow trailing commas
    ($wrapper:ident! ; $e1:expr, $e2:expr, $($es:expr,)*) => {
        wrapped_list!($wrapper! ; $e1 , $e2 , $($es),*)
    };
}

#[macro_export]
macro_rules! __wrapped_list_impl {
    /* ---------------------------------- Paths --------------------------------- */
    ($out:tt) => {
        $out
    };
    ($wrapper:path ; [$($out:tt)*] ; $e1:expr ; $e2:expr, $($es:expr),*) => {
        __wrapped_list_impl!($wrapper ; [$($out)* , $wrapper($e1)] ; $e2 ; $($es),*)
    };
    ($wrapper:path ; [$($out:tt)*] ; $e1:expr ; $e2:expr) => {
        __wrapped_list_impl!([$($out)* , $wrapper($e1), $wrapper($e2)])
    };
    /* --------------------------------- Macros --------------------------------- */
    ($wrapper:ident! ; [$($out:tt)*] ; $e1:expr ; $e2:expr, $($es:expr),*) => {
        __wrapped_list_impl!($wrapper! ; [$($out)* , $wrapper!($e1)] ; $e2 ; $($es),*)
    };
    ($wrapper:ident! ; [$($out:tt)*] ; $e1:expr ; $e2:expr) => {
        __wrapped_list_impl!([$($out)* , $wrapper!($e1), $wrapper!($e2)])
    };
}


#[cfg(test)]
mod tests {
    use duplicate::duplicate_item;


    #[derive(PartialEq, Eq, Debug)]
    struct Wrapper(i32);

    #[derive(PartialEq, Eq, Debug)]
    struct ComplexWrapper {
        wrapped_item: i32,
    }

    impl ComplexWrapper {
        pub fn new(item: i32) -> Self {
            ComplexWrapper { wrapped_item: item }
        }
    }

    fn wrapper_function1(input: i32) -> i32 {
        input * 10
    }

    fn wrapper_function2(input: i32) -> bool {
        input > 2
    }

    macro_rules! wrapper_macro1 {
        ($e:expr) => {
            Wrapper($e)
        };
    }

    macro_rules! add_one {
        ($e:expr) => {
            $e + 1
        };
    }

    macro_rules! wrapper_macro2 {
        ($e:expr) => {
            Wrapper(add_one!($e))
        }
    }

    #[duplicate_item(
        wrapper                test_name;
        [Wrapper]              [wrapper_test];
        [ComplexWrapper::new]  [complex_wrapper_test];
        [Box::new]             [box_test];
        [wrapper_function1]    [function_test1];
        [wrapper_function2]    [function_test2];
    )]
    #[test]
    fn test_name() {
        let my_list = [wrapper(1)];
        assert_eq!(my_list, wrapped_list!(wrapper; 1));
        let my_list = [wrapper(1), wrapper(2)];
        assert_eq!(my_list, wrapped_list!(wrapper; 1, 2));
        let my_list = [wrapper(1), wrapper(2), wrapper(3)];
        assert_eq!(my_list, wrapped_list!(wrapper; 1, 2, 3));
        let my_list = [wrapper(1), wrapper(2), wrapper(3), wrapper(4)];
        assert_eq!(my_list, wrapped_list!(wrapper; 1, 2, 3, 4));
    }

    #[test]
    fn trailing_commas() {
        let my_list = [Wrapper(1)];
        assert_eq!(my_list, wrapped_list!(Wrapper; 1,));
        let my_list = [Wrapper(1), Wrapper(2)];
        assert_eq!(my_list, wrapped_list!(Wrapper; 1, 2,));
        let my_list = [Wrapper(1), Wrapper(2), Wrapper(3)];
        assert_eq!(my_list, wrapped_list!(Wrapper; 1, 2, 3,));
        let my_list = [Wrapper(1), Wrapper(2), Wrapper(3), Wrapper(4)];
        assert_eq!(my_list, wrapped_list!(Wrapper; 1, 2, 3, 4,));
    }

    #[duplicate_item(
        macro_name        test_name;
        [wrapper_macro1]  [macro_test1];
        [wrapper_macro2]  [macro_test2];
    )]
    #[test]
    fn test_name() {
        let my_list = [macro_name!(1)];
        assert_eq!(my_list, wrapped_list!(macro_name!; 1));
        let my_list = [macro_name!(1), macro_name!(2)];
        assert_eq!(my_list, wrapped_list!(macro_name!; 1, 2));
        let my_list = [macro_name!(1), macro_name!(2), macro_name!(3)];
        assert_eq!(my_list, wrapped_list!(macro_name!; 1, 2, 3));
        let my_list = [macro_name!(1), macro_name!(2), macro_name!(3), macro_name!(4)];
        assert_eq!(my_list, wrapped_list!(macro_name!; 1, 2, 3, 4));
    }

    #[test]
    fn trailing_commas_macro() {
        let my_list = [wrapper_macro2!(1)];
        assert_eq!(my_list, wrapped_list!(wrapper_macro2!; 1,));
        let my_list = [wrapper_macro2!(1), wrapper_macro2!(2)];
        assert_eq!(my_list, wrapped_list!(wrapper_macro2!; 1, 2,));
        let my_list = [wrapper_macro2!(1), wrapper_macro2!(2), wrapper_macro2!(3)];
        assert_eq!(my_list, wrapped_list!(wrapper_macro2!; 1, 2, 3,));
        let my_list = [wrapper_macro2!(1), wrapper_macro2!(2), wrapper_macro2!(3), wrapper_macro2!(4)];
        assert_eq!(my_list, wrapped_list!(wrapper_macro2!; 1, 2, 3, 4,));
    }
}