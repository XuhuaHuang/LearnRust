use core::slice::Iter;

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}

fn main() {
    let vec: Vec<u8> = vec![1, 2, 3];

    let vec_iter: Iter<u8> = vec.iter();

    for val in vec_iter {
        println!("Got: {}", val);
    }

    let mapped_vec: Vec<_> = vec.iter().map(|x| x + 1).collect();

    assert_eq!(mapped_vec, vec![2, 3, 4]);
}
