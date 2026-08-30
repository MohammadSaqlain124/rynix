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