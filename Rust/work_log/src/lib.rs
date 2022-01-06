
pub mod cl_parse;
pub mod logio;
pub mod hours;

#[path = "hours/hours_sub_total.rs"]
pub mod hours_sub_total ;

#[cfg(test)]
mod flag_tests {
    use super::*;
    #[test]
    fn in_flag() {
        let mut flag: i8 = 0;
        let in_str = String::from("i");
        cl_parse::flag_check(&mut flag, in_str);
        assert_eq!(&flag, &1);
    }
    #[test]
    fn out_flag() {
        let mut flag: i8 = 0;
        let in_str = String::from("o");
        cl_parse::flag_check(&mut flag, in_str);
        assert_eq!(&flag, &2);
    }
    #[test]
    fn total_flag() {
        let mut flag: i8 = 0;
        let in_str = String::from("t");
        cl_parse::flag_check(&mut flag, in_str);
        assert_eq!(&flag, &4);
    }
    #[test]
    fn new_flag() {
        let mut flag: i8 = 0;
        let in_str = String::from("n");
        cl_parse::flag_check(&mut flag, in_str);
        assert_eq!(&flag, &8);
    }
    #[test]
    #[should_panic(expected = "NON-EXISTANT FLAG ERROR")]
    fn panic_flag() {
        let mut flag: i8 = 0;
        let in_str = String::from("x");
        cl_parse::flag_check(&mut flag, in_str);
        assert_eq!(&flag, &1);
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

#[cfg(test)]
mod hours_tests {
    use super::*;

    #[test]
    fn test_find() {
        let test_str = "date\nLogin: 01:00:00\nLogout: 02:00:00\n".to_string();
        let result = hours_sub_total::find_times(test_str.chars());
        let test_res = "1:0:0".to_string();

        assert_eq!(result, test_res);
    }

    #[test]
    fn test_total() {
        let login  = "01:23:50".to_string();
        let logout = "02:23:50".to_string();
        let result : String = "1:0:0".to_string();
        let test_res : String = match hours_sub_total::total_hours(login.chars(), logout.chars()) {
            Err(why) => panic!("TOTAL_HOURS ERROR: {}", why),
            Ok(res) => res,
        };
        assert_eq!(test_res, result);
    }

    #[test]
    fn test_total2() {
        let login  = "10:23:50".to_string();
        let logout = "21:30:55".to_string();
        let result : String = "11:7:5".to_string();
        let test_res : String = match hours_sub_total::total_hours(login.chars(), logout.chars()) {
            Err(why) => panic!("TOTAL_HOURS ERROR: {}", why),
            Ok(res) => res,
        };
        assert_eq!(test_res, result);
    }
}
