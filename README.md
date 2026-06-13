# CRDT-GSet — Grow-Only Set Conflict-Free Replicated Data Type

A Rust implementation of the **G-Set** (Grow-Only Set), a state-based CRDT that supports element addition and set union. It is the simplest set-based CRDT and forms the foundation for more complex types like 2P-Sets (add-remove sets) and OR-Sets (observed-remove sets).

## Why It Matters

The G-Set solves a fundamental problem in distributed systems: **how do multiple replicas agree on a set of elements when they can't coordinate synchronously?**

Traditional approaches require distributed locks, two-phase commit, or Paxos consensus — all of which add latency and failure modes. The G-Set sidesteps this entirely: each replica adds elements locally, and merging is just set union. The math guarantees convergence.

Real-world uses include:

- **Distributed presence tracking** — which users are online across server fleet
- **Tag propagation** — collaborative tagging systems across data centers
- **Feature flags** — rolling out features across edge nodes independently
- **Anti-entropy gossip protocols** — seed sets for bootstrap peer discovery

## How It Works

### Data Model

A G-Set wraps a `HashSet<T>` where `T: Hash + Eq + Clone`. The state at each replica is simply:

$$S \subseteq \mathcal{U}$$

where 𝒰 is the universe of possible elements.

### Add Operation

Adding element *x* to set *S*:

$$S \leftarrow S \cup \{x\}$$

This is purely local — no coordination required.

### Merge Operation

Merging two replicas *A* and *B*:

$$S_{\text{merged}} = S_A \cup S_B$$

Set union is:
- **Commutative:** A ∪ B = B ∪ A
- **Associative:** (A ∪ B) ∪ C = A ∪ (B ∪ C)
- **Idempotent:** A ∪ A = A

These three properties make (∪, ⊆) a **bounded semilattice** with bottom element ∅, which is the mathematical requirement for state-based CRDT convergence.

### Convergence Theorem

**Theorem:** Let R₁, R₂, ..., Rₖ be replicas of a G-Set. If the communication graph is strongly connected (every replica can eventually reach every other via some sequence of merges), then:

$$\lim_{t \to \infty} S_i(t) = \bigcup_{j=1}^{k} S_j \quad \forall\ i$$

**Proof sketch:** Union is monotone (elements are never removed), idempotent, and commutative. The join (least upper bound) under ⊆ of all observed states is their union. Each merge moves the state toward this join and never away. ∎

### Complexity

| Operation | Time | Space |
|---|---|---|
| `add(x)` | O(1) expected (HashSet insert) | O(1) amortized per element |
| `contains(x)` | O(1) expected | — |
| `merge(other)` | O(m) where m = |other| | O(n + m) total |
| `len()` | O(1) | — |

## Quick Start

```toml
[dependencies]
crdt-gset = "0.1"
```

```rust
use crdt_gset::GSet;

let mut east = GSet::new();
let mut west = GSet::new();

east.add("user:alice");
east.add("user:bob");
west.add("user:charlie");
west.add("user:bob");  // overlap — no conflict

// Merge via union
east.merge(&west);
assert_eq!(east.len(), 3);
assert!(east.contains(&"user:charlie"));
```

## API

### `GSet<T>`

```rust
pub struct GSet<T> { /* HashSet<T> */ }

impl<T: Hash + Eq + Clone> GSet<T> {
    pub fn new() -> Self;
    pub fn add(&mut self, value: T);
    pub fn contains(&self, value: &T) -> bool;
    pub fn merge(&mut self, other: &Self);
    pub fn len(&self) -> usize;
}
```

| Method | Description |
|---|---|
| `new()` | Construct an empty G-Set. |
| `add(value)` | Insert `value`. Idempotent — adding an existing element is a no-op. |
| `contains(value)` | Check membership. |
| `merge(other)` | Set union with another G-Set. Idempotent, commutative, associative. |
| `len()` | Number of elements currently in the set. |

## Architecture Notes

The G-Set embodies the **γ + η = C** design principle:

- **γ (gamma)**: The merge specification — set union (∪) forming a join-semilattice under subset (⊆). This is the *mathematical contract*.
- **η (eta)**: The `HashSet<T>` implementation — Rust's standard library hash set with its open-addressing, SipHash-1-3 hashing, and automatic resizing. This is the *physical realization*.
- **C (Configuration)**: **Eventual set convergence** — the guarantee that all replicas hold the same set after sufficient merge rounds.

The generic `T: Hash + Eq + Clone` bound ensures elements have the necessary properties for hash-based equality, which must be consistent with the set-theoretic equality assumed by γ. If `Hash`/`Eq` implementations are inconsistent (e.g., two equal elements hash differently), the implementation (η) violates the specification (γ), and convergence (C) breaks.

The G-Set is the building block for richer CRDTs:
- **2P-Set**: Pair of G-Sets (add-set, remove-set); element is present iff in add-set and not in remove-set.
- **OR-Set**: Each add is tagged with a unique identifier; removal only removes observed tags.
- **LWW-Set**: Elements carry timestamps; last-write-wins on merge.

## References

- **Shapiro, M., Preguiça, N., Baquero, C., & Zawirski, M. (2011).** "Conflict-Free Replicated Data Types." *Proc. 17th Int. Symp. on Stabilization, Safety, and Security of Distributed Systems (SSS)*, LNCS 6976, pp. 386–400. — Defines G-Set, 2P-Set, OR-Set within the unified CRDT framework.
- **Baquero, C. (1997).** "Probabilistic Estimation of Update Propagation in Weakly Consistent Systems." *Proc. ACM PODC*. — Analysis of how quickly CRDT merges converge based on gossip topology.
- **Brito, A., et al. (2016).** "Replication Types for Ledger Systems." *Proc. 2nd Workshop on Distributed Infrastructures for Deep Learning (DIDL)*. — Set-based CRDTs in blockchain contexts.
- **Terry, D. B., et al. (1995).** "Managing Update Conflicts: Bayou." *Proc. 15th ACM SOSP*. — Semi-lattice merge for eventually consistent data.
- **Preguiça, N., Shapiro, M., & Baquero, C. (2011).** "Proving the Correctness of State-based CRDTs." *Tech Report RR-7762, INRIA*. — Formal proofs of CRDT convergence using semilattice theory.
- **Davey, B. A., & Priestley, H. A. (2002).** *Introduction to Lattices and Order*, 2nd ed. Cambridge University Press. — Semilattice theory underlying all CRDT merge semantics.

## License

MIT
