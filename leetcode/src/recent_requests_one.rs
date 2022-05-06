use std::collections::VecDeque;

static RANGE: i32 = 3000;

pub struct RecentCounter {
    pub queue: VecDeque<i32>
}

#[allow(dead_code)]
impl RecentCounter {
    pub fn new() -> Self {
        RecentCounter {
            queue: VecDeque::new()
        }
    }
    
    pub fn ping(&mut self, t:i32) -> i32 {
        while !self.queue.is_empty() && t - self.queue.front().unwrap() > RANGE{
            self.queue.pop_front();
        }
        
        self.queue.push_back(t);

        self.queue.len() as i32
    }
}

#[test]
fn test_ping() {
    let mut recent_counter = RecentCounter::new();
    recent_counter.queue.push_back(1);
    recent_counter.queue.push_back(100);
    recent_counter.queue.push_back(3001);
    recent_counter.queue.push_back(3002);
    
    println!("size: {}", recent_counter.ping(4005));
}