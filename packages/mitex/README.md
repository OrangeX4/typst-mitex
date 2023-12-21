# [MiTeX](https://github.com/OrangeX4/typst-mitex)

**[LaTeX](https://www.latex-project.org/) support for [Typst](https://typst.app/), powered by [Rust](https://www.rust-lang.org/) and [WASM](https://webassembly.org/).**

[MiTeX](https://github.com/OrangeX4/typst-mitex) processes LaTeX code into an abstract syntax tree (AST). Then it transforms the AST into Typst code and evaluates code into Typst content by `eval` function.

MiTeX is not only **SMALL** but also **FAST**! MiTeX has a size of just about 150 KB, comparing that [texmath](https://github.com/jgm/texmath) has a size of 17 MB. Its rendering speed is nearly indistinguishable from typst native equations, and from [benchmark](./crates/mitex-parser/benches/simple.rs), the speed of parsing input into an AST has reached about 61.04 MB/s.

Thanks to [@Myriad-Dreamin](https://github.com/Myriad-Dreamin), he completed the most complex development work: developing the parser for generating AST.

## Implemented Features

- [x] LaTeX equations support.
- [x] Coloring commands (`\color{red} text`, `\textcolor{red}{text}`).
- [x] Support for various environments, such as aligned, matrix, cases.

## Features to Implement

- [ ] User-defined commands (specification), such as `\newcommand{\mysym}{\alpha}` or bind `\newcommand{\myop}[1]{\operatorname{#1}}` to a typst's native function `let myop(it) = op(upright(it))`.
- [ ] "usepackage" support, which means that you can change set of commands by telling MiTeX to use a list of packages.
- [ ] Text mode support, enabling the rendering entire LaTeX documents in Typst!

## Usage

- Use `mitex-convert` to convert LaTeX code into Typst code in string.
- Use `mi` to embed an inline LaTeX equation into Typst.
- Use `mitex` to embed a block LaTeX equation into Typst.

Following is a simple example of using MiTeX in Typst:

```typst
#import "lib.typ": *
// #import "@preview/mitex:0.1.0": *

#assert.eq(mitex-convert("\alpha x"), "alpha  x ")

Write inline equations like #mi("x") or #mi[y].

Also block equations (this case is from #text(blue.lighten(20%), link("https://katex.org/")[katex.org])):

#mitex(`
  f(x) = \int_{-\infty}^\infty
    \hat f(\xi)\,e^{2 \pi i \xi x}
    \,d\xi
`)
```

![example](examples/example.png)

## Differences between MiTeX and other solutions

MiTeX has different objectives compared to [texmath](https://github.com/jgm/texmath) (a.k.a. [pandoc](https://pandoc.org/)):

- MiTeX focuses on rendering LaTeX content correctly within Typst, leveraging the powerful programming capabilities of WASM and typst to achieve results that are essentially consistent with LaTeX display.
- texmath aims to be general-purpose converters and generate strings that are more human-readable.

For example, MiTeX transforms `\frac{1}{2}_3` into `frac(1, 2)_3`, while texmath converts it into `1 / 2_3`. The latter's display is not entirely correct, whereas the former ensures consistency in display.

Another example is that MiTeX transforms `(\frac{1}{2})` into `\(frac(1, 2)\)` instead of `(frac(1, 2))`, avoiding the use of automatic Left/Right to achieve consistency with LaTeX rendering.

**Certainly, the greatest advantage is that you can directly write LaTeX content in Typst without the need for manual conversion!**

## Submit Issues

If you find missing commands or bugs of MiTeX, please feel free to submit an issue [here](https://github.com/OrangeX4/mitex/issues).