fn insertion_sort(A: &mut Vec<i32>) -> &mut Vec<i32> {
    for i in 1..A.len() {
        let mut j = i;
        let tmp = A[i];
        while j > 0 && A[j] < A[j - 1] {
            A[j] = A[j - 1];
            j -= 1;
        }
        A[j] = tmp;
    }
    A
}

fn main() {
    let A = &mut vec![1, 5, 2, 8];
    println!("{:?}", insertion_sort(A));
}
