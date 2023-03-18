# wrapped-list

[![Build](https://github.com/hunterlawson/wrapped-list/actions/workflows/tests.yml/badge.svg)](https://github.com/hunterlawson/wrapped-list/blob/master/.github/workflows/tests.yml)
[![Crate](https://img.shields.io/crates/v/wrapped-list)](https://crates.io/crates/wrapped-list)
[![License](https://img.shields.io/crates/l/wrapped-list)](https://github.com/hunterlawson/wrapped-list/blob/master/LICENSE)

This crate provides the `wrapped_list!` macro which allows you to create a list of elements that are wrapped by an object, function, or another macro at compile time.

```rust
wrapped_list!(Box::new; value_1, value_2, ...)
```

Expands to:

```rust
[Box::new(value_1), Box::new(value_2), ...]
```

With this you can:

- [Wrap values with a tuple struct or enum](#wrap-values-with-a-tuple-struct-or-enum)
- [Wrap values with an object or function](#wrap-values-with-an-object-or-function)
- [Wrap values with a macro](#wrap-values-with-a-macro)

## Examples

### Wrap values with a tuple struct or enum

```rust
use wrapped_list::wrapped_list;

#[derive(Debug, PartialEq, Eq)]
struct Wrapper(i32);

let wrapped_items = wrapped_list!(Wrapper; 1, 2, 3, 4);

assert_eq!(wrapped_items, [Wrapper(1), Wrapper(2), Wrapper(3), Wrapper(4)]);
```

### Wrap values with an object or function

```rust
use wrapped_list::wrapped_list;

let boxed_items = wrapped_list!(Box::new; 1, 2, 3);

assert_eq!(boxed_items, [Box::new(1), Box::new(2), Box::new(3)])
```

```rust
use wrapped_list::wrapped_list;

let func = |x| x * 2;

let doubled = wrapped_list!(func; 1, 2, 3);

assert_eq!(doubled, [2, 4, 6]);
```

### Wrap values with a macro

```rust
use wrapped_list::wrapped_list;

macro_rules! add_one {
    ($e:expr) => {
        $e + 1
    };
}

let one_more = wrapped_list!(add_one!; 1, 2, 3);

assert_eq!(one_more, [2, 3, 4]);
```
