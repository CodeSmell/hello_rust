#[derive(Debug)]
pub struct Foo {
    value: i32,
    svalue: String,
}

impl Foo {

    pub fn new(num: i32) -> Self {
        let x = format!("Holder of {}", num);
        Foo { value: num, svalue: x }
    }

    // builder pattern 
    // method that takes ownership of self 
    // and returns an updated instance
    pub fn with_svalue(mut self, svalue: String) -> Self {
        self.svalue = svalue;
        self
    }

    // borrowing an immutable reference to self
    pub fn hello(&self) {
        println!("Hello from Foo with value: {} and svalue: {}", self.value, self.svalue);
    }
}

/// Free Functions
/// These are similar to static methods in Java

// takes ownership of Foo instance
pub fn do_something(mut foo: Foo) {
    println!("Inside do_something: {:#?}", foo);
    foo.value += 10;
    foo.svalue = String::from("Changed");
    foo.hello();
}

// borrows an immutable reference to Foo instance
pub fn do_something_simple(foo: &Foo) {
    println!("Inside do_something_simple: {:#?}", foo);
    //foo.svalue = String::from("Changed");
}

// borrows a mutable reference to Foo instance
pub fn do_something_interesting(foo: &mut Foo) {
    println!("Inside do_something_interesting: {:#?}", foo);
    foo.value += 20;
    foo.svalue = String::from("Changed w/ mut ref");
}

// borrows a mutable reference to Foo instance 
// and changes its svalue attribute with the provided value 
pub fn do_something_with_input(foo: &mut Foo, value: String) {
    foo.svalue = value
}

// takes ownership of a String
pub fn do_something_with_string(num_as_string: String) {
    println!("Inside do_something_with_string: {:#?}", num_as_string);
}

// receives a copy of the value
pub fn do_more_something_with_number(num: i32) {
    println!("Inside do_more_something_with_number: {:#?}", num);
}