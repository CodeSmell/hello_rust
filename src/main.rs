mod foo;
use crate::foo::{Foo, do_something};

fn foo_sandbox() {
    // a mutable vector to hold Foo instances
    let mut foos = vec![];

    // an immutable binding of a Foo instance
    let foo = Foo::new(42);
    foo.hello();
    
    // move ownership of Foo instance to the vector
    foos.push(foo);
    println!("Initial state of foos vector: {:#?}", foos);

    // this will not compile because foo has moved
    //foo.hello();

    // borrow an immutable reference to the first Foo in the vector
    // which is returned as an Option<&Foo>
    let foo2 = foos.get(0).expect("Panic ensues");
    foo2.hello();

    // borrow a mutable reference to the first Foo in the vector
    // which is returned as an Option<&mut Foo>
    let mut foo3 = foos.get_mut(0).expect("Panic ensues");
    foo::do_something_interesting(&mut foo3);
    foo3.hello();

    // this will panic because there is no 11th element
    //let foo4 = foos.get(10).expect("Panic ensues");
    
    // move ownership of Foo instance out of the vector
    let mut foo4 = foos.pop().expect("Panic ensues");
    foo::do_something_with_input(&mut foo4, String::from("Rust Rocks!"));
    foo4.hello();

    println!("Final state of foos vector: {:#?}", foos);

}

fn main() {
    foo_sandbox();
}