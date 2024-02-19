# Fuse Lang

##### This project is in development, __NOT__ ready for production.

[Documentation](https://fuse-lang.github.io/docs/home/)

## Consider giving this project a star if you would like to see it's release, We need to hit a certain amount of Github stars in order to secure the development funding.

### Inspiration

The language itself is directly inspired by [Lua](https://lua.org/), [Rust](https://www.rust-lang.org/), and [TypeScript](https://www.typescriptlang.org/). There are also some ideas taken from [Haskell](https://haskell.org), `Elxir`, and `Kotlin` among others.
Fuse has started as an alternative to [Luau](https://luau-lang.org/) that can compiler to `Lua`. However Fuse isn't advertised as a super-set of Lua; Instead, it is a separate language that can target different versions of Lua(including LuaJit and Luau), Comes with a minimal yet powerful standard library with zero-cost abstractions, Decent interoperability with the target runtime and a suite of tools such as compiler, linter, LSP, testing and a package manager all built into the `fuse` command.

When it comes to the actual implementation, The [fuse_parser](https://github.com/fuse-lang/fusec/tree/master/crates/fuse-parser/) borrows heavily from how [oxc](https://github.com/oxc-project/oxc/) approaches its lexer. In addition to that, [elm](https://github.com/elm/compiler/), [full-moon](https://github.com/Kampfkarren/full-moon/) and [roc](https://github.com/roc-lang/roc/) also contribute to how the implementation is done.
