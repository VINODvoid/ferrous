# Rust Learning Roadmap

> Goal: CLI tools, web backends, systems-level programs. Learn by building real things.

---

## How This Works

- Learn topics one by one with hands-on code exercises in `src/main.rs`
- Small exercises after each topic — real code, not throwaway
- Each phase ends with a real project
- After Phase 3 — open source contributions on actual Rust projects

---

## Phase 1 — Foundations ✅
> Understand how Rust thinks. Get comfortable with the compiler.

- [x] Variables, mutability, constants, shadowing
- [x] Data types — scalar (`i32`, `u32`, `f64`, `bool`, `char`) and compound (arrays, tuples)
- [x] Functions — parameters, return types, implicit return
- [x] Control flow — `if/else`, `for`, `while`, `loop`
- [x] Ownership — move, clone, borrow, mutable borrow
- [x] Structs and `impl` — defining types, methods, `&self`
- [x] Enums and `match` — variants with data, exhaustive matching
- [x] Option and Result — replacing null and exceptions
- [x] Modules — `mod`, `pub`, `use`, separate files

### Phase 1 Project — `ferrous-cli`
> Personal knowledge base CLI — add notes, tag them, search, list by date

```
ferrous add "note text" --tag rust
ferrous list
ferrous search "keyword"
ferrous tag rust
```

Status: ✅ Complete

---

## Phase 2 — Intermediate
> Write idiomatic Rust. Use the standard library and external crates confidently.
 
- [x] Closures — anonymous functions, capturing environment
- [x] Iterators — `.map()`, `.filter()`, `.fold()`, chaining, lazy evaluation
- [x] Traits — shared behavior, implementing for your types
- [x] Generics — write code that works for any type
- [x] Error handling — `?` operator, custom error types with `thiserror`
- [x] Collections — `Vec`, `HashMap`, `HashSet`
- [x] `serde` — serialize/deserialize JSON, most used Rust crate
- [ ] `clap` — proper CLI interfaces
- [ ] String types deep dive — `String` vs `&str` in practice

### Phase 2 Project — `logwatch`
> Real-time log file alerter — actual devops tooling

```
logwatch --pattern "ERROR|PANIC" /var/log/app.log
```

Tails a log file, matches regex patterns, prints colored alerts with timestamp and context.

Status: ⬜ Not started

---

## Phase 3 — Advanced
> Concurrency, async, systems-level Rust.

- [ ] Lifetimes — annotating references, lifetime bounds
- [ ] Smart pointers — `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>`
- [ ] Concurrency — threads, `Mutex`, `Arc<Mutex<T>>`, channels
- [ ] Async/await — `async fn`, `.await`, the Future trait
- [ ] Tokio — async runtime, tasks, async I/O
- [ ] Trait objects vs generics — dynamic vs static dispatch
- [ ] Custom iterators — implementing the `Iterator` trait
- [ ] Unsafe Rust — what it is, when it's needed

### Phase 3 Project — Choose One

#### Option A — `ghostchat` (Networking + Async)
P2P terminal chat over LAN. UDP peer discovery, TCP messages, `ratatui` TUI.
Teaches: async/await, Tokio, `Arc<Mutex<T>>`, channels, TCP/UDP

#### Option B — `chip8` (Systems + Emulation)
Working CHIP-8 emulator — runs actual Pong, Space Invaders, Tetris ROMs.
Teaches: bit manipulation, fixed arrays, exhaustive match as opcode dispatch

Status: ⬜ Not started — choose when you get here

---

## Phase 4 — Open Source
> Contribute to real Rust projects used by thousands.

1. Pick a project you actually use
2. Start with `good-first-issue` — docs, tests, small bug fixes
3. Work up to feature contributions

Good starting points: `fd`, `ripgrep`, `zellij`, `helix`

- [ ] First PR merged
- [ ] First feature PR opened
- [ ] Regular contributor to one project

---

## Progress

| Phase | Status |
|---|---|
| Phase 1 — Foundations | ✅ Complete |
| Phase 1 Project — `ferrous-cli` | ✅ Complete |
| Phase 2 — Intermediate | ⬜ In progress |
| Phase 2 Project — `logwatch` | ⬜ Pending |
| Phase 3 — Advanced | ⬜ Not started |
| Phase 3 Project | ⬜ Not started |
| Phase 4 — Open Source | ⬜ Not started |
