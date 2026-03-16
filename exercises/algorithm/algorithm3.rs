/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn merge_sort<T>(array: &mut [T]) where T: Ord + Copy{
    let len = array.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2 ;
    let (fst, snd) = array.split_at_mut(mid);
    merge_sort(fst);
    merge_sort(snd);
    merge(fst, snd);
}

fn merge<T>(array_l: &mut [T], array_r: &mut [T]) where T: Ord + Copy {
    if array_l.len() == 0 || array_r.len() == 0 {
        return;
    }
    let mut l_cur= 0;
    let mut r_cur= 0;
    let mut temp: Vec<T> = Vec::new();
    while l_cur < array_l.len() && r_cur < array_r.len() {
        if array_l[l_cur] <= array_r[r_cur] {
            temp.push(array_l[l_cur]);
            l_cur += 1;
        } else {
            temp.push(array_r[r_cur]);
            r_cur += 1;
        }
    }
    if l_cur != array_l.len() {
        temp.extend_from_slice(&array_l[l_cur..]);
    } else {
        temp.extend_from_slice(&array_r[r_cur..]);
    }
    let (new_l, new_r) = temp.split_at_mut(array_l.len());
    array_l.copy_from_slice(new_l);
    array_r.copy_from_slice(new_r);
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