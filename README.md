# Brust64

A cargo command to encore a bunch of static files into base64 and contained in a rust file as Hashmap.

# TODO

- [ ] check extensions (only html, css, js)
- [ ] check errors when calling unwrap()
- [ ] documentation
- [ ] ignore-extension flag
- [ ] extract code from main and create a cli module
- [ ] testing
- [ ] code security (avoid being root from this command), file injection ?
- [ ] publish on crates.io

# Install

```
cargo install --path .
```

# Usage

```
cargo brust64 -d <src_directory> -o file.rs
```

🏗 WIP...