use aoc::get_args;
use std::collections::HashMap;
use std::fmt;
use std::ptr::NonNull;

struct Node<T> {
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            prev: None,
            next: None,
            value: value,
        }
    }

    fn new_raw(value: T) -> NonNull<Self> {
        unsafe {
            let boxed = Box::new(Self::new(value));
            let mut ptr = NonNull::new_unchecked(Box::into_raw(boxed));
            ptr.as_mut().prev = Some(ptr);
            ptr.as_mut().next = Some(ptr);
            ptr
        }
    }
}

struct Ouroboros<T> {
    current: Option<NonNull<Node<T>>>,
    len: usize,
}

// TODO: Release memory when Ouroboros is dropped
impl<T> Ouroboros<T> {
    fn new() -> Self {
        Self {
            current: None,
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn insert_after(&mut self, value: T) {
        unsafe {
            let mut new_ptr = Node::new_raw(value);

            if let Some(mut current) = self.current {
                let mut next = current.as_mut().next.unwrap();
                std::mem::swap(&mut new_ptr.as_mut().next, &mut current.as_mut().next);
                std::mem::swap(&mut new_ptr.as_mut().prev, &mut next.as_mut().prev);
            } else {
                self.current = Some(new_ptr);
            }
        }
        self.len += 1;
    }

    fn get(&self) -> Option<&T> {
        unsafe {
            if let Some(ref c) = self.current {
                Some(&c.as_ref().value)
            } else {
                None
            }
        }
    }

    fn take(&mut self) -> Option<T> {
        unsafe {
            if let Some(c) = self.current {
                self.len -= 1;

                let curr_ptr = c.as_ptr();
                let prev_ptr = (*curr_ptr).prev.unwrap().as_ptr();
                let next_ptr = (*curr_ptr).next.unwrap().as_ptr();
                std::mem::swap(&mut (*curr_ptr).next, &mut (*prev_ptr).next);
                std::mem::swap(&mut (*curr_ptr).prev, &mut (*next_ptr).prev);

                if self.len() != 0 {
                    self.current = Some(NonNull::new_unchecked(next_ptr));
                } else {
                    self.current = None;
                }

                let node = Box::from_raw(c.as_ptr());
                Some(node.value)
            } else {
                None
            }
        }
    }

    fn move_prev(&mut self) {
        unsafe {
            if let Some(c) = self.current {
                self.current = (*c.as_ptr()).prev;
            }
        }
    }
    fn move_next(&mut self) {
        unsafe {
            if let Some(c) = self.current {
                self.current = (*c.as_ptr()).next;
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Ouroboros<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f, "Ouroboros[")?;

            let mut node = self.current;
            for i in 0..self.len {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", node.unwrap().as_ref().value)?;
                node = node.unwrap().as_ref().next;
            }

            write!(f, "]")
        }
    }
}

fn part_a(players: usize, max_marble: usize) -> usize {
    let mut circle = Ouroboros::new();
    circle.insert_after(0);

    let mut scores = HashMap::with_capacity(players + 1);
    let players = (1..players + 1).into_iter().cycle();
    for (player, marble) in players.zip((1..max_marble + 1).into_iter()) {
        if marble % 23 == 0 {
            for _ in 0..7 {
                circle.move_prev();
            }
            let player_score = scores.entry(player).or_insert(0);
            *player_score += marble + circle.take().unwrap();
        } else {
            circle.move_next();
            circle.insert_after(marble);
            circle.move_next();
        }
    }
    *scores.values().max().unwrap()
}

fn main() {
    let args: Vec<usize> = get_args(2).unwrap();
    println!("Answer A: {}", part_a(args[0], args[1]));
    println!("Answer B: {}", part_a(args[0], args[1] * 100));
}

#[test]
fn test_ouroboros() {
    let mut circle = Ouroboros::new();
    assert_eq!(circle.len(), 0);

    circle.insert_after(1);
    circle.insert_after(3);
    circle.insert_after(2);

    assert_eq!(circle.len(), 3);
    assert_eq!(circle.get(), Some(&1));

    circle.move_next();
    assert_eq!(circle.get(), Some(&2));
    circle.move_next();
    assert_eq!(circle.get(), Some(&3));
    circle.move_next();
    assert_eq!(circle.get(), Some(&1));

    circle.move_prev();
    assert_eq!(circle.get(), Some(&3));

    assert_eq!(circle.take(), Some(3));
    assert_eq!(circle.take(), Some(1));
    assert_eq!(circle.take(), Some(2));
    assert_eq!(circle.len(), 0);

    assert_eq!(circle.take(), None);
    assert_eq!(circle.get(), None);
}

#[test]
fn test_a() {
    assert_eq!(part_a(9, 25), 32);
    assert_eq!(part_a(10, 1618), 8317);
    assert_eq!(part_a(13, 7999), 146373);
    assert_eq!(part_a(17, 1104), 2764);
    assert_eq!(part_a(21, 6111), 54718);
    assert_eq!(part_a(30, 5807), 37305);
}
