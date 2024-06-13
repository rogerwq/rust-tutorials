// use std::fs::read_to_string;

// use for-loops
// fn main() {
//     // let input = read_to_string("src/input.txt").unwrap();
//     let input = include_str!("input.txt");
//     let input = input.replace("\r\n", "\n");
//     let groups = input.split("\n\n")
//         .collect::<Vec<_>>();
//     let mut max = 0;
//     for group in groups {
//         let mut sum = 0;
//         for line in group.lines() {
//             let value = line.parse::<u64>().unwrap();
//             sum += value;
//         }

//         if sum > max {
//             max = sum;
//         }
//     }
//     dbg!(max);
// }

// use functional programming methds
// fn main() {
//     let input = include_str!("input.txt").lines().collect::<Vec<_>>();
//     let max = input.split(|line| line.is_empty())
//         .map(|group| group.iter()
//             .filter_map(|v| v.parse::<u64>().ok())
//             .sum::<u64>()
//         )
//         .max();
//     dbg!(max);
// }

// use iterator
// struct GroupSumIter<I> {
//     inner: I 
// }

// impl<I> Iterator for GroupSumIter<I> 
// where
//     I: Iterator<Item = Option<u64>>
// {
//     type Item = u64;
//     fn next(&mut self) -> Option<Self::Item> {
//         let mut sum = 0;
//         loop {
//             match self.inner.next() {
//                 Some(Some(v)) => sum += v,
//                 Some(None) => return Some(sum), 
//                 None => return None
//             } 
//         }
        
//     }
// }

// fn main() {
//     let input = include_str!("input.txt").lines()
//         .map(|v| v.parse::<u64>().ok());
//     let max = GroupSumIter { inner: input }.max();
//     dbg!(max);
// }

// use itertools::Itertools;

// fn main() {
//     // let nums = [0, 1, 2, 3, 5, 5, 0, 1, 5, 2, 5]; // => [6, 1, 2];
//     // let results = nums.into_iter().batching(|it| {
//     //     let mut sum = 0;
//     //     loop {
//     //         match it.next() {
//     //             Some(v) => if v < 5 {
//     //                 sum += v
//     //             } else { break Some(sum) }
//     //             None => return None 
//     //         }
//     //     }
//     // }).collect_vec();
//     // dbg!(results);

//     let input = include_str!("input.txt").lines()
//         .map(|v| v.parse::<u64>().ok());
//     let max = input.batching(|it| {
//         let mut sum = None;
//         loop {
//             match it.next() {
//                 Some(Some(v)) => { sum = Some(sum.unwrap_or(0) + v); },
//                 Some(None) | None => break sum,
//             }
//         }
//     }).max();
//     dbg!(max);
// }

// use itertools::Itertools;

// fn main() {
//     let input = include_str!("input.txt").lines()
//         .map(|v| v.parse::<u64>().ok());
//     let max = input.coalesce(|x, y| match (x, y) {
//         (None, None) => Ok(None),
//         (None, Some(v)) => Ok(Some(v)),
//         (Some(v1), Some(v2)) => Ok(Some(v1 + v2)),
//         (Some(v), None) => Err((Some(v), None))
//     })
//     .max();
//     dbg!(max);
// }

// Find max 3

// use std::cmp::Reverse;

// use itertools::Itertools;

// fn main() {
//     let res = include_str!("input.txt").lines()
//         .map(|v| v.parse::<u64>().ok())
//         .coalesce(|x, y| match (x, y) {
//             (None, None) => Ok(None),
//             (None, Some(v)) => Ok(Some(v)),
//             (Some(v1), Some(v2)) => Ok(Some(v1 + v2)),
//             (Some(v), None) => Err((Some(v), None))
//         })
//         .flatten()
//         // .sorted_by_key(|&v| u64::MAX - v)
//         // .sorted_by_key(|&v| Reverse(v))
//         // .take(3)
//         .map(Reverse)
//         .k_smallest(3)
//         .map(|x| x.0)
//         .collect_vec();
//     dbg!(res);
// }

// BinaryHeap

// use std::{cmp::Reverse, collections::BinaryHeap};

// use itertools::Itertools;

// fn main() {
//     let mut sums = include_str!("input.txt").lines()
//         .map(|v| v.parse::<u64>().ok())
//         .coalesce(|x, y| match (x, y) {
//             (None, None) => Ok(None),
//             (None, Some(v)) => Ok(Some(v)),
//             (Some(v1), Some(v2)) => Ok(Some(v1 + v2)),
//             (Some(v), None) => Err((Some(v), None))
//         })
//         .flatten()
//         .map(Reverse);

//     // insert 3 elements into heap
//     // insert a new element, pop an old element
//     let mut heap = BinaryHeap::new();
//     for init in (&mut sums).take(3) {
//         heap.push(init);
//     }

//     for rest in sums {
//         heap.push(rest);
//         heap.pop();
//     }

//     dbg!(heap);
// }

use itertools::Itertools;

fn main() {
    let res = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u64>())
        .k_largest(3)
        .collect_vec();
    dbg!(res);
}