
pub mod cl_parse;
pub mod logio;

pub fn flag_eq(flag: &i8, check: &i8) -> bool {
    if flag == check{
        return true;
    }
    return false;
}

#[cfg(test)]
mod flag_tests {
    use super::*;
    #[test]
    fn in_flag() {
        let mut flag: i8 = 0;
        let in_str = String::from("i");
        cl_parse::flag_check(&mut flag, in_str);
        assert!(flag_eq(&flag, &1));
    }
    #[test]
    fn out_flag() {
        let mut flag: i8 = 0;
        let in_str = String::from("o");
        cl_parse::flag_check(&mut flag, in_str);
        assert!(flag_eq(&flag, &2));
    }
    #[test]
    fn total_flag() {
        let mut flag: i8 = 0;
        let in_str = String::from("t");
        cl_parse::flag_check(&mut flag, in_str);
        assert!(flag_eq(&flag, &4));
    }
    #[test]
    fn new_flag() {
        let mut flag: i8 = 0;
        let in_str = String::from("n");
        cl_parse::flag_check(&mut flag, in_str);
        assert!(flag_eq(&flag, &8));
    }
    #[test]
    #[should_panic(expected = "NON-EXISTANT FLAG ERROR")]
    fn panic_flag() {
        let mut flag: i8 = 0;
        let in_str = String::from("x");
        cl_parse::flag_check(&mut flag, in_str);
        assert!(flag_eq(&flag, &1));
    }
}

#[cfg(test)]
mod cmd_parse_tests {
    use super::*;

    #[test]
    fn itter_safe1() {
        let apple = ["a", "b", "c", "d"];
        let apple_iter = apple.iter().map(|s| s.to_string());
        let mut flag : String = "".to_string();
        let mut date : String = "".to_string();
        let mut time : String = "".to_string();
        cl_parse::get_cmd(apple_iter, &mut flag, &mut date, &mut time);
        if apple[1] == flag && apple[2] == date  && apple[3] == time {
            assert!(true);
        }else {
            assert!(false);
        }
    }

    #[test]
    fn itter_safe2() {
        let apple = ["z", "t", "b", "1"];
        let apple_iter = apple.iter().map(|s| s.to_string());
        let mut flag : String = "".to_string();
        let mut date : String = "".to_string();
        let mut time : String = "".to_string();
        cl_parse::get_cmd(apple_iter, &mut flag, &mut date, &mut time);
        if apple[1] == flag && apple[2] == date  && apple[3] == time {
            assert!(true);
        }else {
            assert!(false);
        }
    }
}

