# Rust in the browser: three WASM UI options

**Suggested LMS slug:** `rust-wasm-frontend-options`  
**Replaces:** `yew-components-and-demo-video` (drop “demo video” and Yew-only framing).

## Purpose

Before writing UI code, pick an ecosystem. Rust compiles to **WebAssembly** for the browser; these three are the most common **component-style** choices as of this course.

## The three options

### 1. [Yew](https://yew.rs/)

- **Model:** React-inspired components, hooks, and HTML-like macros.
- **Why people use it:** Mature docs, large community, explicit WASM story.
- **Typical tooling:** [`trunk`](https://trunkrs.dev/) to build and serve the WASM bundle.

### 2. [Dioxus](https://dioxuslabs.com/)

- **Model:** React-like `rsx!` and signals; same ideas can target web, desktop, mobile, etc.
- **Why people use it:** Familiar mental model, active development, optional `dx` CLI.
- **Typical tooling:** Trunk or Dioxus’s own tooling for the web target.

### 3. [Leptos](https://leptos.dev/)

- **Model:** Fine-grained reactivity (similar in spirit to SolidJS); strong full-stack story with **Leptos SSR**, but the UI layer also works as WASM in the browser.
- **Why people use it:** Performance story, integrated router and server patterns when you need them.
- **Typical tooling:** `cargo-leptos` for full-stack apps; simpler WASM-only setups are possible.

## How to choose (for this course)

| If you care most about…        | Consider        |
|--------------------------------|-----------------|
| Widest tutorial surface area   | Yew             |
| React-like ergonomics + growth | Dioxus          |
| Reactive primitives + SSR path | Leptos        |

You do **not** need to master all three. **Next lesson:** we pick **one** stack and build a **small, static** front end (layout + routes + placeholder content). Wiring HTTP and state comes after the shell exists.

## Checklist

- [ ] I can name the three options and one sentence about each.
- [ ] I know which one I will use for the next exercise (or I’m following the instructor’s pick).
