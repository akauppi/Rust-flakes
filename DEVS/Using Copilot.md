# Using Copilot

I consider, whether I should mark the Microsoft Copilot as a co-author of this repo. Perhaps I should.

This [code snippet](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d916fad8203ab26d5933aeb9f8f8f7db) about wrapping a closure was created, based on the advice I got from a brief discussion with the Copilot.

```rust
struct Pool<F: Fn() -> ()> {
    pub call_me: F
}

impl<F: Fn() -> ()> Pool<F> {
    fn new(f: F) -> Pool<F> {
        Pool{ call_me: f }
    }
}


fn main() {
    let x = Pool::new(|| println!("called"));
    (x.call_me)();
    (x.call_me)();
}
```

What you should do - instead of banging head and/or googling endlessly - is to use the same approach.

So. Much. More. Fun + Fast!

