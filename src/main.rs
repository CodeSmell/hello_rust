fn do_something(foo: Foo) {
    println!("Inside do_something: {:#?}", foo);
}

fn do_something_ref(foo: &Foo) {
    println!("Inside do_something_ref: {:#?}", foo);
    //foo.svalue = "Changed"; 
}

fn do_something_else(numAsString: String) {
    println!("Inside do_something_else: {:#?}", numAsString);
}

#[derive(Debug)]
struct Foo {
    value: i32,
    svalue: String,
}
impl Foo {

    fn new(num: i32) -> Self {
        let x = format!("Holder of {}", num);
        Foo { value: num, svalue: x }
    }

    fn hello(&self) {
        println!("Hello from Foo with value: {}", self.value);
    }
}

fn main() {
    let foo = Foo::new(42);
    do_something_ref(&foo);
    foo.hello();
}