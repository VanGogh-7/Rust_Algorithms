use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<i32> = stones.into_iter().collect();

    while heap.len() > 1 {
        let y = heap.pop().unwrap();
        let x = heap.pop().unwrap();

        if y != x {
            heap.push(y - x);
        }
    }
    heap.pop().unwrap_or(0)
}

fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    if nums1.is_empty() || nums2.is_empty() || k == 0 {
        return Vec::new();
    }

    let heap_size = nums1.len().min(k);
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = (0..heap_size)
        .map(|i| Reverse((nums1[i] + nums2[i], i, 0)))
        .collect();

    let mut result = Vec::with_capacity(k.min(nums1.len().saturating_mul(nums2.len())));

    while result.len() < k {
        let Some(Reverse((_sum, i, j))) = heap.pop() else {
            break;
        };
        result.push(vec![nums1[i], nums2[j]]);
        if j + 1 < nums2.len() {
            heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
        }
    }
    result
}

fn is_possible(target: Vec<i32>) -> bool {
    let mut total: i64 = target.iter().map(|&x| x as i64).sum();
    let mut heap: BinaryHeap<i64> = target.into_iter().map(i64::from).collect();

    while let Some(max) = heap.pop() {
        let rest = total - max;
        if max == 1 || rest == 1 {
            return true;
        }
        if rest == 0 || max <= rest {
            return false;
        }
        let previous = max % rest;

        if previous == 0 {
            return false;
        }

        total = rest + previous;
        heap.push(previous);
    }
    true
}

fn main() {
    let stones = vec![2, 7, 4, 1, 8, 1];
    let result = last_stone_weight(stones);
    println!("The last weight is {}", result);

    let nums1 = vec![1, 7, 11];
    let nums2 = vec![2, 4, 6];
    let k = 3;
    let result = k_smallest_pairs(nums1, nums2, k);
    println!("The k smallest pairs are {:?}", result);

    let target = vec![9, 3, 5];
    let result = is_possible(target);
    println!("The target is {}", result);
}
