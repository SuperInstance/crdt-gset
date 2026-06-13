# CRDT: G-Set (Grow-Only Set)

**A state-based Conflict-free Replicated Data Type (CRDT) for distributed sets** where elements can be added but never removed. Merging two G-Sets is simply set union — guaranteeing convergence without coordination.

## Why It Matters

Distributed systems frequently need to track collections of items across replicas: registered nodes, seen transactions, active sessions, delivered messages. The G-Set is the simplest set-based CRDT: it supports only `add` and `contains` operations. Merge is defined as **set union**, which is inherently commutative, associative, and idempotent.

**Key property:** Because elements can never be removed, the G-Set only grows. This makes it correct for tracking monotonic facts — "this event was processed," "this node joined," "this message was delivered." For sets that need removal, see OR-Set or LWW-Register.

**Real-world usage:** Apache Cassandra uses G-Sets internally for tracking seen mutations. Riak's set data type is built on G-Set primitives. Distributed deduplication systems use G-Sets to track processed IDs.

## How It Works

The G-Set wraps a standard `HashSet<T>`. The beauty is that set union (the merge operation) is already a well-understood, correct operation:

**Add:** Simply `insert` into the backing HashSet. O(1) average case.

**Merge:** Iterate over the other set's elements and insert each one. Since `HashSet::insert` is idempotent (inserting an existing element is a no-op), merge is automatically idempotent. The result is the set union: `A ∪ B`.

**Convergence proof:** Two replicas that have received the same set of adds — in any order, with any number of intermediate merges — will contain identical elements. This is because union is commutative (A ∪ B = B ∪ A) and associative ((A ∪ B) ∪ C = A ∪ (B ∪ C)).

The implementation requires `T: Hash + Eq + Clone` — `Hash` and `Eq` for HashSet membership, `Clone` for merging (elements must be copied between sets).

## Quick Start

```rust
use crdt_gset::GSet;

let mut node_a = GSet::new();
let mut node_b = GSet::new();

// Each node adds elements independently
node_a.add(1);
node_a.add(2);
node_b.add(2);
node_b.add(3);

// Merge — result is the union
node_a.merge(&node_b);
assert_eq!(node_a.len(), 3); // {1, 2, 3}
assert!(node_a.contains(&1));
assert!(node_a.contains(&3));
```

## API

### `GSet<T>` where `T: Hash + Eq + Clone`
- `new() -> Self` — Create an empty set
- `add(&mut self, value: T)` — Add an element. O(1) average
- `contains(&self, value: &T) -> bool` — Check membership. O(1) average
- `merge(&mut self, other: &Self)` — Set union with another G-Set. O(m) where m = other set's size
- `len(&self) -> usize` — Number of elements

## Architecture Notes

The G-Set is part of SuperInstance's CRDT collection, used for tracking monotonic membership and presence information across distributed nodes. For sets that support removal, see the OR-Set implementation.

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## License

MIT
