# VS Code T27 Language Support

Syntax highlighting for [T27](https://github.com/trinity-ssai/t27) (TRI-27), the ternary language for Trinity S³AI.

## Why `.t27` had no colours

VS Code colours a file only when an extension that knows its language is *installed*. This folder is that
extension, but it was never installed, and the grammar inside it was wrong in three ways:

- it treated every `;` as the start of a comment, so everything after the first `;` of a line (`return x; }`)
  went grey. In t27 `;` is a statement terminator; it is a comment only in **column 1 followed by a space or
  tab** (`bootstrap/src/compiler.rs`, `skip_whitespace_and_comments`);
- it missed `///` doc comments, `#` comments, nested `/* */`, single-quoted literals, `@extern` and the
  `@builtin` calls, hex/binary/float literals and the type names;
- `language-configuration.json` gave `lineComment` as an array (VS Code ignores it, so Cmd+/ did nothing) and
  auto-closed `.` with `:`.

## Install

```bash
code --install-extension vscode-trinity-swe/vscode-trinity-swe-0.2.1.vsix --force
```

For Insiders use `code-insiders`, for Cursor `cursor`. Reload the window afterwards.

## File icon

`.t27` tabs and the Explorer show the 27-petal Trinity logo: white on a transparent background in dark themes
(`icons/dark.png`), the same shape in near-black in light themes (`icons/light.png`). The vector source is
`icons/logo.svg`, the 27 paths of `favicon.svg` in gHashTag/trinity recoloured white. Regenerate the PNGs with
plain commands:

```bash
rsvg-convert -w 256 -h 256 icons/logo.svg -o icons/dark.png
magick icons/dark.png -fill '#1f1f1f' -colorize 100 icons/light.png
```

## Rebuild the `.vsix` after editing the grammar

A `.vsix` is a zip with `[Content_Types].xml`, `extension.vsixmanifest` (bump its version) and an `extension/`
folder that holds this directory's `package.json`, `README.md` as `readme.md`, `LICENSE` as `LICENSE.txt`,
`language-configuration.json`, `syntaxes/`, `icons/`, `dark.png` and `light.png`. Unpack the current package,
replace the changed files under `extension/`, then zip it again:

```bash
zip -X -r vscode-trinity-swe-0.2.1.vsix '[Content_Types].xml' extension.vsixmanifest extension
```

## What is highlighted

- Comments: `//`, `///` (doc), `#`, nested `/* */`, and `; ` in column 1
- Strings `"..."` and `'...'` with escapes
- Declarations: `module`, `fn`, `struct`, `enum`, `union`, `type`, `const`, `let`, `var`, `test`, `invariant`,
  `bench`, `use`, `pragma`, with the declared name coloured
- Control words: `if else for while switch return try break continue in and or`
- `@extern` and builtins such as `@as`, `@intCast`, `@compileAssert`
- Types (`u8 i32 f64 usize bool str Trit`, CamelCase names), `ALL_CAPS` constants, `true false null`
- Numbers: decimal, `0x`, `0b`, floats, exponents, `_` separators and `i32`/`u8` style suffixes
- `.member` access and `.enum_literal`

## License

MIT
