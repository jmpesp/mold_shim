An illumos `mold` shim, intended to be called by cargo to link Rust programs:

```
[target.x86_64-unknown-illumos]
linker = "/opt/ooce/bin/clang"
rustflags = ["-C", "link-arg=--ld-path=/home/james/mold_shim/target/release/mold_shim", "-C", "save-temps"]
```

Note you need https://github.com/luqmana/mold/tree/illumos to fix a dlopen related bug.

It can output to `/tmp/args` "invoked" when called, and "passing" with list of args that it calls `mold` with. Uncomment the `std::fs::write` lines in `src/main.rs`!

Combined with `save-temps`, it allows one to rerun failing commands while making changes.

Make sure to change the hard coded `/home/james` paths! If `mold` is installed to something PATH can see, then an absolute path isn't required.

