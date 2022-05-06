use std::convert::TryInto;

static RANGE: usize = 3000;

pub struct Queue<T> {
    pub vec: Vec<T>,
    pub capacity: Option<usize>,
}

impl<T: Clone> From<Vec<T>> for Queue<T> {
    fn from(v: Vec<T>) -> Queue<T> {
        Queue {
            vec: v,
            capacity: None,
        }
    }
}

#[allow(dead_code)]
impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            vec: Vec::new(),
            capacity: None,
        }
    }
    
    pub fn offer(&mut self, item: T) {
		if let Some(capacity) = self.capacity {
			if self.vec.len() >= capacity {
			} else {
				self.vec.push(item);
			}
		} else {
			self.vec.push(item);
		}
	}


    pub fn size(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn peek(&self) -> Option<T> {
        if !self.vec.is_empty() {
            Some(self.vec[0].clone())
        } else {
            None
        }
    }

    pub fn poll(&mut self) -> Option<T> {
        if !self.vec.is_empty() {
            Some(self.vec.remove(0))
        } else {
            None
        }
    }
}

#[allow(dead_code)]
fn ping(t: usize) -> usize{
    let mut queue: Queue<usize> = Queue::new();

    while !queue.is_empty() && queue.peek() < Some((t - RANGE).try_into().unwrap()) {
        queue.poll();
    }
    queue.offer(t);

    queue.size()
}

#[test]
fn test_ping() {

    let mut queue1: Queue<&str> = Queue::new();
    queue1.offer("RecentCounter");
    queue1.offer("ping");
    queue1.offer("ping");
    queue1.offer("ping");
    queue1.offer("ping");

    ping(queue1.size());

    let mut queue2: Queue<Vec<u32>> = Queue::new();

    let vec_one = Vec::new();
    let vec_two = vec![1];
    let vec_three = vec![100];
    let vec_four = vec![3001];
    let vec_five = vec![3002];
    

    queue2.offer(vec_one);
    queue2.offer(vec_two);
    queue2.offer(vec_three);
    queue2.offer(vec_four);
    queue2.offer(vec_five);

    ping(queue2.size());
}