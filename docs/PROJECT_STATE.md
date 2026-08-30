# Rynix — Project State

## Session log

### 2026-08-29, 20:07 IST
- Resolved full environment setup (Windows PATH issue; Rust 1.98.0 confirmed).
- Consolidated a nested-folder mistake into one clean project.
- Finalized design doc (docs/DESIGN.md) and README.
- Committed scaffolding: `cargo run` prints the banner.
- Commits: 4e930ec (design doc), 1e1196c (skeleton).

### 2026-08-30, 22:35 IST
- Built the Token data structures (TokenKind enum + Token struct) with tests.
- Implemented the lexer: whitespace-skipping, integers, single-char
  operators, parentheses, Eof, with line/column positions and clean
  errors for illegal characters.
- Extended the lexer: identifiers, 10 keywords (read-then-classify),
  and the `=` token.
- Changed comment syntax from `//` to `~...~` (delimited, multi-line),
  with an unterminated-comment error. Added a `lettuce`-is-not-`let`
  regression test.
- Wrote learning notes (docs/NOTES.md sections 0–10) and generated a
  printable PDF of the notes (kept outside the repo).
- Commits: e310eed (Token types), 28ffa6a (lexer core),
  5555721 (identifiers/keywords/comments/=), 7c703a2 (~...~ comments).

## Snapshot
- Current version: v0.1.0 — LEXER COMPLETE.
- Current milestone: about to begin Milestone 2 (v0.2) — the parser.

## What works
- Lexer turns source text into a located token stream:
  integers, identifiers, 10 keywords (let, const, if, else, while, for,
  fn, return, true, false), operators `+ - * / =`, parentheses,
  `~...~` multi-line comments, whitespace, and Eof.
- Line/column tracked on every token (for future diagnostics).
- Clean errors for illegal characters and unterminated comments.
- 17 passing tests, no warnings.
- Repo: MohammadSaqlain124/rynix, branch main, all pushed.

## Decisions locked
- Name: Rynix. Extension: .ryx. Implementation: Rust.
- Philosophy: diagnostics-first, DSA-capable, safety-flavoured, GC-backed.
- Strategy: tree-walk interpreter -> bytecode VM -> deferred native back-end.
- Type system: static, nominal, explicit-first (local inference later).
- Memory: value semantics early -> simple mark-sweep GC later.
- Integers: i64 semantics, BUT the lexer stores literals as String so the
  representation stays swappable (i64 -> bignum later won't touch the lexer).
- Comments: `~ ... ~` (deliberately distinct; can be multi-line;
  unterminated is an error).

## Files
- src/lexer/token.rs — Token, TokenKind (+ keyword variants, Equals).
- src/lexer/lexer.rs — the lexer + 15 unit tests.
- src/lexer/mod.rs — module wiring (exposes token + lexer).
- src/main.rs — tokenizes a sample line and prints the tokens.
- docs/DESIGN.md, docs/NOTES.md, docs/PROJECT_STATE.md, README.md.

## Tests
- 17 passing (2 token + 15 lexer), incl. negative tests for illegal
  characters and unterminated comments.

## Known bugs / limitations
- No floats, strings, or multi-char operators (==, !=, <=) yet.
- No parser/AST yet — tokens are still a flat list, not a tree.
- (Cosmetic) project path is doubled: E:\Projects\rynix — harmless.

## Next milestone
- v0.2 — the parser (turn the token stream into an AST).

## Next task
- Start the parser by first defining the AST (Abstract Syntax Tree) data
  structures — "data before algorithm". Begin with the concept: what an
  AST is, and how `1 + 2 * 3` becomes a tree where `*` binds tighter
  than `+`. THEN write the parsing logic. This is the biggest conceptual
  leap so far (grammar, precedence, recursion).