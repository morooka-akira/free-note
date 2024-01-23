use core::fmt::Debug;
use std::cmp::max;

trait List<T> {
    fn new() -> Self;
    fn add(&mut self, item: T);
    fn remove(&mut self) -> T;
    fn resize(&mut self);
}

#[derive(Debug)]
struct Queue<T> {
    data: Vec<T>,
    start_index: usize,
    size: usize,
}

impl<T: Copy + Debug + Default> List<T> for Queue<T> {
    fn new() -> Self {
        Queue {
            data: vec![T::default(); 5],
            start_index: 0,
            size: 0,
        }
    }

    fn add(&mut self, item: T) {
        if self.size + 1 >= self.data.len() {
            self.resize();
        }
        println!(
            "size: {}, capacity: {:?}",
            self.data.len(),
            self.data.capacity()
        );
        let len = self.data.len();
        self.data[(self.start_index + self.size) % len] = item;
        self.size += 1;
    }

    fn remove(&mut self) -> T {
        // 先頭から出す
        let item = self.data[self.start_index];
        // 先頭のindexを更新
        self.start_index = (self.start_index + 1) % self.data.len();
        self.size -= 1;
        item
    }

    fn resize(&mut self) {
        let mut new_data: Vec<T> = vec![T::default(); max(2 * self.size, 1)];
        for i in 0..self.size {
            new_data[i] = self.data[(self.start_index + i) % self.data.len()];
        }
        self.data = new_data;
        self.start_index = 0;
    }
}

fn main() {
    let mut queue: Queue<i32> = Queue::new();
    queue.add(1);
    queue.add(2);
    queue.add(3);
    queue.add(4);
    queue.add(5);
    queue.add(6);
    queue.add(7);
    queue.add(8);
    queue.add(9);
    queue.add(10);
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    queue.add(11);
    queue.add(12);
    queue.add(13);
    queue.add(14);
    queue.add(15);
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    queue.add(16);
    queue.add(17);
    queue.add(18);
    queue.add(19);
    queue.add(20);
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    queue.add(21);
    queue.add(22);
    queue.add(23);
    queue.add(24);
    queue.add(25);
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());
    println!("{:?}", queue.remove());

    println!("{:?}", queue);
}
