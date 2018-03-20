trait Foo {
    type Bar: Bar;
}

trait Bar: Copy + Clone {
}