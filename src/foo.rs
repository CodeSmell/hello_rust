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

    // borrowing a mutable reference to self
    pub fn hello(&self) {
        println!("Hello from Foo with value: {} and svalue: {}", self.value, self.svalue);
    }
}

/// Free Functions
pub fn do_something(mut foo: Foo) {
    println!("Inside do_something: {:#?}", foo);
    foo.value += 10;
    foo.svalue = String::from("Changed");
    foo.hello();
}

pub fn do_something_ref(foo: &Foo) {
    println!("Inside do_something_ref: {:#?}", foo);
    //foo.svalue = String::from("Changed");
}

pub fn do_something_interesting(foo: &mut Foo) {
    println!("Inside do_something_interesting: {:#?}", foo);
    foo.value += 10;
    foo.svalue = String::from("Changed");
}

pub fn do_something_mut2(foo: &mut Foo) {
    println!("Inside do_something_mut2: {:#?}", foo);
    foo.svalue = String::from("Changed with a mutable reference");
}

fn do_more_something(foo: &mut Foo, value: String) {
    foo.svalue = value
}

pub fn do_something_else(num_as_string: String) {
    println!("Inside do_something_else: {:#?}", num_as_string);
}

pub fn do_more_something_else(num: i32) {
    println!("Inside do_more_something_else: {:#?}", num);
}