use super::linked_list_unique::{LinkedList, Next};
use std::fmt::Debug;

pub fn has_cycle<T: Debug + Copy + PartialEq>(list: &LinkedList<T>) -> (Next<T>, bool) {
    let mut slow = list.header.clone();
    let mut fast = list.header.clone();

    while fast.is_some() {
        fast = LinkedList::walk(fast, 2);
        slow = LinkedList::walk(slow, 1);

        /*
        let v = match fast {
            None => return (None, false),
            Some(ref v) => v,
        };

        if v.borrow().value == slow.clone().unwrap().borrow().value {
            return (fast, true);
        }
        */

        let (v1, v2) = match (&slow, &fast) {
            (Some(v1), Some(v2)) => (v1, v2),
            _ => return (None, false),
        };

        if v1.borrow().value == v2.borrow().value {
            return (fast, true);
        }
    }

    (None, false)
}

pub fn cycle_start<T: Debug + Copy + PartialEq>(list: &LinkedList<T>) -> Next<T> {
    let (mut fast, has_cycle) = has_cycle(list);
    if !has_cycle {
        return None;
    }

    let mut slow = list.header.clone();

    loop {
        let (v1, v2) = match (&slow, &fast) {
            (Some(v1), Some(v2)) => (v1, v2),
            _ => return None,
        };

        if v1.borrow().value == v2.borrow().value {
            return slow;
        }

        slow = LinkedList::walk(slow, 1);
        fast = LinkedList::walk(fast, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_cycle() {
        let mut list = LinkedList::new();
        list.push(1).push(2).push(3).push(2).push(4).push(5);

        let n2 = list.get(1);
        let last = list.last();
        assert_eq!(LinkedList::walk(n2.clone(), 2).unwrap().borrow().value, 4);

        last.unwrap().borrow_mut().next = n2;
        let (next, ok) = has_cycle(&list);
        assert!(ok);
        assert_eq!(next.unwrap().borrow().value, 5);

        assert_eq!(cycle_start(&list).unwrap().borrow().value, 2);
    }
}
