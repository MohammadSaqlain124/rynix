# Rynix — Project State

## Session log

### 2026-08-29, 20:07 IST
- Resolved full environment setup (Windows PATH; Rust 1.98.0 confirmed).
- Consolidated a nested-folder mistake into one clean project.
- Finalized design doc (docs/DESIGN.md) and README.
- Committed scaffolding: `cargo run` prints the banner.
- Commits: 4e930ec (design doc), 1e1196c (skeleton).

### 2026-08-30, 22:35 IST
- Built Token data structures + the full lexer (integers, identifiers,
  10 keywords, `+ - * / =`, parens, `~...~` comments, Eof, line/column
  positions, clean errors). Changed comments from `//` to `~...~`.
- Wrote learning notes (NOTES.md sections 0–10) and a printable PDF.
- Commits: e310eed, 28ffa6a, 5555721, 7c703a2.

### 2026-08-31 (started 10:04 IST — add sign-off time)
- Built the AST: Expr enum (Number + recursive Binary using Box),
  BinaryOp enum. Data-before-algorithm.
- Built the recursive-descent parser (grammar: expression -> term ->
  factor). Correct precedence and left-associativity fall out of the
  grammar layering. Located syntax errors via `expect`.
- Built the tree-walking interpreter. Children-first evaluation obeys
  precedence for free. The deferred String->i64 decision landed here.
  Division-by-zero handled as a runtime error.
- Concepts learned: grammar encodes precedence; recursive descent;
  left-associativity; syntax vs runtime errors; deferring representation.
- Notes: NOTES.md sections 11 (AST), 12 (parser), 13 (interpreter).
- Commits: b1c7cc9 (AST), ab5d6d5 (parser), b7d7f57 (interpreter).

## Snapshot
- Current version: v0.3.0 — Rynix computes arithmetic end to end.
- Current milestone: front-end complete for arithmetic; about to extend
  the language (Option B): unary minus, then variables.

## What works
- Full pipeline: source text -> tokens -> AST -> computed integer result.
- Arithmetic: + - * /, nested parentheses, correct precedence and
  left-associativity, integer (i64) semantics.
- Errors at every phase: illegal char / unterminated comment (lexer),
  malformed expression / missing paren (parser), division by zero
  (runtime).
- 38 passing tests, no warnings.
- Repo: MohammadSaqlain124/rynix, branch main, all pushed.

## Architecture (current)
- src/lexer/       — token.rs, lexer.rs, mod.rs
- src/parser/      — ast.rs, parser.rs, mod.rs
- src/interpreter/ — eval.rs, mod.rs
- src/main.rs      — wires the pipeline; prints the result
- docs/            — DESIGN.md, NOTES.md, PROJECT_STATE.md, README.md

## Decisions locked
- Name: Rynix. Extension: .ryx. Implementation: Rust.
- Philosophy: diagnostics-first, DSA-capable, safety-flavoured, GC-backed.
- Strategy: tree-walk interpreter (DONE for arithmetic) -> bytecode VM ->
  deferred native back-end.
- Integers: i64 semantics; literals kept as String through lexer+parser,
  converted to i64 only in the evaluator (swappable to bignum later in
  one line).
- Comments: ~ ... ~ (multi-line; unterminated is an error).
- Grammar layering encodes precedence; the evaluator is precedence-agnostic.

## Known limitations
- Only arithmetic expressions: no unary minus (-5), no variables, no
  statements, no printing from within the language.
- Integers only (no floats, strings, or boolean values yet).
- (Cosmetic) project path is E:\Projects\rynix — clean, single folder.

## Next milestone
- Extend the language (Option B); each feature is now immediately runnable.

## Next task
1. Unary minus (-5, -(3+2)) — a PREFIX operator; small grammar addition
   (new level, e.g. unary -> "-" unary | factor). Teaches prefix parsing.
2. Then variables: let-statements + an ENVIRONMENT (state that remembers
   values). Introduces STATEMENTS (vs expressions) — the step where
   Rynix becomes a real language, not just a calculator.