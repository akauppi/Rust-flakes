# Rust flakes

<!--
<img src="https://images.fineartamerica.com/images/artworkimages/mediumlarge/1/pile-of-rust-flakes-garry-loss.jpg" title="Architecture Photograph - Pile of Rust Flakes by Garry Loss" alt="Architecture Photograph - Pile of Rust Flakes by Garry Loss" />

<p align=right><font color=gray><i>Image by Garry Loss, 2016, <a href=https://pixels.com/featured/pile-of-rust-flakes-garry-loss.html>pixels.com</a></i></font></p>

<!_- Image is UNLICENSED. Have tried to be in touch with Garry. DO NOT COMMIT!!!
-->

Various tidbits of Rust code.

## Requirements

- Rust installed, via `rustup`

## Steps

```
$ cargo run --example {futures|...}
```

- `futures` - experiments with [`futures`](https://docs.rs/futures/latest/futures/) streams etc.
- ...

## Note!

One goal is to keep the dependcies relatively shallow. 

Example: [Actix](https://actix.rs) pulled in all of [Tokio](https://tokio.rs). That's considered heavy (but is relative, as all weights are!).



<!-- REMOVE
### Setup

Create a file.

```bash
$ for i in {1..100000}; do echo $i; done > file2.txt
```

The file's supposed to be line-wise (is text; has newlines). Otherwise the content doesn't matter.

>**Creating a big file on macOS**
>
>```
>$ mkfile -n 1g ~/Temp/big_file
>```

### Running

```
$ cargo run --example stream -- file2.txt
```

This would do something with the file in question
-->


<!--
## References
-->

