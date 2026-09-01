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
- Wrote learning notes (NOTES.md sections 0-10) and a printable PDF.
- Commits: e310eed, 28ffa6a, 5555721, 7c703a2.

### 2026-08-31 (10:04 start)
- Built the AST (Expr enum, Box for recursion, BinaryOp).
- Built the recursive-descent parser (grammar: expression -> term ->
  factor). Precedence and left-associativity fall out of grammar layering.
- Built the tree-walking interpreter. Children-first evaluation obeys
  precedence for free. String->i64 decision landed here. Division-by-zero
  handled as a runtime error.
- Notes: NOTES.md sections 11 (AST), 12 (parser), 13 (interpreter).
- Commits: b1c7cc9 (AST), ab5d6d5 (parser), b7d7f57 (interpreter).

### 2026-09-01 (add sign-off time)
- Added UNARY MINUS: new `unary` grammar level below `term`, so negation
  binds tighter than * and /; recursion handles --5. UnaryOp::Negate.
- Added VARIABLES: Stmt type (Let, Expression) + Program; Expr::Identifier;
  the ENVIRONMENT (HashMap<String,i64>); run/exec/eval split. Rynix became
  a real language (multi-statement programs sharing memory).
- Added ASSIGNMENT: Stmt::Assign; one-token lookahead (peek_next) to tell
  `x = 10` (assignment) from `x + 2` (expression); existence check so `=`
  only reassigns an existing variable (else error). `let` declares,
  `=` reassigns.
- Debugging: hit "Assign not found" errors from forgetting to save ast.rs;
  fixed by reading errors literally (all pointed at one definition site).
- Notes: NOTES.md sections 14 (unary minus), 15 (variables), 16 (assignment).
- Commits: fb8dfc2 (unary minus), 7f291da (variables), 2dcbca1 (assignment).

## Snapshot
- Current version: v0.4.1 — Rynix runs multi-statement programs with
  variables and mutation.
- Current milestone: Option B essentially complete. Next horizon:
  output (print) and/or control-flow groundwork (booleans + comparisons).

## What works
- Full pipeline: source text -> tokens -> AST -> executed program.
- Arithmetic: + - * /, nested parentheses, correct precedence and
  left-associativity, unary minus (-5, --5, -(3+2)), integer (i64)
  semantics.
- Variables: `let` declares (create/overwrite); `x = value` reassigns an
  existing variable (errors if undefined); assignment uses current values
  (x = x + 1). Variables shared across statements via one environment.
- Errors at every phase: illegal char / unterminated comment (lexer);
  malformed statement / missing paren / let-without-name (parser);
  division by zero / undefined variable / assign-to-undefined (runtime).
- 52 passing tests, no warnings.
- Repo: MohammadSaqlain124/rynix, branch main, all pushed.

## Architecture (current)
- src/lexer/       — token.rs, lexer.rs, mod.rs
- src/parser/      — ast.rs, parser.rs, mod.rs
- src/interpreter/ — eval.rs (Env, run/exec/eval), mod.rs
- src/main.rs      — wires the pipeline; runs a program; prints the result
- docs/            — DESIGN.md, NOTES.md, PROJECT_STATE.md, README.md

## Grammar (current)
    program              -> statement*
    statement            -> let_statement | assignment | expression_statement
    let_statement        -> "let" IDENTIFIER "=" expression
    assignment           -> IDENTIFIER "=" expression
    expression_statement -> expression
    expression -> term (("+" | "-") term)*
    term       -> unary (("*" | "/") unary)*
    unary      -> "-" unary | factor
    factor     -> NUMBER | IDENTIFIER | "(" expression ")"

## Decisions locked
- Name: Rynix. Extension: .ryx. Implementation: Rust.
- Philosophy: diagnostics-first, DSA-capable, safety-flavoured, GC-backed.
- Strategy: tree-walk interpreter (DONE) -> bytecode VM -> deferred native
  back-end.
- Integers: i64 semantics; literals kept as String through lexer+parser,
  converted to i64 only in the evaluator (swappable to bignum in one line).
- Comments: ~ ... ~ (multi-line; unterminated is an error).
- Grammar layering encodes precedence; the evaluator is precedence-agnostic.
- Mutability: `let` declares-or-overwrites; `=` reassigns an existing
  variable only (assign-to-undefined is an error). Chosen deliberately,
  sequenced: simple overwrite first, then explicit assignment.

## Known limitations
- Integers only (no floats, strings, or booleans as values yet).
- No output from within the language (no print) — only the final value
  is shown.
- No control flow (no if / while) and no comparisons yet.
- No functions (fn/return reserved but unused).

## Next milestone
- Choose next session between:
  A) a `print` statement (output mid-run) — small, satisfying.
  B) booleans + comparisons (true/false, <, ==) — groundwork for if/while
     (control flow), the next big leap toward a Turing-complete language.

## Next task
- Decide A vs B (Claude will lay out the trade-off), then implement.