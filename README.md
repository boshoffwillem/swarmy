## Hot reloading your code

```powershell
cargo install cargo-watch
```

`cargo-watch` monitors your source code to trigger commands every time a file changes.

For example:

```powershell
cargo watch -x check
```

will run cargo check after every code change.
This reduces the perceived compilation time:
• you are still in your IDE, re-reading the code change you just made;
• `cargo-watch`, in the meantime, has already kick-started the compilation process;
• once you switch to your terminal, the compiler is already halfway through!
`cargo-watch` supports command chaining as well:

```powershell
cargo watch -x check -x test -x run
```

It will start by running cargo check.
If it succeeds, it launches cargo test.
If tests pass, it launches the application with cargo run.

## Linting your code

You can lint your code using `clippy`.

Install it with:

```powershell
rustup component add clippy
```

Run it with:

```powershell
cargo clippy
```

## Formatting your code

You can format your code using `rustfmt`.

Install it with:

```powershell
rustup component add rustfmt
```

Run it with:

```powershell
cargo fmt
```

## Scan your code for vulnerabilities

You can scan your code using `cargo-audit`.

Install it with:

```powershell
cargo install cargo-audit
```

Run it with:

```powershell
cargo audit
```

## Scan you dependencies for vulnerabilities

You can scan your dependencies using `cargo-deny`.

Install it with:

```powershell
cargo install cargo-deny
```

Run it with:

```powershell
cargo deny check advisories
```
