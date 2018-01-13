
fn main() {

    let mut arr = vec![4, 1, 2, 8, 0, 3];

    println!("{:?}", arr);

    heap_sort(&mut arr);

    println!("{:?}", arr);
}

fn heap_sort<T: Ord>(xs: &mut[T]) {
    heapify(xs);
    let n = xs.len();
    for last_i in (0..n).rev() {
        xs.swap(0, last_i);
        shift_down(&mut xs[..last_i], 0);
    }
}

fn shift_down<T: Ord>(xs: &mut [T], i: usize) {

    //TODO: can be replaced with lambda?
    fn check_child<T: Ord>(xs: &mut [T], i: usize, child_i: usize) {
        if child_i < xs.len() {
            if &xs[child_i] > &xs[i] {
                xs.swap(i, child_i);
                shift_down(xs, child_i);
            }
        }
    };

    check_child(xs, i, 2 * i + 1);
    check_child(xs, i, 2 * i + 2);
}

fn heapify<T: Ord>(xs: &mut [T]) {

    fn parent_index(i: usize) -> Option<usize> {
        let r = (i as isize - 1) / 2;
        if r < 0 { None } else { Some(r as usize) }
    }

    fn up_heap<T: Ord>(xs: &mut [T], i: usize) {
        if let Some(p) = parent_index(i) {
            if &xs[p] < &xs[i] {
                xs.swap(p, i);
                up_heap(xs, p);
            }
        }
    }

    for i in 1..xs.len() {
        up_heap(xs, i);
    }

}

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    quickcheck! {
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

