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
#[derive(Debug)]
// struct is similar to a class in Java
// defines group of attributes
struct Foo {
    value: i32,
    svalue: String,
}
// inherent implementation block for Foo
// adds methods and associated functions 
impl Foo {
    // new() is a common convention for constructor-like functions in Rust
    // Self is similar to `this` in Java
    // An "associated function" operates on the type itself, not an instance
    // similar to static methods in Java
    fn new(num: i32) -> Self {
        let x = format!("Holder of {}", num);
        // implicit return (last line with no semicolon)
        Foo { value: num, svalue: x }
    }

    // method on Foo
    // This operates on an instance of Foo
    fn hello(&self) {
        println!("Hello from Foo with value: {} and svalue: {}", self.value, self.svalue);
    }
}
```

Now let's examine how we might create and use an instance of Foo

```rust
// the binding (foo) points to a space in memory (Foo instance) 
// and it is the owner of that value
// and it is immutable by default
let foo = Foo::new(42);
// calling a method on the Foo instance
foo.hello();
```

Looking at the way ownership and moving work

```rust
// the binding (foo) points to a space in memory (Foo instance) 
// and it is the owner of that value
// and it is immutable by default
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
- When the owner goes out of scope, the value (and the binding) will be removed from memory

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

## Borrowing rules (immutable reference)
- By default the access to a "borrowed" value (via a reference) is immutable
- We can have multiple immutable references to a value
- An owner can't be moved if there is a reference in scope

This is a problem because we are trying to change the something about the Foo instance

```rust
fn do_something_ref(foo: &Foo) {
    // this is a compiler error
    // we can't change what we borrow
    foo.svalue = String::from("Changed");
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

another example showing we can't move ownership when there is a reference

```rust
    let foo = Foo::new(42);

    // an immutable reference to foo
    let foo_ref = &foo;

    // this is a compiler error 
    // we are attempting to move ownership of Foo instance to the Vector
    // while also having an active reference to foo
    foos.push(foo);

    // attempting to use the immutablereference
    do_something_ref(foo_ref);
```

## Borrowing rules (mutable reference)
By default the foo binding is immutable, so if we want to change something in the Foo instance we need to make sure we add the `mut` prefix. 

The ownership is still "moved" when we pass the foo binding to a method

```rust
    fn do_something_mut(mut foo: Foo) {
        foo.svalue = String::from("Changed");
    }

    // a mutable binding to a Foo instance
    let mut foo = Foo::new(42);
    // this moves the ownership of the instance
    do_something(foo);
    // this is a compiler error 
    foo.hello();
```

We need to use a mutable reference

```rust
    fn do_something_mut2(foo: &mut Foo) {
        foo.svalue = String::from("Changed with a mutable reference");
    }

    // a mutable binding to a Foo instance
    let mut foo = Foo::new(42);
    // let the function "borrow" the instance with a mutable reference
    do_something_mut2(&mut foo);
    // this is NOT a compiler error 
    // foo still owns the instance Foo
    foo.hello();
```

Rules
- If you have a mutable reference to a value, you can have no other references to that value
- the owner can't change the instance when there is a reference to it

```rust
    // a mutable binding to a Foo instance
    let mut foo = Foo::new(42);

    // we create an immutable reference to the Foo instance owned by foo
    let foo_ref = &foo;

    // this is a compiler error 
    // we create a mutable reference to the Foo instance owned by foo
    // and also try to use the immutable reference
    let foo_mut_ref = &mut foo;
    
    foo_ref.hello();
```

Here is an example where we can't change the value via the owner

```rust
    // a mutable binding to a Foo instance
    let mut foo = Foo::new(42);

    // we create a mutable reference to the Foo instance owned by foo
    let foo_mut_ref = &mut foo;

    // this is a compiler error 
    // when we also try to use the mutable reference
    foo.svalue = String::from("new value via the owner");

    do_something_mut2(foo_mut_ref);
```

# Copying
Simple values with a known, fixed size are pushed onto the stack not the heap. These are copied and not moved. Simple values include numbers, char, bool, arrays, references

```rust
     // an immutable binding to a Foo instance
    let foo = Foo::new(42);

    // this is not moving anything
    // it is copying the value from the Foo instance
    // if we were to do this w/ foo.svalue we would be 
    // moving the String and that would cause a compiler error later
    let copied_value = foo.value;

    // this is NOT a compiler error
    // we have not moved foo nor partially moved it
    foo.hello();
```