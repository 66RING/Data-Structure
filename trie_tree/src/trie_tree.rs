use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default)]
struct Node<K: Default, V: Default> {
    children: HashMap<K, Node<K, V>>,
    value: Option<V>,
}

struct Trie<K: Default, V: Default> {
    root: Node<K, V>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl<K, V> Trie<K, V> 
where K: Eq + Hash + Default,
      V: Default
{
    pub fn new() -> Self {
        Self { root: Node::default() }
    }

    pub fn get(&self, key: impl IntoIterator<Item = K>) -> Option<&V> {
        let mut node = &self.root;
        // 逐层下查
        for k in key.into_iter() {
            if node.children.contains_key(&k) {
                node = node.children.get(&k).unwrap();
            } else {
                return None;
            }
        }
        node.value.as_ref()
    }

    pub fn insert(&mut self, key: impl IntoIterator<Item = K> , value: V) {
        let mut node = &mut self.root;
        for k in key.into_iter() {
            node = node.children.entry(k).or_insert(Node::default());
        }
        node.value = Some(value);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_insertion() {
        let mut trie = Trie::new();
        assert_eq!(trie.get("".chars()), None);

        trie.insert("foo".chars(), 1);
        trie.insert("foobar".chars(), 2);

        let mut trie = Trie::new();
        assert_eq!(trie.get(vec![1, 2, 3]), None);

        trie.insert(vec![1, 2, 3], 1);
        trie.insert(vec![3, 4, 5], 2);
    }

    #[test]
    fn test_get() {
        let mut trie = Trie::new();
        trie.insert("foo".chars(), 1);
        trie.insert("foobar".chars(), 2);
        trie.insert("bar".chars(), 3);
        trie.insert("baz".chars(), 4);

        assert_eq!(trie.get("foo".chars()), Some(&1));
        assert_eq!(trie.get("food".chars()), None);

        let mut trie = Trie::new();
        trie.insert(vec![1, 2, 3, 4], 1);
        trie.insert(vec![42], 2);
        trie.insert(vec![42, 6, 1000], 3);
        trie.insert(vec![1, 2, 4, 16, 32], 4);

        assert_eq!(trie.get(vec![42, 6, 1000]), Some(&3));
        assert_eq!(trie.get(vec![43, 44, 45]), None);
    }
}
