# Milestones

## v1.0 Spec Generation (Shipped: 2026-02-10)

**Delivered:** Complete behavioral specification of all 162 Qwik optimizer snapshot tests, ready to serve as the single source of truth for the OXC optimizer port.

**Phases completed:** 3 phases, 9 plans | 204 files, 166K lines | 2.7 hours
**Git range:** `feat(01-01)` → `feat(03-02)` (5424c29..845a40f)

**Key accomplishments:**
1. Built oxc-ast-util Rust CLI — parses JS/TS/JSX/TSX via oxc_parser 0.113, outputs ESTree JSON AST
2. Generated 162 structured spec files — one per SWC snapshot, with full OXC ASTs and consistent template
3. Automated convention detection — Python scripts detect all 14 CONV types with zero false negatives
4. Structural audit and normalization — 134 files normalized, 0 remaining issues across all 162 specs

**Archives:** `milestones/v1.0-ROADMAP.md`, `milestones/v1.0-REQUIREMENTS.md`, `milestones/v1.0-MILESTONE-AUDIT.md`

---


## v2.0 OXC API Research & Architecture (Shipped: 2026-02-11)

**Delivered:** Complete OXC API mapping for all 14 CONV transformation types, working Rust proof-of-concept programs, and an architectural blueprint — so the v3.0 port can proceed with zero guesswork.

**Phases completed:** 3 phases, 8 plans | 36 files, 16.2K lines | 1 day
**Git range:** `feat(04-01)` → `docs(phase-06)` (44da4cc..5f78633)

**Key accomplishments:**
1. OXC API mapping guide for foundational patterns ($-extraction, QRL wrapping, import rewriting) with 8 complete Rust functions
2. Architecture blueprint with crate module layout, public API design, data flow specification, and Cargo.toml
3. Working Rust POCs for dollar detection, capture analysis, multi-module output, and source maps — all compiling against real spec files
4. Complete OXC API mappings for JSX transforms, signal optimization, props destructuring, entry strategies, code stripping, and const folding (99 Rust code blocks total)
5. Master cross-reference for all 14 CONV types with dependency ordering graph and 9-tier implementation roadmap
6. Span strategy table for 50 node types and PURE annotation mechanism with two implementation options

**Archives:** `milestones/v2.0-ROADMAP.md`, `milestones/v2.0-REQUIREMENTS.md`

---

