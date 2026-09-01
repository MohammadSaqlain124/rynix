# Rynix — Learning Notes

My personal notebook for building the Rynix programming language and its
compiler from scratch, in Rust. Written so that even someone who knows
nothing about Rust or compilers can follow along. Updated as we learn.

---

## 0. What are we even building?

A **programming language** called Rynix, and the **compiler** that runs it.

- A *programming language* is a set of rules for writing instructions
  (like Python or C).
- A *compiler* is the program that reads code written in that language
  and turns it into something the computer can actually run.

We're writing our compiler in **Rust** (a fast, safe systems language).

Think of it like inventing a new human language AND also building the
translator that converts it into something others understand.

---

## 1. How a compiler is structured (the big picture)

A compiler is not one big blob. It's a **pipeline** — a series of stages,
each doing one job and passing its result to the next:

    Source code (plain text you write)
      -> Lexer        -> Tokens
      -> Parser       -> AST (a tree)
      -> Name Resolution
      -> Type Checking
      -> IR (intermediate representation)
      -> Code Generation
      -> Runnable output

**Golden rule:** each stage only talks to the next through a clearly
defined result (tokens, tree, etc). No stage reaches into another's
internals. This keeps everything testable and changeable.

Analogy: an assembly line. Each worker does one task and passes the
product along. Nobody reaches back to redo someone else's station.

---

## 2. The Lexer (Stage 1) — what and why

The **lexer** (also called scanner or tokenizer) is the FIRST stage.

**Problem it solves:** source code arrives as one long, flat string of
characters. For example `let x = 42` is just the characters
l, e, t, space, x, space, =, space, 4, 2 — meaningless individually.

**What the lexer does:** it chops that character stream into **tokens** —
meaningful chunks with labels. So `let x = 42` becomes:

    [ Let, Identifier("x"), Equals, Integer("42") ]

Whitespace gets thrown away. Each piece is now labeled.

Analogy: reading a sentence. Before you understand grammar, your brain
first splits the letters into WORDS. "Thecatsat" is hard; "The cat sat"
is easy. The lexer adds those word-boundaries.

**Why it's a separate stage:** the next stage (parser) is far simpler if
it works on clean labeled tokens instead of raw characters. One job per
stage.

---

## 3. Rust concept: enums (sum types)

To represent tokens, we use a Rust **enum**.

In many languages an enum is just a list of names (RED, GREEN, BLUE).
In Rust, an enum is more powerful: **each variant can carry its own
data**, of different types.

Example:

    enum Shape {
        Circle(f64),         // carries one number (radius)
        Rectangle(f64, f64)  // carries two numbers (width, height)
    }

A `Shape` value is EXACTLY ONE of these — never both, never neither.
This is perfect for tokens: a token is exactly one kind, and different
kinds carry different data (a number carries its digits; a `+` carries
nothing).

Bonus: later, when we process tokens, Rust FORCES us to handle every
variant, or it won't compile. The language catches whole classes of bugs
for us. This is a big reason we chose Rust.

---

## 4. The files we made for tokens

### `src/lexer/token.rs` (new)
Defines the data structures the lexer will produce:
- `TokenKind` — an enum of every kind of token (Integer, Identifier,
  Plus, Minus, Star, Slash, LeftParen, RightParen, Eof).
- `Token` — a struct holding a `TokenKind` PLUS its location
  (line, column) in the source.

We built the DATA STRUCTURES first, before the lexing logic. Habit:
"data before algorithm" — define what tokens ARE, then write the code
that makes them.

### `src/lexer/mod.rs` (new)
The "front door" of the lexer module. Right now it just says
`pub mod token;` to connect token.rs into the project.

### `src/main.rs` (modified)
Added `mod lexer;` so the program knows the lexer module exists.

**How Rust finds files:** `main.rs` says `mod lexer;` -> Rust looks for
`src/lexer/mod.rs` -> that says `mod token;` -> Rust looks for
`src/lexer/token.rs`. Each parent explicitly declares its children.
Nothing is visible unless connected this way.

---

## 5. The most important design decision so far

**We store integer literals as text (a `String`), not as a parsed number.**

So the token is `Integer("42")`, not `Integer(42)`.

**Why?** The lexer's only job is to recognize "these characters form a
number." It should NOT decide how that number is stored (64-bit? bignum?).
If the lexer stored `Integer(i64)`, it would be permanently deciding that
Rynix integers are 64-bit — and if we later switch to arbitrary-precision
(bignum) numbers, we'd have to go back and change the lexer.

By keeping it a `String`, the lexer stays ignorant of representation.
The decision "turn these digits into an i64 / bignum" moves to a LATER
stage that's actually responsible for it.

Analogy: a mailroom clerk routes an envelope labeled "number literal"
without opening it. Opening it and doing the accounting is a different
department's job. If the clerk started doing accounting, changing the
accounting software would wrongly force retraining the clerk.

General principle: **each stage makes only the decisions it owns, and
defers everything else.** (This is sometimes called "deferring
commitment" — don't lock in a decision until the stage that needs it.)

---

## 6. Rust concept: `#[derive(...)]`

On top of `TokenKind` and `Token` we wrote:

    #[derive(Debug, Clone, PartialEq)]

This auto-generates standard behavior so we don't hand-write it:
- `Debug`   -> lets us print a value for inspection (great for debugging).
- `Clone`   -> lets us copy a value when needed.
- `PartialEq` -> lets us compare values with `==` (our tests need this).

Deriving is the normal, idiomatic Rust way to get this for free.

---

## 7. Rust concept: tests in the same file

Rust convention: unit tests live in the same file as the code, inside:

    #[cfg(test)]
    mod tests {
        use super::*;   // pull in the code from this file
        ...
    }

- `#[cfg(test)]` means "only compile this when running tests" — it's not
  part of the real program.
- `cargo test` runs them.

Our first two tests just check that a `Token` correctly holds its kind
and location. They prove the data structures work — NOT that lexing
works (there's no lexer yet).

---

## 8. Things that confused me / gotchas

- **Windows PATH:** after installing Rust, the terminal couldn't find
  `cargo`/`rustc` until I opened a FRESH terminal. Environment variables
  are read once when a terminal opens; changes need a new terminal.
- **Doubled folder:** I accidentally created `rynix\rynix` (nested). Cargo
  commands must be run from the folder that has `Cargo.toml`. Always
  check the terminal path before running commands.
- **"never used" warnings:** Rust warned that my token variants are never
  used. That's EXPECTED — nothing creates tokens yet. The lexer will.
  Warnings aren't always errors; sometimes they're just "not yet".

---

## Current status
- Project scaffold works (`cargo run` prints a banner).
- Token data structures defined and tested (2 passing tests).
- Next: write the actual lexer logic — read a source string and produce
  a list of tokens.

---

## 9. The Lexer logic (Stage 1, now actually working)

We built `src/lexer/lexer.rs` — the code that reads real source text and
produces tokens. Before, we only had the token *types*; now we have the
thing that *makes* them.

### How it works: the cursor

The lexer holds:
- `chars`: the source turned into a list of characters (so we can jump to
  any position by number).
- `pos`: how far we've read (starts at 0, marches forward).
- `line`, `column`: where we are in the source, for error messages.

It walks the source one character at a time. At each character it decides:
- whitespace  -> skip it
- a digit     -> read a whole number (loop until digits stop)
- + - * / ( ) -> emit that single-character token
- anything else -> report an error and stop

At the very end it adds one `Eof` token so later stages know where input
ends.

### Two fundamental operations: peek and advance

- `peek()` looks at the current character WITHOUT moving. Used to *decide*.
- `advance()` consumes the current character and moves forward (updating
  line/column). Used to *commit*.

Almost every lexer is built from this pair: peek to look, advance to move.
Separating them is what lets us "look before we leap" — essential for
multi-character tokens.

### Fixed vs variable-length tokens (the key idea)

- `+` is ONE token made of ONE character. Easy: emit and move on.
- `423` is ONE token made of THREE characters. We must LOOP, consuming
  digits until a non-digit appears, collecting them into a string.

That loop lives in `read_number`. This same "keep consuming while a
condition holds" pattern will power identifiers, strings, and keywords
later.

Analogy: reading the number 423 off a page — your eye keeps scanning
right (4, 2, 3) until the digits stop, THEN you know the number.

### Rust concepts that showed up

- `Option<char>`: a value that is either `Some(c)` or `None`. `peek`
  returns this because at the end of input there's no character. Rust
  forces us to handle "there might be nothing here" — preventing
  read-past-the-end bugs.
- `Result<Vec<Token>, String>`: lexing either succeeds (`Ok` with the
  token list) or fails (`Err` with a message). Rust's standard way to
  handle errors that can happen — no exceptions, just a value we must
  check.
- The `?` operator: shorthand for "if this is None/Err, stop and return
  it now."

### Tests we wrote (6 new)

Empty input (only Eof), a single number, a multi-digit number as ONE
token, a full expression `1 + 2 * 3`, parentheses, and a NEGATIVE test
that an illegal character `@` produces an error instead of crashing.
Negative tests prove the compiler fails *gracefully*.

### Files
- `src/lexer/lexer.rs` (new) — the lexer.
- `src/lexer/mod.rs` (updated) — now exposes both `token` and `lexer`.
- `src/main.rs` (updated) — tokenizes a sample expression

---

## 10. Identifiers, keywords, and comments

### Identifiers (names)

An identifier is a name like `x`, `count`, `main`, `_temp1`. The rule:
- The FIRST character must be a letter or underscore.
- The REST can be letters, digits, or underscores.

So `count`, `_temp`, and `x1` are valid; `1x` is not an identifier — it
reads as the number `1` followed by the identifier `x`.

This is handled by `read_identifier_or_keyword`, which uses the SAME loop
pattern as `read_number`: start on a valid first character, then keep
consuming while the next characters are valid. Identifiers are numbers'
cousin.

Two helper functions express the "first vs rest" rule:
- `is_identifier_start`  -> letter or underscore
- `is_identifier_continue` -> letter, digit, or underscore

### Keywords (the "read then classify" trick)

Keywords are reserved words like `let` and `if`. They LOOK exactly like
identifiers (they're just letters). So how does the lexer know `let` is
special but `count` isn't?

The trick: **read the whole word first, THEN check if it's a keyword.**
We read every letter (`l`, `e`, `t`), get the finished word, and look it
up in a keyword list. If it's there -> keyword token. If not -> plain
identifier.

Because the check happens on the COMPLETE word, `lettuce` is never
mistaken for `let` + `tuce`. The lexer reads all of `lettuce`, checks the
list, doesn't find it, and makes it an identifier.

The key line does this compactly:
`keyword_kind(&text).unwrap_or(TokenKind::Identifier(text))`
= "if it's a keyword use that; otherwise, it's an identifier."

### Rynix's keywords (so far)

These 10 words are reserved. The lexer recognizes them now; their exact
meaning is defined later when the parser and semantics reach them.

| Keyword  | Purpose (planned)                     |
|----------|---------------------------------------|
| let      | declare a variable                    |
| const    | declare a constant                    |
| if       | conditional                           |
| else     | the alternative branch of an if       |
| while    | loop while a condition holds          |
| for      | loop over a range/collection          |
| fn       | define a function                     |
| return   | return a value from a function        |
| true     | boolean literal (true)                |
| false    | boolean literal (false)               |

More keywords will be added as the language grows.

### Comments: Rynix uses ~ ... ~

Most languages use `//` or `/* */`. Rynix deliberately uses a tilde on
both sides — `~ this is a comment ~` — mainly to look distinct. Comments
are for humans; the lexer discards them and they never become tokens.

Design consequences (honest notes):
- A Rynix comment can span MULTIPLE lines (like a block comment), which a
  `//` line comment couldn't.
- Because it's delimited by `~` on both sides, forgetting the closing `~`
  would eat the rest of the file. So an unterminated comment is treated
  as an ERROR (with the position where it started), not silently ignored.
- Using `~` for comments means `~` can't also be an operator later. We
  don't need it as one, so that's an acceptable trade.

`skip_comment` consumes the opening `~`, then everything up to the next
`~`. If it reaches the end of input first, it returns an error.

### Files changed
- `src/lexer/token.rs` — added keyword variants (Let, Const, ... False)
  and the `=` token (Equals). (Comments need NO token — they're discarded.)
- `src/lexer/lexer.rs` — added identifier/keyword reading, `~...~`
  comment skipping, and unterminated-comment errors.
- `src/main.rs` — demo now tokenizes `let count = 42 ~ a comment ~`.

### Status
The lexer is complete for our core vocabulary: integers, identifiers, 10
keywords, `+ - * / =`, parentheses, `~...~` comments, whitespace, and
Eof — all with line/column positions, and clean errors for bad input.
16 passing tests.

---

## 11. The AST (Abstract Syntax Tree)

### The problem the AST solves

The lexer gave us a flat LIST of tokens:

    1 + 2 * 3  ->  [ Integer("1"), Plus, Integer("2"), Star, Integer("3") ]

But a flat list doesn't capture MEANING. `1 + 2 * 3` should be 7, not 9,
because `*` binds tighter than `+` (do 2*3 first, then 1+6). The list has
no notion of grouping or priority — everything is at the same level.

We need a structure that captures how the pieces RELATE. That structure
is a TREE — the Abstract Syntax Tree.

### Tree shape encodes meaning

`1 + 2 * 3` becomes:

        +
       / \
      1   *
         / \
        2   3

The shape itself encodes "multiply before add". To compute the top `+`,
you must first compute its children — and its right child is `2 * 3`. So
the `*` runs first even though it's lower.

**Key rule:** the tree shape is decided by PRECEDENCE, not by reading
order.
- Tighter-binding operator (*) -> sinks LOWER -> computed EARLIER.
- Looser-binding operator (+)   -> rises to TOP -> computed LAST.
- The operator at the top of the tree runs LAST.

Proof that reading order doesn't matter: `3 * 2 + 1` (where * appears
first in the text) STILL puts `+` on top, because + binds looser. Same
shape logic as `1 + 2 * 3`.

Parentheses OVERRIDE precedence: `(1 + 2) * 3` forces the + into a group
that must run first, so + sinks low and * rises to the top:

        *
       / \
      +   3
     / \
    1   2

Order of authority: parentheses beat precedence; precedence beats reading
order; reading order doesn't matter at all.

### Why "Abstract"

The tree KEEPS the essential structure and DROPS surface details that
were only there to help write it as text — parentheses, spaces. In
`(1 + 2) * 3` the parentheses were vital in the text, but the tree
already captures that grouping in its SHAPE, so it doesn't store the
parentheses themselves.

Analogy: a family tree captures relationships (who is whose parent), not
where people stood in a photo. The AST is the family tree of the program.

### Expressions are recursive

An expression is:
- a number, OR
- two expressions joined by an operator (left OP right), OR
- ...

"Expression" appears inside its own definition. That recursion is why
programs can nest to any depth (1 + 2 * 3 - 4 / 5). Building a parser is
one of the best ways to truly understand recursion, because you BUILD the
tree, not just walk one.

### The Rust code: src/parser/ast.rs

Two types:

- `Expr` — an enum of expression shapes:
  - `Number(String)` — a number literal, stored as TEXT (same reason as
    the lexer: don't commit to i64 vs bignum; a later phase interprets it).
  - `Binary { left, op, right }` — a binary operation. `left` and `right`
    are themselves expressions (the recursive case). Uses named fields so
    left/right (both the same type) don't get mixed up.

- `BinaryOp` — a small enum of exactly four operators (Add, Subtract,
  Multiply, Divide). A SEPARATE enum (not reusing TokenKind) so the type
  system guarantees an operator is always one of these four — you can't
  build a Binary with a parenthesis as its operator.

### New Rust concept: Box (for recursive types)

A type cannot directly contain itself. If `Binary` held a plain `Expr` on
each side, the type would need INFINITE size (a + holds two Exprs, each
of which could be a + holding two more... forever). Rust needs every type
to have a known, finite size at compile time.

Fix: `Box<Expr>`. A Box is a POINTER to an Expr stored on the heap. A
pointer has a fixed, known size (just an address), no matter how big the
thing it points to is. So the type is finite, and the tree can still nest
to any depth — each level is just a pointer to the next.

To read the value through a Box, use `*` (dereference): `*left` follows
the pointer to get the Expr.

### Data before algorithm (again)

We defined WHAT the tree is before writing the code that BUILDS it. The
tests construct trees BY HAND (e.g. the 1 + 2 * 3 tree) to prove the data
structures can represent the shapes we want. There is NO parser yet — the
parser (next step) will build these same shapes automatically from tokens.
Building them by hand first means we already know what "correct" looks
like.

### Files
- `src/parser/ast.rs` (new) — Expr, BinaryOp, and by-hand tree tests.
- `src/parser/mod.rs` (new) — exposes the ast module.
- `src/main.rs` (updated) — registered the parser module.

### Status
AST data structures done, 3 tests. Dead-code warnings (Subtract, Divide,
Expr, BinaryOp "never used") are EXPECTED — nothing builds these yet. The
parser will.

---

## 12. The Parser (recursive descent)

### What the parser does

The lexer gave us a flat LIST of tokens. The parser reads that list and
builds the TREE (the AST), getting precedence right automatically. It is
the bridge from "words in a row" to "structured meaning".

We use RECURSIVE DESCENT — the most common hand-written parsing method,
and the most readable: the code ends up mirroring the language's grammar,
one function per rule.

### Step 1: a grammar

Before parsing, we describe precisely what a valid expression is. That
description is a GRAMMAR — rules saying "this thing is made of these
smaller things":

    expression -> term (("+" | "-") term)*
    term       -> factor (("*" | "/") factor)*
    factor     -> NUMBER | "(" expression ")"

Reading the notation:
- `|`  means "or".
- `*` at the end of a line means "repeat zero or more times".
- `factor` is the smallest piece: a NUMBER, or a whole expression inside
  parentheses (the recursive case).
- `term` handles * and /: a factor, then zero-or-more of (*|/) factor.
- `expression` handles + and -: same shape, one level up, built from terms.

### Step 2: the key insight — layering IS precedence

The operators are separated by LEVEL:
- + and - live in `expression` (the TOP level).
- * and / live in `term` (the level BELOW).
- `expression` is built out of `term`s.

Because `expression` CALLS `term`, and `term` runs to COMPLETION before
returning, `term` greedily grabs the entire `2 * 3` before `expression`
ever builds the `+`. So the `*` gets bound tightly and sits LOWER in the
tree.

**Deeper rule -> runs to completion first -> binds tighter.**

We never wrote an "if operator is * then higher priority" rule. Precedence
falls out of the grammar's nesting for free. Want a new operator that
binds tighter than *? Add a new level BELOW factor. Looser than +? Add a
level ABOVE expression. Precedence = which level you live on.

### Step 3: "recursive descent" explained

- DESCENT: the parser descends through levels —
  parse_expression -> parse_term -> parse_factor — loosest to tightest.
- RECURSIVE: parse_factor can call parse_expression again (for a
  parenthesized group), so the whole thing loops back on itself. That is
  what lets parentheses nest to any depth: ((42)).

Analogy: nested Russian dolls sorted by rule. To open the outer doll
(expression) you must first fully open the one inside (term), and inside
that (factor). A factor that is ( ... ) contains a whole new set of dolls.

### The code: src/parser/parser.rs

- One function per grammar rule: parse_expression, parse_term,
  parse_factor. Each body mirrors its grammar line almost word for word.
- Mechanical tools (cursor over the token list):
  - peek()   -> look at the current token without consuming.
  - advance() -> consume the current token, move forward.
  - expect(kind) -> consume only if it matches, else a located error.
- parse() is the entry point: parse one expression, then require Eof
  (nothing left over).

### Left-associativity (a real correctness property)

In parse_expression / parse_term, each loop iteration does:

    left = Binary { left: Box::new(left), op, right };

It wraps the tree built SO FAR as the new node's LEFT child. So
`1 - 2 - 3` builds `((1 - 2) - 3)` — the earliest operation ends up
deepest on the left. That is LEFT-associativity, and it is correct:
1 - 2 - 3 means (1-2)-3 = -4, not 1-(2-3) = 2. Most beginners get this
wrong; our loop shape gets it right by construction.

### Diagnostics beginning

`expect` produces located errors ("expected RightParen, found ... at line
X, column Y"). So `(1 + 2` (missing close paren) fails cleanly instead of
crashing. The messages are basic now, but the location-aware MACHINERY is
in place — the start of the "diagnostics-first" goal.

### Honest note: .clone()

The parser clones a few tokens for simplicity. Cloning has a small cost,
but for a learning compiler on small inputs it is fine and keeps the code
clear. Avoiding clones now would be premature optimization. Revisit only
if benchmarks ever show it matters.

### Files
- src/parser/parser.rs (new) — the parser + 9 tests.
- src/parser/mod.rs (updated) — exposes ast and parser.
- src/main.rs (updated) — now lexes AND parses, printing the tree.

### Status
Full pipeline works: source -> tokens -> AST. Handles + - * /, nested
parentheses, correct precedence and left-associativity, located syntax
errors. 29 passing tests, no warnings. `1 + 2 * (3 - 4)` builds the
correctly-shaped tree automatically.

---

## 13. The Interpreter (tree-walking evaluation)

### What it does

We had a tree (the AST). The evaluator WALKS the tree and computes a
single number. For `1 + 2 * 3` it produces `7`. This is the first time
the whole pipeline runs end to end: source -> tokens -> tree -> RESULT.

The method is called TREE-WALKING because you literally traverse the AST
node by node, computing as you go. It is RECURSIVE, like the parser,
because the tree is recursive.

### Two rules (that's the whole evaluator)

- Number node -> interpret its text as an actual i64 integer.
- Binary node -> evaluate the LEFT child, evaluate the RIGHT child, then
  combine them with the operator.

Rule 2 is recursive: to evaluate a Binary, you first evaluate its
children — which may themselves be Binary nodes — all the way down until
you hit Number leaves. The recursion bottoms out at numbers.

Trace of 1 + 2 * 3 (tree: + on top, 2*3 as its right child):
1. Evaluate top +. Need left and right first.
2. Left = Number("1") -> 1.
3. Right = the * node. Evaluate it: 2 * 3 = 6.
4. Back at +: 1 + 6 = 7.

Analogy: totalling a bill. To get the grand total (the top +) you first
total each section (the * subtree). You work from innermost sub-totals
outward — leaves to root.

### Children-first gives precedence for free

The evaluator knows NOTHING about which operator binds tighter. It just
follows one dumb rule: evaluate my children before I combine them.

Because the parser already put the tighter-binding operator LOWER in the
tree (as a child), "children first" computes it earlier — automatically.
Precedence was DECIDED by the parser (in the tree shape) and is merely
OBEYED by the evaluator.

Division of labour worth remembering:
- Precedence is a PARSING concern, decided once, in the tree's shape.
- Evaluation is dumb on purpose — walk children-first, the shape does
  the rest.
- Add a new operator with new precedence -> change the GRAMMAR; the
  evaluator needs ZERO changes.

### The deferred i64 decision finally lands here

Way back, we stored integers as text (String) so the lexer and parser
would not commit to a number representation. The EVALUATOR is the phase
that owns that decision, because it is the phase that actually does
arithmetic.

`text.parse::<i64>()` turns "42" into the i64 value 42. The `::<i64>`
literally names our representation choice. If we ever switch to bignum,
THIS ONE LINE changes; nothing else in the codebase does. This is the
"defer the decision to the phase that owns it" principle paying off
exactly as planned.

### Syntax error vs runtime error (important distinction)

- SYNTAX error: a problem with the FORM of the code. Caught by the
  parser, before anything runs, just by looking at how tokens are
  arranged. Example: `(1 + 2` (missing close paren) — malformed.
- RUNTIME error: a problem that only appears WHILE executing. The code is
  shaped correctly, but doing what it says hits an impossible operation.
  Example: `10 / 0` — a perfectly valid tree, but you cannot divide by
  zero.

Parsing asks "is this valid code?"; evaluation asks "does running this
valid code work?" That is why `eval` returns a Result: some well-formed
trees still cannot be computed.

### Other notes

- Integer division truncates: 7 / 2 = 3 (not 3.5), because i64 division
  discards the remainder. This is a deliberate language decision, proven
  by a test. Floats later will divide differently.
- Division by zero is checked explicitly and returns a clean error
  instead of crashing (Rust would otherwise panic).
- `eval` is a free function (no state) for now. When variables arrive
  (let x = 5), we'll add an ENVIRONMENT to remember values, and eval will
  take that as a parameter. We add state exactly when it's needed, not
  before.
- The `?` operator propagates a child's error straight to the top, so an
  error anywhere in the tree (e.g. 1 + 10/0) surfaces cleanly.

### Files
- src/interpreter/eval.rs (new) — the evaluator + 9 tests.
- src/interpreter/mod.rs (new) — exposes eval.
- src/main.rs (updated) — full pipeline; prints the computed Result.

### Status
Rynix computes. Full pipeline source -> tokens -> AST -> result, with
correct precedence/associativity, integer semantics, division-by-zero
handling. 38 passing tests, no warnings.

---

## 14. Unary minus (prefix operators)

### The problem

Rynix couldn't handle `-5`. Our grammar only knew `-` as a BINARY
operator — something BETWEEN two values (`8 - 3`). But in `-5`, the `-`
sits IN FRONT OF one value. Different shape, no rule for it.

### Binary vs unary operators

- BINARY (infix): between two operands. `a + b`, `a - b`. Needs a left
  AND a right side.
- UNARY (prefix): in front of one operand. `-5`, `-x`, `-(a + b)`. Needs
  only one thing, on its right.

The `-` character now does DOUBLE DUTY: binary in `8 - 3` (subtraction),
unary in `-5` (negation). Same symbol, two roles.

Analogy: the word "left" — "I LEFT the room" (verb) vs "turn LEFT"
(direction). Same word, role decided by WHERE it appears.

### Position decides the role

- A `-` that appears AFTER a complete value -> binary subtraction.
- A `-` that appears WHERE A VALUE IS EXPECTED (start of expression,
  after `(`, after another operator) -> unary negation.

Example `10 - -3`: the first `-` comes after `10` (a complete value) ->
binary. The second appears where a new value is expected -> unary. Result:
10 - (-3) = 13. The parser never "figures out which minus is which" — the
grammar structure routes each one to the right function based on where the
parser is.

### Where unary fits in the grammar

Unary minus binds TIGHTER than * and /: `-2 + 3` is `(-2) + 3 = 1`, not
`-(2 + 3) = -5`. By the "deeper rule = tighter binding" rule, unary sits
BELOW `term`, above `factor`:

    expression -> term (("+" | "-") term)*
    term       -> unary (("*" | "/") unary)*
    unary      -> "-" unary | factor          <- NEW
    factor     -> NUMBER | "(" expression ")"

`term` now calls `unary` instead of `factor`. Because `term` calls
`unary` and `unary` finishes first, negation is fully parsed before * or
/ is applied -> binds tighter.

### The recursion in `unary`

    unary -> "-" unary | factor

`unary` refers to ITSELF. That is what lets `--5` (and `---5`) work: a `-`
followed by a unary, which is another `-` followed by a unary, until it
bottoms out at a factor. A non-recursive rule (`"-" factor | factor`)
would only allow ONE leading minus. `--5` = -(-5) = 5.

The `| factor` alternative is the "no leading minus" path — plain numbers
flow straight through untouched.

### Why the change was small (localized)

Because the parser is one-function-per-rule, adding a precedence level
meant: add ONE function (`parse_unary`) and change ONE word in
`parse_term` (call `parse_unary` instead of `parse_factor`).
`parse_expression` and `parse_factor` were untouched. This is the payoff
of the layered grammar: precedence changes are contained, not a rewrite.

### The evaluator

New AST node `Unary { op, operand }` and `UnaryOp::Negate`. Evaluation:
evaluate the operand (children-first, same principle), then apply the
operator (`Negate` -> `-value`). Three lines. It's a `match` (not an `if`)
on the operator so that adding future unary operators forces us to handle
them.

### Files
- src/parser/ast.rs — added Unary variant + UnaryOp enum.
- src/parser/parser.rs — added parse_unary; term now calls it.
- src/interpreter/eval.rs — added the Negate rule.
- src/main.rs — demo with negation.

### Status
Unary minus works: -5, -(3+2), --5, correct precedence (-2 + 3 = 1,
-2 * 3 = -6), mixed binary/unary (10 - -3 = 13). 49 passing tests.

---

## 15. Variables — statements and the environment

This is the step where Rynix stopped being a calculator (one expression,
one value) and became a real language: a PROGRAM is a sequence of
statements that share memory.

### Statements vs expressions

- EXPRESSION: produces a value. `y * 10`, `x + 2`. You can ask "what does
  it equal?"
- STATEMENT: does an ACTION, not necessarily a value. `let x = 5` creates
  a binding. Asking "what does `let x = 5` equal?" doesn't make sense.

Analogy: a recipe. "2 cups flour + 1 cup sugar" is an EXPRESSION (a
quantity). "Preheat the oven to 180C" is a STATEMENT (an action that
changes state). A program, like a recipe, is a SEQUENCE of statements,
some of which contain expressions.

We keep Stmt and Expr as SEPARATE Rust types, so the type system enforces
the distinction — you can't use a statement where a value is expected.

### The new grammar (a top layer above expressions)

    program              -> statement*
    statement            -> let_statement | expression_statement
    let_statement        -> "let" IDENTIFIER "=" expression
    expression_statement -> expression

A program is zero-or-more statements. A statement is a `let` or a bare
expression. Statements sit ABOVE expressions and CONTAIN them.

### The environment (memory)

When you write `let x = 5`, the evaluator must REMEMBER that x is 5 so a
later statement can use it. That memory is the ENVIRONMENT — a map from
names to values: HashMap<String, i64>.

    { "x" -> 5, "y" -> 7 }

Two operations:
- DEFINE: `env.insert(name, value)` — `let x = 5` stores x -> 5. Because
  insert overwrites, re-declaring (`let x = 10`) just changes x. That's
  our chosen "let defines-or-overwrites" behaviour.
- LOOK UP: `env.get(name)` — evaluating the expression `x` reads its
  value. If the name isn't there -> "undefined variable" error.

### Why the evaluator was stateless before

Three sessions ago, `eval` was a plain function with no state, and we said
"add state when it's needed, not before." THIS is that moment. Before
variables, there was nothing to remember. Now `eval` takes the
environment as a parameter. We didn't build it speculatively — we added it
exactly when the feature that needs it arrived.

### Three levels: run / exec / eval

Mirrors the grammar's layers (program -> statement -> expression):
- run(program)   -> makes one environment, executes each statement in
                    order, threading the SAME env through all of them so
                    later statements see earlier variables. Returns the
                    last expression's value.
- exec(stmt)     -> runs one statement. `let` defines (no value);
                    a bare expression is evaluated and its value returned.
- eval(expr)     -> evaluates one expression, using the env to look up
                    variables.

The SHARED environment threaded through every statement is what makes a
program more than a list of independent expressions — it's the memory
connecting them.

### Rust detail: &mut env vs &env

- exec takes `&mut Env` — a statement (let) can CHANGE the environment.
- eval takes `&Env` — an expression only READS variables, never defines
  them.
The types document who is allowed to modify memory: statements can,
expressions can't. A real language rule enforced by Rust's borrow system.

### The lexer needed NO change

We already tokenized `let`, identifiers, and `=` (built in early). That
foresight paid off — this whole step touched only the AST, parser, and
evaluator.

### Files
- src/parser/ast.rs — added Expr::Identifier, the Stmt type (Let,
  Expression), and Program (= Vec<Stmt>).
- src/parser/parser.rs — parse_program/statement/let_statement; factor
  now accepts an identifier.
- src/interpreter/eval.rs — the environment (HashMap) + run/exec/eval.
- src/main.rs — runs a whole program, prints the final value.

### Status
Multi-statement programs run. Variables are defined, remembered across
statements, looked up, and overwritten by re-let. Undefined variables
error cleanly. `let x = 5; let y = x + 2; y * 10` -> 70. 58 passing tests.