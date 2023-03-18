# wrapped-list

This crate provides the `wrapped_list!` macro which allows you to create a list of elements that are ped by an object, function, or macro at compile time.

## Examples

### Wrap with a tuple struct or enum

```rust
use wrapped_list::*;

#[derive(Debug, PartialEq, Eq)]
struct Wrapper(i32);

let wrapped_items = wrapped_list!(Wrapper; 1, 2, 3, 4);

assert_eq!(wrapped_items, [Wrapper(1), Wrapper(2), Wrapper(3), Wrapper(4)]);
```

### Wrap with an object or function

```rust
use wrapped_list::*;

let boxed_items = wrapped_list!(Box::new; 1, 2, 3);

assert_eq!(boxed_items, [Box::new(1), Box::new(2), Box::new(3)])
```

```rust
use wrapped_list::*;

let func = |x| x * 2;

let doubled = wrapped_list!(func; 1, 2, 3);

assert_eq!(doubled, [2, 4, 6]);
```

### Wrap with a macro

```rust
use wrapped_list::*;

macro_rules! add_one {
    ($e:expr) => {
        $e + 1
    };
}

let one_more = wrapped_list!(add_one!; 1, 2, 3);

assert_eq!(one_more, [2, 3, 4]);
```
