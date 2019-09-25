use std::collections::VecDeque;
#[allow(dead_code)]
struct RecentCounter {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    #[allow(dead_code)]
    fn new() -> Self {
        RecentCounter {
            queue: VecDeque::new(),
        }
    }

    #[allow(dead_code)]
    fn ping(&mut self, t: i32) -> i32 {
        while !self.queue.is_empty() && self.queue[0] < t - 3000 {
            self.queue.pop_front();
        }
        self.queue.push_back(t);
        self.queue.len() as i32
    }
}
