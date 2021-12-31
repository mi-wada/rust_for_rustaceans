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
        fn vec_x() -> Vec<Box<i32>> {
            let x = Box::new(10);
            let mut v = vec![];
            v.push(x);
            v
        }

        fn vec_x_n() -> Vec<i32> {
            let x = 10;
            let mut v = vec![];
            v.push(x);
            v
        }

        let box_v = vec_x();

        assert_eq!(*box_v[0], 10);
    }

    fn replace_with_84(s: &mut Box<i32>) {
        // let was = std::mem::take(s);
        // *s = was;
        let mut r = Box::new(84);
        std::mem::swap(s, &mut r);
        assert_ne!(*r, 84);
        assert_eq!(**s, 84);
    }

    #[test]
    fn lifetime() {
        let mut x = Box::new(42);
        let mut y = Box::new(0);
        let mut z = &x;
        for i in 0..100 {
            println!("{}", z);
            x = Box::new(i); // ここでzは無効な変数となる

            z = &y; // ここでzは有効な変数となる
        }
        println!("{}", z);

        struct StrSplit<'s, 'd> {
            document: &'s str,
            delimiter: &'d str,
        }
        impl<'s, 'd> Iterator for StrSplit<'s, 'd> {
            type Item = &'s str;
            fn next(&mut self) -> Option<Self::Item> {
                todo!()
            }
        }
        fn str_before(s: &str, c: char) -> Option<&str> {
            StrSplit {
                document: s,
                delimiter: &c.to_string(),
            }
            .next()
        }
    }
    #[test]
    fn invariant() {
        fn pr<'a>(v: &mut Vec<&'a str>) {
            println!("hoge");
        }

        let x: &'static str = "hoge";

        pr(&mut vec![x, "hoge"]);
        fn mama<'a>() {
            let x: &'static str = "hoe";
            let y: &'a str = "hoe";

            fn take_static(str: &mut Vec<&'static str>) {
                println!("{:?}", str);
            }

            fn take_a(str: &mut Vec<&str>) {
                println!("{:?}", str);
            }

            take_a(&mut vec![y]);
        }
    }

    #[test]
    fn mut_str() {
        struct MutStr<'a, 'b> {
            s: &'a mut &'b str,
        }
        let mut s = "hello";

        // シングルライフタイムにしてしまうと，sは次の行で無効になってしまう．
        // 小さい方のライフタイムに合わせるため
        *MutStr { s: &mut s }.s = "world";
        assert_eq!(s, "world");
    }
}
