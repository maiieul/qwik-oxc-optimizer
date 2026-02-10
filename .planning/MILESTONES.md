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

