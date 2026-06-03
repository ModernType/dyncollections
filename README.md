# dyncollections

This library provides a bunch of collections to store trait objects and retrive concrete types from them.

## Example usage

The simplest collection is `DynSet` which is similar to `Vec<T>` but allows you to store different types which implement the same trait and get them back as concrete types.

Example:
```rust
use dyncollections::DynSet;

// Creating our trait and a few types that implement it
trait Test {
    fn message(&self) -> &'static str;
}

#[derive(Debug)]
struct Hello;
impl Hello {
    fn hello_own_fn(&self) -> &'static str {
        "Hello, I'm concrete `Hello` instance"
    }
}
impl Test for Hello {
    fn message(&self) -> &'static str {
        "Hello"
    }
}

#[derive(Debug)]
struct World;
impl World {
    fn world_own_fn(&self) -> &'static str {
        "World, I'm concrete `World` instance"
    }
}
impl Test for World {
    fn message(&self) -> &'static str {
        "World"
    }
}

// We must use the `dynamify!()` macro to make our trait usable with `DynSet`
dynamify!(Test);

let mut set = DynSet::new();

// Every push call returns a key that can be used to get the value back with correct type.
let hello_key = set.push(Hello);
let world_key = set.push(World);

assert_eq!(
    set.get(&hello_key).unwrap().hello_own_fn(),
    "Hello, I'm concrete `Hello` instance"
);
assert_eq!(
    set.get(&world_key).unwrap().world_own_fn(),
    "World, I'm concrete `World` instance"
);

// We can also iterate over `DynSet`. Iterator will yield trait objects
let messages = dynset.iter().map(|i| i.message()).collect::<Vec<_>>();
assert_eq!(messages, vec!["Hello", "World"]);
```

## Other collections

Except for `DynSet`, `dyncollections` also provides `OrdDynMap` and `HashDynMap` which act similarly to their standard library counterparts except that they use similar mechanism to retrieve back type data from trait objects.

## Features

- `impl_std` (*enabled by default*) - implements inner `MakeConcrete<T>` trait for a bunch of standard library types
