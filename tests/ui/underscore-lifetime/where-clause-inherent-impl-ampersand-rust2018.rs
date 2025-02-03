//@ edition:2018
//@ run-rustfix
#![allow(dead_code)]

trait WithType<T> {}
trait WithRegion<'a> { }

#[allow(dead_code)]
struct Foo<T> {
    t: T
}

impl<T> Foo<T>
where
    T: WithType<&u32>
//~^ ERROR `&` without an explicit lifetime name cannot be used here
{ }

fn main() {}
