# `runner`

This subfolder tries the [runner](https://crates.io/crates/runner) approach, which allows code snippets (ehem, "flakes") to be defined

- ..without surrounding with `main`
- ..without maintaining a Cargo `.toml` file

### Quick course on `runner`

```
$ runner --add time
```

..allows using `time` crate in one's snippets (locally cached).


### Problems ahead

- Confuses IDE: no proper highlighting and no links to actual libraries being used

### Concerns

- `runner` hasn't been updated "in 3 years" (GitHub)
- ..defaults to 2018 edition (not 2021)
- version 0.5.0 reports itself (`runner --version`) as 0.4.0

## Requirements

```
$ cargo install runner
```

## Verdict

I may occasionally place flakes under here, but especially the lack of IDE highlighting eats away the pleasure of having an almost-REPL experience.

If there are any other products (especially [RustRover IDE](https://www.jetbrains.com/rust/) integrated ones) that I could have a look - please chime in! 🛎️


