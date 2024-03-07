# Fuse Lang

##### [Fuse](https://fuse-lang.github.io/) is in development, __NOT__ ready for production; We are at version sub 0.1.

[Documentation](https://fuse-lang.github.io/docs/home/)

## Giving a star to this project can go a long way in order to hit a sooner release date by putting us on the map for both possible contributors and sponsors to see.

```rust
struct Point
	x: Number,
	y: Number,
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
