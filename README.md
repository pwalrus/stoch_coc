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

Primative definitions (or Axioms) are indicated using the `independent` symbol instead of a definition term:

```latex
A : \ast \vartriangleright id \langle A \rangle := \independent : \prod x : A . A
```


## LaTeX Symbols

All of the symbols required are available through `amssymb` and `amsmath`, with the exception of `independent`.

This can be defined separately by the user if needed:

```latex
\def\independent{\perp\!\!\!\perp}
```

Altogether, any judgement/definition/proof can be compiled as a LaTeX document as follows:

```latex
\documentclass{article}
\usepackage{amsmath}
\usepackage{amssymb}

\def\independent{\perp\!\!\!\perp}

\begin{document}

\begin{align*}
A : \ast \vartriangleright id \langle A \rangle := \independent : \prod x : A . A
\end{align*}

\end{document}
```

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

### AND

The typical symbol for and:

```latex
A \wedge B
```

represents:

```latex
\prod C : \ast . (A \to B \to C) \to C
```

building upon the previous `\to` syntax.

### OR

The typical symbol for or:

```latex
A \vee B
```

represents:

```latex
\prod C : \ast . (A \to C) \to (B \to C) \to C
```

building upon the previous `\to` syntax.


## Proof Typesetting

For the Fitch/flag-style natural deduction proofs, [flagderiv](https://www.ctan.org/pkg/flagderiv) is a very nice LaTeX package.
It also appears to be the one used in the text (Nederpelt,Geuvers 2014), which is convenient.

There are many alternatives, which may be implemented later, [Logic Matters](https://www.logicmatters.net/latex-for-logicians/).

In addition to the packages mentioned above, `flagderiv` requires one more import:

```latex
\usepackage{flagderiv}
```

Continueing with the above example judgement:

```latex
A : \ast \vdash \lambda x : A . x : \prod x : A . A
```

The full proof output will be in this format:

```latex
\begin{flagderiv}
   \step*{$(0)$}{\ast : \square}{sort}
   \assume*{}{A : \ast}{}
   \step*{$(1)$}{A : \ast}{var 0}
   \assume*{}{a : A}{}
   \step*{$(2)$}{a : A}{var 1}
   \step*{$(3)$}{A : \ast}{weak 1,1}
   \conclude*[1]{$(4)$}{A \to A : \ast}{form 1,3}
   \step*{$(5)$}{\lambda x : A . x : A \to A}{abst 2,4}
\end{flagderiv}
```

