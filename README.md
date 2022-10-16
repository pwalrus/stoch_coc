# stoch_coc

A stochastic term finder for the calculus of constructions. Implemented in rust.

No CLI or GUI currently written. Code executed via unit tests.

## Syntax

Input and output is in a LaTeX compatible format. It is meant to be a document writing assistent as much as a proof writing assistent.

The exact syntax (a restriction of LaTeX) follows this book as closely as possible.
 * Nederpelt, R.R., & Geuvers, H. (2014). Type Theory and Formal Proof: An Introduction.

Atomic terms can be multi character upper and lower case, with application represented by a space:

```latex
 x A blah blah
```

The 2 sorts are expressed as `\ast` and `\square`.

Abstraction is represented by a lambda, with `:` and `.` to mark the argument type and return:

```latex
\lambda x : A . x
```

Type abstraction is done using the `prod` symbol:

```latex
\prod x : A . A
```

A statement is separated into subject and type by a `:`,

```latex
\lambda x : A . x : \prod x : A . A
```

A judgment (with comma separated context and turnstile `\vdash`) is written like this: 

```latex
A : \ast \vdash \lambda x : A . x : \prod x : A . A
```

### Definitions

Definitions are written first with a context, name and body statement, delimited by `\vartriangleright` and `:=`.

```latex
A : \ast \vartriangleright id \langle A \rangle := \lambda x : A . x : \prod x : A . A
```

This is the one major deviation from the text (Nederpelt,Geuvers 2014). 
We need to distinguish between:
 * application `a (b c)`
 * definition `a \langle b c \rangle`.

This was resolved by changing the bracket style.

## Syntactic Sugar

Basic logical operators are supported in this input/output format.
They are handled in the background in the above syntax.

### Arrow

The arrow `\to` for logical implication:

```latex
A \to B
```

represents:

```latex
\prod x : A . B
```

### Contradiction

The typical symbol for contradiction:

```latex
\perp
```

represents:

```latex
\prod x : A . x
```

### Negation

The typical symbol for negation:

```latex
\neg A
```

represents:

```latex
A \to \perp
```

building upon the previous syntax.

