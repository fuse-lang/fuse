# Fuse Lang

##### [Fuse](https://fuse-lang.github.io/) is in development, __NOT__ ready for production; We are at version sub 0.1.

[Documentation](https://fuse-lang.github.io/docs/home/) | [Roadmap](https://fuse-lang.github.io/docs/roadmap/) | [Lua Compatiblity](https://fuse-lang.github.io/docs/compatblity/)

### Giving a star to this project can go a long way in order to hit a sooner release date by putting us on the map for both possible contributors and sponsors to see.

```rust
struct Point
  x: Number
  y: Number
end

let point_a = Point { x: 10, y: 20 }
print("X: ${point_a.x}, Y: ${point_a.y}")

point_a.x = -10 // error, `point_a` is immutable

let mut point_b = Point { x: 10, y: 20 }
point_b.x = -10 // fine, `point_b` is mutable
```

### Inspiration

The language itself is directly inspired by [Lua](https://lua.org/), [Rust](https://www.rust-lang.org/), and [TypeScript](https://www.typescriptlang.org/). There are also some ideas taken from [Haskell](https://haskell.org), `Elxir`, and `Kotlin` among others.
Fuse has started as an alternative to [Luau](https://luau-lang.org/) that can compiler to `Lua`. However Fuse isn't advertised as a super-set of Lua; Instead, it is a separate language that can target different versions of Lua(including LuaJit and Luau), Comes with a minimal yet powerful standard library with zero-cost abstractions, Decent interoperability with the target runtime and a suite of tools such as compiler, linter, LSP, testing and a package manager all built into the `fuse` command.

### Possible use-cases

Fuse is easy to learn while keeping most of the good techniques used in functional languages. It comes with `immutable` variables by default but has native support for `mutability` and also would allow you to define global variables. We also have an ownership system similar to Rust's borrow checker but it is much simpler since we have no lifetimes or references* to worry about.
We also have exhaustive pattern matching and Optional types instead of exposing `nil` which would help with writing more safe code in general.

Because of our interoperability with Lua, we are aiming to provide a drop-in replacement for projects that are using `Lua` for scripting and extendability. It is also useful when writing libraries that need to be compatible with a range of different runtimes such as LuaJIT and Lua5.4.

While Lua doesn't support parallel programming, We have an async/await feature in our roadmap which would provide the much-needed concurrency to the ecosystem. Combined with the clear mutability checks and borrow checker we can extend our programs with concurrency without any major hassles.

###### *In Fuse everything other than primitives are reference types.
