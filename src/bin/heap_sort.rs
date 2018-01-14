

fn main() {
    let mut arr = vec![4, 1, 2, 8, 0, 3];

    println!("{:?}", arr);

    heap_sort(&mut arr);

    println!("{:?}", arr);
}

fn heap_sort<T: Ord>(xs: &mut [T]) {
    heapify(xs);
    let n = xs.len();
    for last_i in (0..n).rev() {
        xs.swap(0, last_i);
        shift_down(&mut xs[..last_i], 0);
    }
}

fn shift_down<T: Ord>(xs: &mut [T], i: usize) {
    let mut check_child = | child_i: usize| {
        if child_i < xs.len() {
            if xs[child_i] > xs[i] {
                xs.swap(i, child_i);
                shift_down(xs, child_i);
            }
        }
    };
    check_child(left_child_index(i));
    check_child(right_child_index(i));
}

fn left_child_index(i: usize) -> usize {
    2 * i + 1
}

fn right_child_index(i: usize) -> usize {
    2 * i + 2
}

fn parent_index(i: usize) -> Option<usize> {
    if i == 0 { None } else { Some((i - 1) / 2) }
}

fn up_heap<T: Ord>(xs: &mut [T], i: usize) {
    if let Some(p) = parent_index(i) {
        if xs[p] < xs[i] {
            xs.swap(p, i);
            up_heap(xs, p);
        }
    }
}

fn heapify<T: Ord>(xs: &mut [T]) {
    for i in 1..xs.len() {
        up_heap(xs, i);
    }
}

pub struct PriorityQueue<T: Ord> {
    queue: Vec<T>
}

impl <T: Ord> Iterator for PriorityQueue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl <T: Ord> PriorityQueue<T> {

    pub fn new() -> PriorityQueue<T> {
        PriorityQueue { queue: Default::default() }
    }

    pub fn push(&mut self, v: T) {
        self.queue.push(v);
        let last_index = self.queue.len() - 1;
        up_heap(&mut self.queue, last_index)
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.queue.len();
        if len == 0 { None }
        else {
            self.queue.swap(0, len-1);
            let max = self.queue.pop();
            shift_down(&mut self.queue, 0);
            max
        }
    }
}


#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    quickcheck! {

        fn priority_queue_returns_elements_in_reverse_order(xs: Vec<i32>) -> bool {

            let mut queue = PriorityQueue::new();
            for x in xs.clone() {
                queue.push(x)
            }

            let mut xs_sorted = xs.clone();
            xs_sorted.sort();
            xs_sorted.reverse();

            let result: Vec<i32> = queue.collect();

            result == xs_sorted
        }
    }

    quickcheck! {

        fn heapify_stable(org: Vec<i32>) -> bool {
            let mut xs = org.clone();
            heapify(&mut xs);

            let mut ys = xs.clone();
            heapify(&mut ys);

            xs == ys
        }

        fn shift_down_cannot_change_heapified(org: Vec<i32>) -> bool {
            let mut xs = org.clone();
            heapify(&mut xs);

            let ys = xs.clone();
            for i in 0..xs.len() {
                shift_down(&mut xs, i);
            }

            ys == xs
        }

        fn up_heap_cannot_change_heapified(org: Vec<i32>) -> bool {
            let mut xs = org.clone();
            heapify(&mut xs);

            let ys = xs.clone();
            for i in 0..xs.len() {
                up_heap(&mut xs, i);
            }

            ys == xs
        }

        fn parent_index_zero(i: usize) -> bool {
            if i == 0 { parent_index(i) == None }
            else { parent_index(i) != None }
        }

        fn partent_left_child_loop(i: usize) -> bool {
            let child_i = left_child_index(i);
            Some(i) == parent_index(child_i)
        }

        fn partent_right_child_loop(i: usize) -> bool {
            let child_i = right_child_index(i);
            Some(i) == parent_index(child_i)
        }

        fn keeps_length(xs: Vec<i32>) -> bool {
            let arr = &mut xs.clone();
            let len1 = arr.len();
            heap_sort(arr);
            let len2 = arr.len();
            len1 == len2
        }

        fn never_decrease(xs: Vec<i32>) -> bool {
            let arr = &mut xs.clone();
            heap_sort(arr);
            let mut i = 1;
            while i < arr.len() {
                if arr[i-1] > arr[i] {
                    return false;
                }
                i += 1;
            }
            true
        }

        fn equals_to_sorted(xs: Vec<i32>) -> bool  {
            let sorted = &mut xs.clone();
            let arr = &mut xs.clone();
            heap_sort(arr);

            sorted.sort();

            sorted == arr
        }
    }

    use super::*;
}

