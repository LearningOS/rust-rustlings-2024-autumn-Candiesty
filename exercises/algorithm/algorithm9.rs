/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;
use std::fmt::Debug;
use std::mem::swap;

pub struct Heap<T>
where
    T: Default + Debug,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        let mut now = self.count;
        while now > 1{
            let parent = self.parent_idx(now);
            if (self.comparator)(&self.items[now],&self.items[parent]){
                let small = self.smallest_child_idx(parent);
                self.items.swap(small, parent);
                println!("parent is {}",parent);
                println!("small is {}",small);
                println!("heap is {:?}",self.items);
            }
            else {
                println!("heap is {:?}",self.items);
                return;
            }
            now = parent;
        }
        println!("heap is {:?}",self.items);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let l = self.left_child_idx(idx);
        let r = self.right_child_idx(idx);
        if r > self.count{
            println!("r is too large");
            return l;
        }
        if (self.comparator)(&self.items[l],&self.items[r]){
            return l;
        }
        r
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.len() == 0{
            None
        }
		else{
            let ret = self.items[1].clone();
            self.items.swap_remove(1);
            self.count -= 1;
            let mut now = 1;
            while now < self.count{
                let child = self.smallest_child_idx(now);
                if child >= self.count{
                    break;
                }
                println!("now is {} child is {} heap is {:?}",now,child,self.items);
                if (self.comparator)(&self.items[now],&self.items[child]){
                    break;
                }
                self.items.swap(now, child);
                now = child;
            }
            Some(ret)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}