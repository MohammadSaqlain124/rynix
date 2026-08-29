# Rynix — Project State

**Last updated:** 2026-08-29, 20:07 IST

## Snapshot
- Current version: v0.1.0 (scaffolding complete)
- Current milestone: Milestone 1 — lexer (about to write first real code)

## What works
- Cargo project compiles and runs; prints "Rynix v0.1.0 — compiler skeleton".
- Repo live on GitHub (MohammadSaqlain124/rynix), branch `main`, upstream tracking set.
- Clean 2-commit history:
  - 4e930ec — docs: add initial language design document
  - 1e1196c — chore: initialize cargo project skeleton

## What we just implemented
- Full environment setup (resolved Windows PATH issue; Rust 1.98.0 confirmed).
- Consolidated a nested-folder mistake into one clean project at
  E:\Projects\rynix\rynix.
- Finalized design doc (docs/DESIGN.md) and README.

## Decisions locked
- Name: Rynix. Extension: .ryx.
- Implementation language: Rust (straight away).
- Philosophy: diagnostics-first, DSA-capable, safety-flavoured, GC-backed.
- Strategy: tree-walk interpreter -> bytecode VM -> deferred native back-end.
- Type system: static, nominal, explicit-first (local inference later).
- Memory: value semantics early -> simple mark-sweep GC later.

## Files
- Created: docs/DESIGN.md, docs/PROJECT_STATE.md, README.md
- Present: Cargo.toml, src/main.rs, .gitignore, Cargo.lock

## Tests
- None yet. First unit test arrives with the Token type.

## Known bugs / limitations
- Compiler does nothing but print a banner.
- (Cosmetic) Project path is doubled: E:\Projects\rynix\rynix — harmless, left as-is.

## OPEN DECISIONS (needed before next code step)
1. Confirm understanding of Rust enums with data-carrying variants
   (Circle(f64) / Rectangle(f64, f64) shape).
2. Integer model: i64 (fixed 64-bit) vs arbitrary precision.
   - Claude's recommendation: i64 for now, document the limit,
     revisit bignum as a later feature. Reason pending Sam's choice.

## Next milestone
- Still v0.1 — the lexer.

## Next task
- Create src/lexer/token.rs: define TokenKind enum + Token struct + first
  unit test. No lexing logic yet — data structures first.