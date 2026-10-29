# Testing (chapter 8)

Companion crates for the Testing chapter.

| Folder | Lesson |
|---|---|
| `shop/` | Unit tests, integration tests, and doc tests |
| `find-the-bugs/` | Exercise starter (three planted bugs, no tests yet) |
| `find-the-bugs/solution/` | Fixed `lib.rs` with the three tests |

```console
cd shop && cargo test
cd ../find-the-bugs && cargo test
```

Copy `solution/lib.rs` over `src/lib.rs` in `find-the-bugs` to see the green suite.
