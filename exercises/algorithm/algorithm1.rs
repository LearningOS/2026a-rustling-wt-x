/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn merge(mut list_a: Self, mut list_b: Self) -> Self
    where
        T: Ord,
    {
        let mut result = Self::new();
        while list_a.start.is_some() || list_b.start.is_some() {
            let source = match (list_a.start, list_b.start) {
                // SAFETY: Both pointers are live nodes owned by the input lists.
                (Some(a), Some(b)) => if unsafe { a.as_ref().val <= b.as_ref().val } {
                    &mut list_a
                } else {
                    &mut list_b
                },
                (Some(_), None) => &mut list_a,
                (None, Some(_)) => &mut list_b,
                (None, None) => unreachable!(),
            };
            let mut node = source.start.take().unwrap();
            // SAFETY: The inputs are consumed, so this node is exclusively owned.
            // Detach it before transferring ownership to the result list.
            unsafe {
                source.start = node.as_mut().next.take();
                if let Some(mut tail) = result.end {
                    tail.as_mut().next = Some(node);
                } else {
                    result.start = Some(node);
                }
            }
            source.length -= 1;
            if source.start.is_none() { source.end = None; }
            result.end = Some(node);
            result.length += 1;
        }
        result
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.start.take();
        while let Some(ptr) = current {
            // SAFETY: Each allocation was created by Box::into_raw, belongs
            // solely to this list, and is reconstructed and dropped once.
            let node = unsafe { Box::from_raw(ptr.as_ptr()) };
            current = node.next;
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}
#[cfg(test)]
mod edge_tests {
    use super::*;
    #[test]
    fn merge_empty_duplicates_and_append() {
        for (a,b) in [(vec![],vec![]), (vec![1],vec![]), (vec![],vec![2]), (vec![1,1,3],vec![1,2,3])] {
            let mut expected = a.iter().chain(&b).copied().collect::<Vec<_>>();
            expected.sort();
            let mut left = LinkedList::new();
            let mut right = LinkedList::new();
            for value in a { left.add(value); }
            for value in b { right.add(value); }
            let mut merged = LinkedList::merge(left,right);
            assert_eq!(merged.length as usize, expected.len());
            merged.add(9);
            expected.push(9);
            for (i,value) in expected.iter().enumerate() { assert_eq!(merged.get(i as i32), Some(value)); }
            assert_eq!(merged.get(expected.len() as i32), None);
        }
    }
    #[test]
    fn merge_drops_each_owned_value_once() {
        use std::{cell::Cell, rc::Rc};
        #[derive(Eq, PartialEq, Ord, PartialOrd)]
        struct Tracked(i32, Rc<Cell<usize>>);
        impl Drop for Tracked { fn drop(&mut self) { self.1.set(self.1.get() + 1); } }
        let drops = Rc::new(Cell::new(0));
        let mut a = LinkedList::new();
        let mut b = LinkedList::new();
        a.add(Tracked(1, drops.clone()));
        b.add(Tracked(2, drops.clone()));
        let merged = LinkedList::merge(a, b);
        assert_eq!(drops.get(), 0);
        drop(merged);
        assert_eq!(drops.get(), 2);
    }
}
