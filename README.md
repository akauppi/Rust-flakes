# Rust flakes

<!--
<img src="https://images.fineartamerica.com/images/artworkimages/mediumlarge/1/pile-of-rust-flakes-garry-loss.jpg" title="Architecture Photograph - Pile of Rust Flakes by Garry Loss" alt="Architecture Photograph - Pile of Rust Flakes by Garry Loss" />

<p align=right><font color=gray><i>Image by Garry Loss, 2016, <a href=https://pixels.com/featured/pile-of-rust-flakes-garry-loss.html>pixels.com</a></i></font></p>
<!-_- tbd. license the image; see the gecko!! ;)
-->

Various tidbits of Rust code.

## Big file

Streaming a file.

In Rust, async/concurrency story is still developing (Jan'24). The author is familiar with the [Akka Streams](https://doc.akka.io/docs/akka/current/stream/index.html) ecosystem, and this flake is about how to replicate such, under Rust.

Ideally, using least external crates. O:)


### Setup

Create a file.

```bash
$ for i in {1..100000}; do echo $i; done > file2.txt
```

The file's supposed to be line-wise (is text; has newlines). Otherwise the content doesn't matter.

<!-- delete
>**Creating a big file on macOS**
>
>```
>$ mkfile -n 1g ~/Temp/big_file
>```
-->

### Running

```
$ cargo run --example stream -- file2.txt
```

This would do something with the file in question


<!--
## References
-->

