fn binary_search(A: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = A.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        if target == A[mid] {
            return mid as i32
        }
        else if target < A[mid] {
            high = mid - 1;
        }
        else {
            low = mid + 1;
        }
    }
    return -1
}

fn main() {
    // Assume vector already sorted
    let A = vec![1, 2, 7, 10, 11, 22];
    // Return index position: 2
    println!("{}", binary_search(A, 7));
}
