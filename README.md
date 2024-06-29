# Delegate Trait

This crate provides traits and macros that implement the delegation pattern for traits.

## Minimal example

``` rust
#[delegated]
pub trait MyTrait {
    fn do_stuff(&self) -> String;
}

struct A;
impl MyTrait for A {
    fn do_stuff(&self) -> String {
        "Hello there!".into()
    }
}

struct B {
    a: A
}
delegate_field!(a: A as MyTrait for B)

```
