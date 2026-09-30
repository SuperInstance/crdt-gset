use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct GSet<T> {
    elements: HashSet<T>,
}

impl<T: std::hash::Hash + Eq + Clone> GSet<T> {
    pub fn new() -> Self {
        Self { elements: HashSet::new() }
    }

    pub fn add(&mut self, value: T) {
        self.elements.insert(value);
    }

    pub fn contains(&self, value: &T) -> bool {
        self.elements.contains(value)
    }

    pub fn merge(&mut self, other: &Self) {
        for e in &other.elements {
            self.elements.insert(e.clone());
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gset() {
        let mut s = GSet::new();
        s.add(1);
        s.add(2);
        s.add(1);
        assert_eq!(s.len(), 2);
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
