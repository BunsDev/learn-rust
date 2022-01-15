pub mod cacher {
    use std::collections::HashMap;

    pub struct Cacher<T, K, V>
    where
        T: Fn(K) -> V,
    {
        calculation: T,
        value: HashMap<K, V>,
    }

    impl<T, K, V> Cacher<T, K, V>
    where
        T: Fn(K) -> V,
        K: std::cmp::Eq + std::hash::Hash + std::marker::Copy,
        V: std::marker::Copy,
    {
        pub fn new(calculation: T) -> Cacher<T, K, V> {
            Cacher {
                calculation,
                value: HashMap::new(),
            }
        }

        pub fn value(&mut self, arg: K) -> V {
            match self.value.get(&arg) {
                Some(v) => *v,
                None => {
                    let res = (self.calculation)(arg);
                    self.value.insert(arg, res);
                    res
                }
            }
        }
    }
}
