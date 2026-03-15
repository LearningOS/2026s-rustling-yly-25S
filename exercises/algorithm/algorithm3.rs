/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE
fn merge_sort<T>(array: &mut [T]) where T: Ord + Copy{
    let len = array.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let (fst, snd) = array.split_at_mut(mid);
    merge_sort(fst);
    merge_sort(snd);
    merge(fst, snd);
}

fn merge<T>(array_l: &mut [T], array_r: &mut [T]) where T: Ord + Copy {
    for i in 0..array_l.len() {
        for j in 0..array_r.len() {
            if array_l[i] > array_r[j] {
                let temp = array_l[i];
                array_l[i] = array_r[j];
                array_r[j] = temp;
            }
        }
    }
}

fn sort<T>(array: &mut [T]) where T: Ord + Copy{
	merge_sort(array);
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