# Interactive

Learn interactively

```
cargo install --locked trunk
cd shivarthu-client
trunk serve
```

## Tailwindcss

```bash

npx tailwindcss -i ./src/input.css -o ./css/output.css --watch

```


## daisyUI

```bash
npm i -D daisyui@latest
npm i -D flowbite
```

## Leptos format

### Examples

**Single file**

Format a specific file by name

`leptosfmt ./examples/counter/src/lib.rs`

**Current directory**

Format all .rs files within the current directory

`leptosfmt .`

**Directory**

Format all .rs files within the examples directory

`leptosfmt ./examples`

**Glob**

Format all .rs files ending with `_test.rs` within the examples directory

`leptosfmt ./examples/**/*_test.rs`

## Rust format check

`cargo fmt --all -- --check`

## Rust format

`cargo fmt --all`

## Format a file

`rustfmt src/main.rs `


## Redirects
adding a _redirects file on the root of your publish directory,

The content of the _redirects file should be the following:
```
/* /index.html 200
```

## MCQs
```rust
let question_data = McqData {
        question: "".to_string(),
        correct_answer: "".to_string(),
        options: vec![
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ],
        context: r#""#
            .to_string(),
    };
```
