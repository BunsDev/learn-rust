use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T, K, V>
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
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
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

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    let expensive_closure = |num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cacher_with_different_args() {
        let mut cacher = Cacher::new(|a| a);

        let v1 = cacher.value(1);
        let v2 = cacher.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn cacher_with_different_closure_args() {
        let mut c = Cacher::new(|s| s);

        let v1 = c.value("asd");
        let v2 = c.value("qwe");

        assert_eq!(v1, "asd");
        assert_eq!(v2, "qwe");
    }
}
