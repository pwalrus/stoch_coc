use std::hash::Hash;
use priority_queue::PriorityQueue;
use super::base::{SearchModel};


pub struct SearchControl<T: Hash + Eq> {
    pub model: Box<dyn SearchModel<T>>
}

impl<T: Hash + Eq> SearchControl<T> {

    pub fn search(&self, start: T) -> Result<T, String> {
        let mut queue = PriorityQueue::new();
        let w = self.model.weight(&start);
        queue.push(start, w);
        while !queue.is_empty() {
            let (current, _) = queue.pop().unwrap();
            let mut next = self.model.next(&current);
            let done = next.iter().enumerate().find(|(_,x)| self.model.done(x));
            if done.is_none() {
                for x in next {
                    let w = self.model.weight(&x);
                    queue.push(x, w);
                }
            } else {
                return self.model.finalize(next.remove(done.unwrap().0));
            }
        }
        return Err("Exhausted all search options.".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestNum {
        target: i32
    }

    impl SearchModel<i32> for TestNum {
        fn done(&self, x: &i32) -> bool {
            return x*x <= self.target && (x+1) * (x+1) > self.target;
        }

        fn next(&self, x: &i32) -> Vec<i32> {
            return vec![
                x + 1,
                x - 1,
                x * 2,
                x / 2
            ];
        }

        fn weight(&self, x: &i32) -> i32 {
            return -(x*x - self.target).abs();
        }

        fn finalize(&self, x: i32) -> Result<i32, String> {
            return Ok(x);
        }
    }

    #[test]
    fn tokenize_simple() {
        let control = SearchControl {
            model: Box::new(TestNum { target: 5 })
        };
        let output = control.search(0);
        assert_eq!(output.unwrap(), 2);
    }
}


