# Rynix — Language Design Document

**Status:** Draft (living document)
**Version:** 0.1.0
**Author:** Mohd Saqlain Hussain (Sam)

---

## 1. What Rynix is

Rynix is a statically-typed, imperative programming language with a
hand-written, multi-phase compiler. It is designed to be a language you
can actually *write algorithms in* — from basic programs up to
data-structures-and-algorithms (DSA) level work — while giving
best-in-class error messages and treating time/space complexity as a
first-class concern.

Rynix is safety-flavoured (no null, explicit types, predictable
behaviour) but garbage-collected. It sits closer to Go than to Rust on
the systems-vs-ergonomics spectrum: safe and simple, not
manual-memory-management.

## 2. Philosophy & design values

In priority order, when values conflict:

1. **Diagnostics first.** Error messages are a feature, not an
   afterthought. Every error points at the exact source span and, where
   possible, suggests a fix. Target: better than the current gold
   standard (rustc).
2. **Explicit over implicit.** The reader should not have to guess types
   or control flow. Clarity beats cleverness.
3. **Predictable cost.** The language should not hide expensive
   operations. Complexity is documented and, eventually, checked.
4. **Small and teachable.** Every feature must be explainable. If a
   feature can't be justified, it doesn't ship.

## 3. Distinguishing angle: complexity-awareness

This is what makes Rynix more than "another student compiler":

- The standard library documents the time and space complexity of every
  operation.
- (Research goal, later) A linter that detects and *warns* about
  accidental super-linear patterns (e.g. an O(n²) built from nested
  linear scans).

This angle is a stated non-trivial goal, not a v1.0 requirement.

## 4. Non-goals (honesty)

- Not a manual-memory / borrow-checked systems language (v1.0).
- Not the fastest language; correctness and clarity come first.
- Not attempting native codegen early — see §7.
- Not implementing every advanced feature; tiers in §8 are ordered and
  we finish each before starting the next.

## 5. Type system

- **Static**, checked at compile time.
- **Nominal** (types are equal by name, not shape).
- **Explicit annotations first**; local (bidirectional) type inference
  added later so `let x = 5` works. Global Hindley–Milner inference is
  an advanced, optional goal.
- **Generics**: intermediate tier. Monomorphization vs boxing decided
  when we reach it.

## 6. Memory model

- **Value semantics** through the early phases (no heap).
- A **simple mark-and-sweep tracing GC** introduced as an explicit
  milestone once heap types (strings, arrays) exist.
- Ownership / region-based memory is an advanced research track, not a
  v1.0 feature.

## 7. Compilation strategy (a progression, not one choice)

1. **Tree-walking interpreter** — fastest path to a running language;
   validates semantics and the type checker.
2. **Custom IR + bytecode VM** — the highest-value learning phase.
3. **Native / portable back-end** — chosen at Phase 5. Candidates:
   WebAssembly, Cranelift, hand-rolled x86-64. Deliberately deferred.

## 8. Language features (tiers, built strictly in order)

- **Core (v0.1–v1.0):** int/float/bool/string literals, variables &
  constants, arithmetic & comparison operators, if/else, while & for,
  functions with typed params & returns, lexical scoping, print/I/O,
  comments.
- **Intermediate (post-1.0):** arrays, structs, enums, pattern matching,
  modules, closures, error handling, generics.
- **Advanced:** iterators, standard library, maps, traits, concurrency.
- **Experimental / research:** ownership, compile-time evaluation,
  macros, complexity linter, incremental compilation.

## 9. Compiler architecture (phase boundaries are sacred)

    Source
      -> Lexer      -> Tokens
      -> Parser     -> AST
      -> Name Resolution
      -> Type Checking
      -> IR
      -> (Optimization, later)
      -> Code Generation
      -> Runtime + Standard Library

Rule: phases communicate ONLY through their defined data structures
(tokens, AST, IR). No phase reaches into another's internals.

## 10. Repository structure (grows with the project)

Current (v0.1):

    rynix/
    ├── Cargo.toml
    ├── README.md
    ├── .gitignore
    ├── docs/
    │   └── DESIGN.md
    └── src/
        └── main.rs

New directories (lexer/, parser/, ast/, semantic/, types/, ir/,
backend/, runtime/, tests/, examples/, benchmarks/) are added only when
the corresponding phase actually exists — never scaffolded ahead of time.

## 11. Roadmap

- v0.1 — lexer
- v0.2 — parser + AST (arithmetic)
- v0.3 — tree-walk evaluation
- v0.4 — variables, scoping
- v0.5 — functions, recursion
- v0.6 — type checker + great diagnostics
- v0.7 — custom IR
- v0.8 — bytecode VM
- v0.9 — heap types + mark-sweep GC
- v1.0 — usable language, real test corpus, docs

## 12. Diagnostics standard

Every user-facing error must eventually include: an error code, the
source file, a line:column span, a caret pointing at the offending text,
and (where possible) a help/suggestion line. Format may evolve.

## 13. Testing standard

Every feature ships with tests: unit, integration, end-to-end, and
negative (invalid input produces the right error). Every fixed bug
becomes a permanent regression test.

## 14. Open questions (decide later)

- Native back-end target (Phase 5).
- Generics: monomorphization vs boxing.
- Integer model (fixed-width vs arbitrary precision).
- Concurrency model (if any).

## 15. Design-decision changelog

- 2026-xx-xx — Language named Rynix; extension `.ryx`. Implementation
  language: Rust. Philosophy: diagnostics-first, DSA-capable,
  safety-flavoured, GC-backed. Strategy: interpreter -> bytecode VM ->
  deferred native back-end.