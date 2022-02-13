pub mod q_alg;

#[cfg(test)]
mod tests{
    use super::*;
    use q_alg::quick_sort;

    #[test]
    pub fn test1() {
        let mut a : Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let p : Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let size : i32 = (a.len() as i32)-(1 as i32);
        quick_sort(&mut a, 0, size);
        assert_eq!(a, p);
    }

    #[test]
    pub fn test2() {
        let mut a : Vec<i32> = vec![3, 10, 7, 5, 4, 6, 1, 8, 9, 2];
        let p : Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let size : i32 = (a.len() as i32)-(1 as i32);
        quick_sort(&mut a, 0, size);
        assert_eq!(a, p);
    }

    #[test]
    pub fn test3() {
        let mut a : Vec<i32> = vec![100, 10, 21, 50, 1, 3, -4, 30, 19];
        let p : Vec<i32> = vec![-4, 1, 3, 10, 19, 21, 30, 50, 100];
        let size : i32 = (a.len() as i32)-(1 as i32);
        quick_sort(&mut a, 0, size);
        assert_eq!(a, p);
    }
}
