

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

fn main() {
    let mut xs = vec![1,3,4,2,0,5];
    insertion_sort(&mut xs);
    println!("insertion sort: {:?}", xs);
}

fn insertion_sort<T: Ord>(xs: &mut [T]) {

    for i in 0..xs.len() {
        for j in 0..i {
            if xs[i] < xs[j] {
                xs.swap(i, j);
            }
        }
    }

}


#[cfg(test)]
mod tests {
    quickcheck! {
        fn keeps_length(xs: Vec<i32>) -> bool {
            let arr = &mut xs.clone();
            let len1 = arr.len();
            insertion_sort(arr);
            let len2 = arr.len();
            len1 == len2
        }

        fn never_decrease(xs: Vec<i32>) -> bool {
            let arr = &mut xs.clone();
            insertion_sort(arr);
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
            insertion_sort(arr);

            sorted.sort();

            sorted == arr
        }
    }

    use super::*;
}

