#![forbid(unsafe_code)]

pub mod doublylinkedlist;
pub mod linkedlist;

#[cfg(test)]
mod ll1_tests {
    use super::linkedlist::*;

    #[test]
    fn test_new() {
        let ll1 = LinkedList::<i32>::new();
        assert_eq!(ll1.len(), 0);
        assert!(ll1.get_value(0).is_err());
    }

    #[test]
    fn test_len() {
        let mut ll1 = LinkedList::<i32>::from([]);
        assert_eq!(ll1.len(), 0);
        ll1.push_front(2);
        assert_eq!(ll1.len(), 1);
        ll1.push_back(3);
        assert_eq!(ll1.len(), 2);
        ll1.push(1, 5).unwrap();
        assert_eq!(ll1.len(), 3);
        ll1.pop(1).unwrap();
        assert_eq!(ll1.len(), 2);
        ll1.pop_front().unwrap();
        assert_eq!(ll1.len(), 1);
        ll1.pop_back().unwrap();
        assert_eq!(ll1.len(), 0);

        let mut ll2 = LinkedList::from([2, 3, 5, 7, 11]);
        assert_eq!(ll2.len(), 5);
        ll2.reverse();
        assert_eq!(ll2.len(), 5);
        ll2.clear();
        assert_eq!(ll2.len(), 0);
    }

    #[test]
    fn test_get_value() {
        let ll1 = LinkedList::<i32>::from([]);
        assert!(ll1.get_value(0).is_err());

        let ll2 = LinkedList::from([2, 3, 5]);
        assert_eq!(*ll2.get_value(0).unwrap(), 2);
        assert_eq!(*ll2.get_value(1).unwrap(), 3);
        assert_eq!(*ll2.get_value(2).unwrap(), 5);
    }

    #[test]
    fn test_get_value_mut() {
        let mut ll1 = LinkedList::<i32>::from([]);
        assert!(ll1.get_value_mut(0).is_err());

        let mut ll2 = LinkedList::from([2, 3, 5]);
        assert_eq!(*ll2.get_value_mut(0).unwrap(), 2);
        assert_eq!(*ll2.get_value_mut(1).unwrap(), 3);
        assert_eq!(*ll2.get_value_mut(2).unwrap(), 5);
        *ll2.get_value_mut(0).unwrap() = 7;
        *ll2.get_value_mut(1).unwrap() = 11;
        *ll2.get_value_mut(2).unwrap() = 13;
        let v: Vec<_> = ll2.into();
        assert_eq!(v, vec![7, 11, 13]);
    }

    #[test]
    fn test_set_value() {
        let mut ll1 = LinkedList::<i32>::from([]);
        assert!(ll1.set_value(0, 1).is_err());

        let mut ll2 = LinkedList::from([2, 3, 5]);
        ll2.set_value(0, 7).unwrap();
        ll2.set_value(1, 11).unwrap();
        ll2.set_value(2, 13).unwrap();
        let v: Vec<_> = ll2.into();
        assert_eq!(v, vec![7, 11, 13]);
    }

    #[test]
    fn test_push() {
        let mut ll1 = LinkedList::new();
        assert_eq!(ll1, [].into());
        assert!(ll1.push(1, 0).is_err());
        ll1.push(0, 3).unwrap();
        assert_eq!(ll1, [3].into());
        ll1.push(0, 2).unwrap();
        assert_eq!(ll1, [2, 3].into());
        ll1.push(2, 7).unwrap();
        assert_eq!(ll1, [2, 3, 7].into());
        ll1.push(2, 5).unwrap();
        assert_eq!(ll1, [2, 3, 5, 7].into());
    }

    #[test]
    fn test_push_front() {
        let mut ll1 = LinkedList::new();
        assert_eq!(ll1, [].into());
        ll1.push_front(5);
        assert_eq!(ll1, [5].into());
        ll1.push_front(3);
        assert_eq!(ll1, [3, 5].into());
        ll1.push_front(2);
        assert_eq!(ll1, [2, 3, 5].into());
    }

    #[test]
    fn test_push_back() {
        let mut ll1 = LinkedList::new();
        assert_eq!(ll1, [].into());
        ll1.push_back(2);
        assert_eq!(ll1, [2].into());
        ll1.push_back(3);
        assert_eq!(ll1, [2, 3].into());
        ll1.push_back(5);
        assert_eq!(ll1, [2, 3, 5].into());
    }

    #[test]
    fn test_pop() {
        let mut ll1 = LinkedList::from([2, 3, 5, 7]);
        assert_eq!(ll1, [2, 3, 5, 7].into());
        assert!(ll1.pop(4).is_err());
        ll1.pop(3).unwrap();
        assert_eq!(ll1, [2, 3, 5].into());
        ll1.pop(1).unwrap();
        assert_eq!(ll1, [2, 5].into());
        ll1.pop(0).unwrap();
        assert_eq!(ll1, [5].into());
        ll1.pop(0).unwrap();
        assert_eq!(ll1, [].into());
        assert!(ll1.pop(0).is_err());
    }

    #[test]
    fn test_pop_front() {
        let mut ll1 = LinkedList::from([2, 3, 5]);
        assert_eq!(ll1, [2, 3, 5].into());
        ll1.pop_front().unwrap();
        assert_eq!(ll1, [3, 5].into());
        ll1.pop_front().unwrap();
        assert_eq!(ll1, [5].into());
        ll1.pop_front().unwrap();
        assert_eq!(ll1, [].into());
        assert!(ll1.pop_front().is_err());
    }

    #[test]
    fn test_pop_back() {
        let mut ll1 = LinkedList::from([2, 3, 5]);
        assert_eq!(ll1, [2, 3, 5].into());
        ll1.pop_back().unwrap();
        assert_eq!(ll1, [2, 3].into());
        ll1.pop_back().unwrap();
        assert_eq!(ll1, [2].into());
        ll1.pop_back().unwrap();
        assert_eq!(ll1, [].into());
        assert!(ll1.pop_back().is_err());
    }

    #[test]
    fn test_clear() {
        let mut ll1 = LinkedList::<i32>::new();
        ll1.clear();
        assert_eq!(ll1.len(), 0);
        assert_eq!(ll1, [].into());

        let mut ll2 = LinkedList::from([2, 3, 5, 7]);
        assert_eq!(ll2.len(), 4);
        assert_eq!(ll2, [2, 3, 5, 7].into());
        ll2.clear();
        assert_eq!(ll2.len(), 0);
        assert_eq!(ll2, [].into());
    }

    #[test]
    fn test_reverse() {
        let mut ll1 = LinkedList::<i32>::new();
        assert_eq!(ll1.len(), 0);
        assert_eq!(ll1, [].into());
        ll1.reverse();
        assert_eq!(ll1.len(), 0);
        assert_eq!(ll1, [].into());

        let mut ll2 = LinkedList::from([1]);
        assert_eq!(ll2.len(), 1);
        assert_eq!(ll2, [1].into());
        ll2.reverse();
        assert_eq!(ll2.len(), 1);
        assert_eq!(ll2, [1].into());

        let mut ll3 = LinkedList::from([2, 3]);
        assert_eq!(ll3.len(), 2);
        assert_eq!(ll3, [2, 3].into());
        ll3.reverse();
        assert_eq!(ll3.len(), 2);
        assert_eq!(ll3, [3, 2].into());

        let mut ll4 = LinkedList::from([4, 5, 6]);
        assert_eq!(ll4.len(), 3);
        assert_eq!(ll4, [4, 5, 6].into());
        ll4.reverse();
        assert_eq!(ll4.len(), 3);
        assert_eq!(ll4, [6, 5, 4].into());

        let mut ll2 = LinkedList::from([2, 3, 5, 7, 11, 13, 17, 19, 23]);
        assert_eq!(ll2, [2, 3, 5, 7, 11, 13, 17, 19, 23].into());
        ll2.reverse();
        assert_eq!(ll2, [23, 19, 17, 13, 11, 7, 5, 3, 2].into());
    }

    #[test]
    fn test_iter() {
        let ll1 = LinkedList::<i32>::new();
        let mut ll1_iter = ll1.iter();
        assert_eq!(ll1_iter.next(), None);

        let ll2 = LinkedList::from([2, 3, 5, 7]);
        let mut ll2_iter = ll2.iter();
        assert_eq!(ll2_iter.next(), Some(&2));
        assert_eq!(ll2_iter.next(), Some(&3));
        assert_eq!(ll2_iter.next(), Some(&5));
        assert_eq!(ll2_iter.next(), Some(&7));
        assert_eq!(ll2_iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut ll1 = LinkedList::<i32>::new();
        let mut ll1_iter = ll1.iter_mut();
        assert_eq!(ll1_iter.next(), None);

        let mut ll2 = LinkedList::from([2, 3, 5, 7]);
        let mut ll2_iter = ll2.iter_mut();
        assert_eq!(ll2_iter.next(), Some(&mut 2));
        assert_eq!(ll2_iter.next(), Some(&mut 3));
        assert_eq!(ll2_iter.next(), Some(&mut 5));
        assert_eq!(ll2_iter.next(), Some(&mut 7));
        assert_eq!(ll2_iter.next(), None);

        let ll2_iter = ll2.iter_mut();
        ll2_iter.for_each(|n| *n += 1);
        assert_eq!(ll2, [3, 4, 6, 8].into());
    }

    #[test]
    fn test_default() {
        let ll1 = LinkedList::<i32>::new();
        let ll2 = LinkedList::default();
        let ll3 = Default::default();
        assert_eq!(ll1, ll2);
        assert_eq!(ll1, ll3);
        assert_eq!(ll2.len(), 0);
        assert_eq!(ll3.len(), 0);
    }

    #[test]
    fn test_index() {
        let ll1 = LinkedList::<i32>::new();
        assert!(std::panic::catch_unwind(|| {
            let _a = ll1[0];
        })
        .is_err());

        let ll2 = LinkedList::from([2, 3, 5]);
        assert_eq!(ll2[0], 2);
        assert_eq!(ll2[1], 3);
        assert_eq!(ll2[2], 5);
        assert!(std::panic::catch_unwind(|| {
            let _a = ll2[3];
        })
        .is_err());

        let ll3 = LinkedList::from([
            LinkedList::new(),
            LinkedList::from([1]),
            LinkedList::from([2, 3, 5]),
        ]);
        assert_eq!(ll3[0], LinkedList::new());
        assert_eq!(ll3[1], LinkedList::from([1]));
        assert_eq!(ll3[2], LinkedList::from([2, 3, 5]));
    }

    #[test]
    fn test_index_mut() {
        let ll1 = LinkedList::<i32>::new();
        assert!(std::panic::catch_unwind(|| {
            let mut _a = ll1[0];
        })
        .is_err());

        let mut ll2 = LinkedList::from([2, 3, 5]);
        ll2[0] = 7;
        ll2[1] = 11;
        ll2[2] = 13;
        assert_eq!(ll2[0], 7);
        assert_eq!(ll2[1], 11);
        assert_eq!(ll2[2], 13);
        assert!(std::panic::catch_unwind(|| {
            let mut _a = ll2[3];
        })
        .is_err());

        let mut ll3 = LinkedList::from([
            LinkedList::new(),
            LinkedList::from([1]),
            LinkedList::from([2, 3, 5]),
        ]);
        ll3[0] = LinkedList::from([0]);
        ll3[1] = LinkedList::from([1, 2]);
        ll3[2] = LinkedList::from([3, 4, 5]);
        assert_eq!(ll3[0], LinkedList::from([0]));
        assert_eq!(ll3[1], LinkedList::from([1, 2]));
        assert_eq!(ll3[2], LinkedList::from([3, 4, 5]));
    }

    #[test]
    fn test_from_slice() {
        let ll1: LinkedList<i32> = LinkedList::from(vec![].as_slice());
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from(vec![2, 3, 5, 7].as_slice());
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = vec![].as_slice().into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = vec![2, 3, 5, 7].as_slice().into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_from_mut_slice() {
        let ll1: LinkedList<i32> = LinkedList::from(vec![].as_mut_slice());
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from(vec![2, 3, 5, 7].as_mut_slice());
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = vec![].as_mut_slice().into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = vec![2, 3, 5, 7].as_mut_slice().into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_from_sized_slice() {
        let ll1: LinkedList<i32> = LinkedList::from(&[]);
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from(&[2, 3, 5, 7]);
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = (&[]).into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = (&[2, 3, 5, 7]).into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_from_mut_sized_slice() {
        let ll1: LinkedList<i32> = LinkedList::from(&mut []);
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from(&mut [2, 3, 5, 7]);
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = (&mut []).into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = (&mut [2, 3, 5, 7]).into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_from_array() {
        let ll1: LinkedList<i32> = LinkedList::from([]);
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from([2, 3, 5, 7]);
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = [].into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = [2, 3, 5, 7].into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_from_vec() {
        let ll1: LinkedList<i32> = LinkedList::from(vec![]);
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from(vec![2, 3, 5, 7]);
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = vec![].into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = vec![2, 3, 5, 7].into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_try_into_array() {
        let arr: [_; 0] = LinkedList::<i32>::from([]).try_into().unwrap();
        assert_eq!(arr, []);

        let arr: [_; 4] = LinkedList::from([2, 3, 5, 7]).try_into().unwrap();
        assert_eq!(arr, [2, 3, 5, 7]);

        let arr_result: Result<[_; 1]> = LinkedList::<i32>::from([]).try_into();
        assert!(arr_result.is_err());
    }

    #[test]
    fn test_into_vec() {
        let v: Vec<_> = LinkedList::<i32>::from([]).into();
        assert_eq!(v, vec![]);

        let v: Vec<_> = LinkedList::from([2, 3, 5, 7]).into();
        assert_eq!(v, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_from_iter() {
        let ll1: LinkedList<i32> = [].into_iter().collect();
        assert_eq!(ll1, LinkedList::from([]));

        let ll2: LinkedList<_> = [2, 3, 5, 7].into_iter().collect();
        assert_eq!(ll2, LinkedList::from([2, 3, 5, 7]));
    }

    #[test]
    fn test_into_iter() {
        let ll1 = LinkedList::<i32>::new();
        let v: Vec<_> = ll1.into_iter().collect();
        assert_eq!(v, vec![]);

        let ll2 = LinkedList::from([2, 3, 5, 7]);
        let v: Vec<_> = ll2.into_iter().collect();
        assert_eq!(v, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(LinkedList::<i32>::from([]), LinkedList::from([]));
        assert_eq!(
            LinkedList::from([2, 3, 5, 7]),
            LinkedList::from([2, 3, 5, 7])
        );
        assert_ne!(LinkedList::from([]), LinkedList::from([2, 3, 5, 7]));
        assert_ne!(
            LinkedList::from([1, 2, 3, 4]),
            LinkedList::from([2, 3, 5, 7])
        );
        assert_ne!(
            LinkedList::from([1, 4, 9, 16, 25]),
            LinkedList::from([2, 3, 5, 7])
        );
        assert_ne!(
            LinkedList::from([-2, -3, -5, -7]),
            LinkedList::from([2, 3, 5, 7])
        );
    }
}

#[cfg(test)]
mod ll2_tests {
    use super::doublylinkedlist::*;

    #[test]
    fn test_new() {
        let ll1 = LinkedList::<i32>::new();
        assert_eq!(ll1.len(), 0);
        assert!(ll1.get(0).is_err());
    }

    #[test]
    fn test_len() {
        let mut ll1 = LinkedList::<i32>::from([]);
        assert_eq!(ll1.len(), 0);
        ll1.push_front(2);
        assert_eq!(ll1.len(), 1);
        ll1.push_back(3);
        assert_eq!(ll1.len(), 2);
        ll1.push(1, 5).unwrap();
        assert_eq!(ll1.len(), 3);
        ll1.pop(1).unwrap();
        assert_eq!(ll1.len(), 2);
        ll1.pop_front().unwrap();
        assert_eq!(ll1.len(), 1);
        ll1.pop_back().unwrap();
        assert_eq!(ll1.len(), 0);

        let mut ll2 = LinkedList::from([2, 3, 5, 7, 11]);
        assert_eq!(ll2.len(), 5);
        ll2.reverse();
        assert_eq!(ll2.len(), 5);
        ll2.clear();
        assert_eq!(ll2.len(), 0);
    }

    #[test]
    fn test_get() {
        let ll1 = LinkedList::<i32>::from([]);
        assert!(ll1.get(0).is_err());

        let ll2 = LinkedList::from([2, 3, 5]);
        assert_eq!(ll2.get(0).unwrap(), 2);
        assert_eq!(ll2.get(1).unwrap(), 3);
        assert_eq!(ll2.get(2).unwrap(), 5);
    }

    #[test]
    fn test_set_value() {
        let mut ll1 = LinkedList::<i32>::from([]);
        assert!(ll1.set_value(0, 1).is_err());

        let mut ll2 = LinkedList::from([2, 3, 5]);
        ll2.set_value(0, 7).unwrap();
        ll2.set_value(1, 11).unwrap();
        ll2.set_value(2, 13).unwrap();
        let v: Vec<_> = ll2.into();
        assert_eq!(v, vec![7, 11, 13]);
    }

    #[test]
    fn test_push() {
        let mut ll1 = LinkedList::new();
        assert_eq!(ll1, [].into());
        assert!(ll1.push(1, 0).is_err());
        ll1.push(0, 3).unwrap();
        assert_eq!(ll1, [3].into());
        ll1.push(0, 2).unwrap();
        assert_eq!(ll1, [2, 3].into());
        ll1.push(2, 7).unwrap();
        assert_eq!(ll1, [2, 3, 7].into());
        ll1.push(2, 5).unwrap();
        assert_eq!(ll1, [2, 3, 5, 7].into());
    }

    #[test]
    fn test_push_front() {
        let mut ll1 = LinkedList::new();
        assert_eq!(ll1, [].into());
        ll1.push_front(5);
        assert_eq!(ll1, [5].into());
        ll1.push_front(3);
        assert_eq!(ll1, [3, 5].into());
        ll1.push_front(2);
        assert_eq!(ll1, [2, 3, 5].into());
    }

    #[test]
    fn test_push_back() {
        let mut ll1 = LinkedList::new();
        assert_eq!(ll1, [].into());
        ll1.push_back(2);
        assert_eq!(ll1, [2].into());
        ll1.push_back(3);
        assert_eq!(ll1, [2, 3].into());
        ll1.push_back(5);
        assert_eq!(ll1, [2, 3, 5].into());
    }

    #[test]
    fn test_pop() {
        let mut ll1 = LinkedList::from([2, 3, 5, 7]);
        assert_eq!(ll1, [2, 3, 5, 7].into());
        assert!(ll1.pop(4).is_err());
        ll1.pop(3).unwrap();
        assert_eq!(ll1, [2, 3, 5].into());
        ll1.pop(1).unwrap();
        assert_eq!(ll1, [2, 5].into());
        ll1.pop(0).unwrap();
        assert_eq!(ll1, [5].into());
        ll1.pop(0).unwrap();
        assert_eq!(ll1, [].into());
        assert!(ll1.pop(0).is_err());
    }

    #[test]
    fn test_pop_front() {
        let mut ll1 = LinkedList::from([2, 3, 5]);
        assert_eq!(ll1, [2, 3, 5].into());
        ll1.pop_front().unwrap();
        assert_eq!(ll1, [3, 5].into());
        ll1.pop_front().unwrap();
        assert_eq!(ll1, [5].into());
        ll1.pop_front().unwrap();
        assert_eq!(ll1, [].into());
        assert!(ll1.pop_front().is_err());
    }

    #[test]
    fn test_pop_back() {
        let mut ll1 = LinkedList::from([2, 3, 5]);
        assert_eq!(ll1, [2, 3, 5].into());
        ll1.pop_back().unwrap();
        assert_eq!(ll1, [2, 3].into());
        ll1.pop_back().unwrap();
        assert_eq!(ll1, [2].into());
        ll1.pop_back().unwrap();
        assert_eq!(ll1, [].into());
        assert!(ll1.pop_back().is_err());
    }

    #[test]
    fn test_clear() {
        let mut ll1 = LinkedList::<i32>::new();
        ll1.clear();
        assert_eq!(ll1.len(), 0);
        assert_eq!(ll1, [].into());

        let mut ll2 = LinkedList::from([2, 3, 5, 7]);
        assert_eq!(ll2.len(), 4);
        assert_eq!(ll2, [2, 3, 5, 7].into());
        ll2.clear();
        assert_eq!(ll2.len(), 0);
        assert_eq!(ll2, [].into());
    }

    #[test]
    fn test_reverse() {
        let mut ll1 = LinkedList::<i32>::new();
        assert_eq!(ll1.len(), 0);
        assert_eq!(ll1, [].into());
        ll1.reverse();
        assert_eq!(ll1.len(), 0);
        assert_eq!(ll1, [].into());

        let mut ll2 = LinkedList::from([1]);
        assert_eq!(ll2.len(), 1);
        assert_eq!(ll2, [1].into());
        ll2.reverse();
        assert_eq!(ll2.len(), 1);
        assert_eq!(ll2, [1].into());

        let mut ll3 = LinkedList::from([2, 3]);
        assert_eq!(ll3.len(), 2);
        assert_eq!(ll3, [2, 3].into());
        ll3.reverse();
        assert_eq!(ll3.len(), 2);
        assert_eq!(ll3, [3, 2].into());

        let mut ll4 = LinkedList::from([4, 5, 6]);
        assert_eq!(ll4.len(), 3);
        assert_eq!(ll4, [4, 5, 6].into());
        ll4.reverse();
        assert_eq!(ll4.len(), 3);
        assert_eq!(ll4, [6, 5, 4].into());

        let mut ll2 = LinkedList::from([2, 3, 5, 7, 11, 13, 17, 19, 23]);
        assert_eq!(ll2, [2, 3, 5, 7, 11, 13, 17, 19, 23].into());
        ll2.reverse();
        assert_eq!(ll2, [23, 19, 17, 13, 11, 7, 5, 3, 2].into());
    }

    #[test]
    fn test_default() {
        let ll1 = LinkedList::<i32>::new();
        let ll2 = LinkedList::default();
        let ll3 = Default::default();
        assert_eq!(ll1, ll2);
        assert_eq!(ll1, ll3);
        assert_eq!(ll2.len(), 0);
        assert_eq!(ll3.len(), 0);
    }

    #[test]
    fn test_from_slice() {
        let ll1: LinkedList<i32> = LinkedList::from(vec![].as_slice());
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from(vec![2, 3, 5, 7].as_slice());
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = vec![].as_slice().into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = vec![2, 3, 5, 7].as_slice().into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_from_mut_slice() {
        let ll1: LinkedList<i32> = LinkedList::from(vec![].as_mut_slice());
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from(vec![2, 3, 5, 7].as_mut_slice());
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = vec![].as_mut_slice().into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = vec![2, 3, 5, 7].as_mut_slice().into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_from_sized_slice() {
        let ll1: LinkedList<i32> = LinkedList::from(&[]);
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from(&[2, 3, 5, 7]);
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = (&[]).into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = (&[2, 3, 5, 7]).into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_from_mut_sized_slice() {
        let ll1: LinkedList<i32> = LinkedList::from(&mut []);
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from(&mut [2, 3, 5, 7]);
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = (&mut []).into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = (&mut [2, 3, 5, 7]).into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_from_array() {
        let ll1: LinkedList<i32> = LinkedList::from([]);
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from([2, 3, 5, 7]);
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = [].into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = [2, 3, 5, 7].into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_from_vec() {
        let ll1: LinkedList<i32> = LinkedList::from(vec![]);
        let ll2 = LinkedList::new();
        assert_eq!(ll1, ll2);

        let ll3 = LinkedList::from(vec![2, 3, 5, 7]);
        let mut ll4 = LinkedList::new();
        ll4.push_back(2);
        ll4.push_back(3);
        ll4.push_back(5);
        ll4.push_back(7);
        assert_eq!(ll3, ll4);

        let ll5: LinkedList<i32> = vec![].into();
        let ll6 = LinkedList::new();
        assert_eq!(ll5, ll6);

        let ll7: LinkedList<i32> = vec![2, 3, 5, 7].into();
        let mut ll8 = LinkedList::new();
        ll8.push_back(2);
        ll8.push_back(3);
        ll8.push_back(5);
        ll8.push_back(7);
        assert_eq!(ll7, ll8);
    }

    #[test]
    fn test_try_into_array() {
        let arr: [_; 0] = LinkedList::<i32>::from([]).try_into().unwrap();
        assert_eq!(arr, []);

        let arr: [_; 4] = LinkedList::from([2, 3, 5, 7]).try_into().unwrap();
        assert_eq!(arr, [2, 3, 5, 7]);

        let arr_result: Result<[_; 1]> = LinkedList::<i32>::from([]).try_into();
        assert!(arr_result.is_err());
    }

    #[test]
    fn test_into_vec() {
        let v: Vec<_> = LinkedList::<i32>::from([]).into();
        assert_eq!(v, vec![]);

        let v: Vec<_> = LinkedList::from([2, 3, 5, 7]).into();
        assert_eq!(v, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_from_iter() {
        let ll1: LinkedList<i32> = [].into_iter().collect();
        assert_eq!(ll1, LinkedList::from([]));

        let ll2: LinkedList<_> = [2, 3, 5, 7].into_iter().collect();
        assert_eq!(ll2, LinkedList::from([2, 3, 5, 7]));
    }

    #[test]
    fn test_into_iter() {
        let ll1 = LinkedList::<i32>::new();
        let v: Vec<_> = ll1.into_iter().collect();
        assert_eq!(v, vec![]);

        let ll2 = LinkedList::from([2, 3, 5, 7]);
        let v: Vec<_> = ll2.into_iter().collect();
        assert_eq!(v, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(LinkedList::<i32>::from([]), LinkedList::from([]));
        assert_eq!(
            LinkedList::from([2, 3, 5, 7]),
            LinkedList::from([2, 3, 5, 7])
        );
        assert_ne!(LinkedList::from([]), LinkedList::from([2, 3, 5, 7]));
        assert_ne!(
            LinkedList::from([1, 2, 3, 4]),
            LinkedList::from([2, 3, 5, 7])
        );
        assert_ne!(
            LinkedList::from([1, 4, 9, 16, 25]),
            LinkedList::from([2, 3, 5, 7])
        );
        assert_ne!(
            LinkedList::from([-2, -3, -5, -7]),
            LinkedList::from([2, 3, 5, 7])
        );
    }
}
