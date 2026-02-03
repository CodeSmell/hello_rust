mod foo;
use crate::foo::{Foo};

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
    // if Option is a None the program will panic
    let foo2 = foos.get(0).unwrap();
    foo2.hello();

    // borrow a mutable reference to the first Foo in the vector
    // which is returned as an Option<&mut Foo>
    let mut foo3 = foos.get_mut(0).expect("Panic ensues");
    foo::do_something_interesting(&mut foo3);
    foo3.hello();

    println!("Status check on foos vector: {:#?}", foos);

    // this will panic because there is no 11th element
    //let foo4 = foos.get(10).expect("Panic ensues");

    // this will not panic because we provide a default Foo instance
    let foo5 = foos.get(10).unwrap_or(&Foo::new(0));
    // but the instance of Foo is dropped so this is a compile error
    //foo5.hello();

    //  but we could do this so we don't panic
    // and can access the default Foo instance
    let default_foo = Foo::new(0).with_svalue(String::from("None"));
    let foo6 = foos.get(10).unwrap_or(&default_foo);
    foo6.hello();

    // move ownership of Foo instance out of the vector
    if let Some(mut foo7) = foos.pop() {
        foo::do_something_with_input(&mut foo7, String::from("Rust Rocks!"));
        foo7.hello();
    }

    println!("Final state of foos vector: {:#?}", foos);

}

fn main() {
    foo_sandbox();
}