/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

// fn sort<T: std::cmp::PartialOrd>(array: &mut [T]) {
//     let len = array.len();
//     for i in 0..len {
//         let mut min = &array[i];
//         let mut ind = i;
//         for j in i..len {
//             if array[j] < *min {
//                 min = &array[j];
//                 ind = j;
//             }
//         }
//         array.swap(ind, i);
//     }
// }
//本人qsort实现
// fn sort<T: std::cmp::PartialOrd + Clone>(array: &mut [T]) {
//     let len = array.len();
//     qsort(array, 0, len - 1)
// }

// fn qsort<T: std::cmp::PartialOrd + Clone>(array: &mut [T], lp: usize, rp: usize) {
//     if (lp >= rp) {
//         return;
//     }
//     let flag = array[lp].clone();
//     let mut clp: usize = lp;
//     let mut crp: usize = rp;
//     while clp < crp {
//         while array[crp] >= flag && clp < crp {
//             crp -= 1;
//         }
//         if clp < crp {
//             array.swap(clp, crp);
//         } else {
//             qsort(array, lp, clp);
//             qsort(array, crp + 1, rp);
//             return;
//         }
//         while array[clp] <= flag && clp < crp {
//             clp += 1;
//         }
//         if clp < crp {
//             array.swap(clp, crp);
//         } else {
//             qsort(array, lp, clp);
//             qsort(array, crp + 1, rp);
//             return;
//         }
//     }
// }

//AI实现
fn sort<T: std::cmp::PartialOrd>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    qsort(array, 0, array.len() - 1);
}

fn qsort<T: std::cmp::PartialOrd>(array: &mut [T], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let pivot_idx = partition(array, left, right);

    if pivot_idx > 0 {
        qsort(array, left, pivot_idx - 1);
    }
    qsort(array, pivot_idx + 1, right);
}

fn partition<T: std::cmp::PartialOrd>(array: &mut [T], left: usize, right: usize) -> usize {
    let pivot_idx = right;
    let mut i = left;

    for j in left..right {
        if array[j] <= array[pivot_idx] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, pivot_idx);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
