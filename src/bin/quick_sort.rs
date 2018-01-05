


#[cfg(test)]
#[macro_use]
extern crate quickcheck;

fn main() {

    let mut arr = vec![94, -42, -17, 98, -71, -95, -42, -96, -6, 20, -23, -95, 2, 65, -40, -61, -90, -45];

    quick_sort_gen(&mut arr);

    println!("sorted: {:?}", arr);
}

fn quick_sort_gen<T: Ord>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mut a = 0;
        let last = arr.len() - 1;
        let mut b = last;
        while a < b {
            while a < b && arr[a] <= arr[last] {
                a += 1;
            }
            while arr[b] > arr[last] {
                b -= 1;
            }
            if a < b {
                arr.swap(a, b);
                a += 1;
            }
        }
        if a > 0 {
            quick_sort_gen(&mut arr[a..]);
            quick_sort_gen(&mut arr[..a]);
        }
    }
}

#[cfg(test)]
mod tests {
    quickcheck! {
        fn keeps_length(xs: Vec<i32>) -> bool {
            let arr = &mut xs.clone();
            let len1 = arr.len();
            quick_sort_gen(arr);
            let len2 = arr.len();
            len1 == len2
        }

        fn never_decrease(xs: Vec<i32>) -> bool {
            let arr = &mut xs.clone();
            quick_sort_gen(arr);
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
            quick_sort_gen(arr);

            sorted.sort();

            sorted == arr
        }
    }

    use super::*;
}

