# crdt-core

Conflict-free replicated data types: G-Counter, PN-Counter, G-Set, OR-Set, LWW-Register.

## Features

- **G-Counter** — Grow-only counter (component-wise max merge)
- **PN-Counter** — Positive-negative counter (increment and decrement)
- **G-Set** — Grow-only set (union merge)
- **OR-Set** — Observed-remove set (add and remove with causal semantics)
- **LWW-Register** — Last-writer-wins register (timestamp + node tiebreaker)

## Modules

| Module | Description |
|--------|-------------|
| `gcounter` | Grow-only counter |
| `pncounter` | Positive-negative counter |
| `gset` | Grow-only set |
| `orset` | Observed-remove set |
| `lww` | Last-writer-wins register |

## Usage

```rust
use crdt_core::gcounter::GCounter;

let mut c1 = GCounter::new();
c1.increment(1);
c1.increment(1);
let mut c2 = GCounter::new();
c2.increment(2);
let merged = c1.merged(&c2);
assert_eq!(merged.value(), 3);
```

## Testing

```bash
cargo test    # 37 tests
cargo clippy  # zero warnings
```

## License

MIT
