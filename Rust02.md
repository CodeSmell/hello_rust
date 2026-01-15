# Ownership
Rust’s typing model describes who owns the data and how long it lives. This is where a lot of complexity in the language Rust comes from.

From the [Rust Programming Language](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html):
> Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. **Rust uses a third approach: Memory is managed through a system of ownership with a set of rules that the compiler checks.** If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

Rust wants to avoid unexpected or surprising changes to values

The rules (for values that are placed on the heap)
- Each value in Rust is "owned" by a single owner
- There can only be one owner at a time for a value

Given we have a struct `Foo` with an inherent implementation. The struct has two attributes. The first is `value` which holds a 32 bit signed integer. The second is `svalue` which is a `String` representation of the number in `value`.

```rust
// the binding (foo) points to a space in memory (Foo instance) 
// and it is the owner of that value
let foo = Foo::new(42);
// this is not a shallow copy since we do NOT have two pointers to
// the same place in memory (where Foo instance is stored)
// Instead this "moves" the value of the Foo instance to the new owner (foo2)
// The instance of Foo is in the same place in memory but only foo2 points to it
let foo2 = foo;
// this is a compiler error 
// it is using the older binding (foo)
foo.hello();
```
Moving the owner can occur in several ways

```rust
// foo is the owner of the Foo instance
let foo = Foo::new(42);
// this "moves" ownership to the function
// and the instance of Foo will be dropped (removed from memory)
// when the function is finished
do_something(foo);
// this is a compiler error 
foo.hello();
```
and another way that can happen

```rust
// foo is the owner of the Foo instance
let foo = Foo::new(42);
// this "moves" ownership to the vector at index 0
let list = vec![foo];
// this is a compiler error
foo.hello();
```

And now for something really messed up

```rust
// foo is the owner of the Foo instance
// and Foo has a holder attribute that is a String
let foo = Foo::new(42);
// this "moves" ownership of foo.svalue to the function
do_something_else(foo.svalue); // "String is 42"
// this is a compiler error (we moved the string)
let my_foo_string = foo.svalue;
// this is a compiler error too
// because foo is partially moved
foo.hello();
```

Another rule
- When the owner goes out of scope, the value will be dropped

Rust does not rely on a garbage collector to remove unused values from the heap. The memory is reclaimed once the owner goes out of scope. Since a value can only have one owner, this makes memory management much easier. 

# Borrowing
Up to now we were calling the method `do_something` and "moving" ownership of the Foo instance.

```rust
fn do_something(foo: Foo) {
    // do something w/ Foo
}
```

Instead, we can create a method that takes a reference to a Foo instance. This means the method is "borrowing" access to it.

```rust
fn do_something_ref(foo: &Foo) {
    // do something w/ Foo
}
```

Now we can do something like this:

```rust
// foo is the owner of the Foo instance
let foo = Foo::new(42);
// let the method "borrow" Foo via the reference
do_something_ref(&foo);
// this is NOT a compiler error 
// foo still owns the instance Foo
foo.hello();
```

Borrowing rules
- By default the access to a "borrowed" value (via a reference) is immutable
- An owner can't be moved if there is a reference in scope

This is a problem because we are trying to change the something about the Foo instance

```rust
fn do_something_ref(foo: &Foo) {
    // this is a compiler error
    // we can't change what we borrow
    foo.svalue = "Changed"; 
}
```

This is also a problem, because we are trying to move an owner while there is a reference to it.

```rust
    let foo = Foo::new(42);
    // we create a reference to the Foo instance owned by foo
    let foo_ref = &foo;
    // we try to move the owner of the Foo instance to a new binding
    // this is a compiler error 
    let foo2 = foo;
    // attempting to use the reference
    do_something_ref(foo_ref);
```