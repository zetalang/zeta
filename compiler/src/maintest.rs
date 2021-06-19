use codegen::{Block, Function, Scope};

fn main() {
    let mut scope = Scope::new();
    scope
        .new_struct("Foo")
        .derive("Debug")
        .field("one", "usize")
        .field("two", "String");

    // scope.new_module("std").scope().new_module("fs").new_fn();

    scope
        .new_fn("add")
        .vis("pub")
        .arg("a", "i32")
        .arg("b", "i32")
        .ret("i32")
        .line("a + b");

    println!("{}", scope.to_string());
}
/*
#[derive(Debug)]
struct Foo {
    one: usize,
    two: String,
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
*/