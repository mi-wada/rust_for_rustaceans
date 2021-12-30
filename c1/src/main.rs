fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer() {
        let x = 42;
        let y = 43;
        let p1 = &x;
        let mut p2 = &x;
        p2 = &y;

        assert_eq!(*p1, x);
        assert_eq!(*p2, y);
    }

    #[test]
    fn test_box() {
        fn def_x() {
            let x = Box::new(10);
        }
        let x = Box::new(10);
    }
}
