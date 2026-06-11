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
