# Overview

This is a proof-of-concept lexer-based syntax highlighter.
For each language we're interested in, we provide a struct capable of transforming an input string (technically a slice: `&str`) into HTML (typically a bunch of `<span>` tags inside `<pre>...</pre>`).

At the moment, I've only added a lexer for a subset of Scheme.
More to come.

## Usage

Assuming you've cloned the repository,

```
cargo run -- <lexer-name> [<input-file>]
```

will run the lexer indicated by `<lexer-name>` on either the contents of `<input-file>` (if provided), or stdin (if no filename was provided).
Thus it can be chained using pipes:

```
echo my-file.scm | cargo run -- scheme
```

## Example

Suppose the following contents are saved to a file named _./example.scm_:

```scheme
(define quux (+ 2 40))
```

The output of

```
cargo run -- scheme ./example.scm
```

is

```
<pre><span class="delim">(</span><span class="symbol">define</span><span class="whitespace"> </span><span class="symbol">quux</span><span class="whitespace"> </span><span class="delim">(</span><span class="symbol">+</span><span class="whitespace"> </span><span class="number">2</span><span class="whitespace"> </span><span class="number">40</span><span class="delim">)</span><span class="delim">)</span><span class="whitespace">
</span></pre>
```

which is ready to be inserted into an HTML file and styled with CSS.

## Installation

To install on your machine, run

```
cargo install --path <path-to-crate>
```

## Limitations

At the moment, the lexer is not determined until _after_ the input source has been determined and read.
We'd rather check if the indicated lexer is actually defined _before_ going through the work of reading input; however, this will require a slight reorganization.

If input is taken from stdin, **all** input on stdin is consumed before lexing commences.
This is to avoid incorrectly lexing multi-line tokens.
