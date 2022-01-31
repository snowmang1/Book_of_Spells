
pub mod cl_parse;
pub mod logio;
pub mod hours;

#[path = "hours/hours_sub_total.rs"]
pub mod hours_sub_total;

#[path = "hours/find.rs"]
pub mod find;

#[path = "hours/add_times.rs"]
pub mod add_times;

pub mod total;

#[cfg(test)]
mod flag_tests {
    use super::*; #[test]
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
    fn test_find1() {
        let test_str = "date\nLogin: 01:00:00\nLogout: 02:00:00\n".to_string();
        let result = find::find_times(test_str.chars());
        let login_res : Vec<String> = ["01:00:00".to_string()].to_vec();
        let logout_res : Vec<String> = ["02:00:00".to_string()].to_vec();
        let test_res = find::Times {login_list:login_res, logout_list:logout_res};

        assert_eq!(result, test_res);
    }

    #[test]
    fn test_find2() {
        let test_str = "date\nLogin: 01:00:00\nLogout: 02:00:00\n
                        date\nLogin: 05:00:00\nLogout: 07:00:00\n".to_string();
        let result = find::find_times(test_str.chars());
        let login_res : Vec<String> = ["01:00:00".to_string(),"05:00:00".to_string()].to_vec();
        let logout_res : Vec<String> = ["02:00:00".to_string(), "07:00:00".to_string()].to_vec();
        let test_res = find::Times {login_list:login_res, logout_list:logout_res};

        assert_eq!(result, test_res);
    }

    #[test]
    fn test_sub_total1() {
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
    fn test_sub_total2() {
        let login  = "10:23:50".to_string();
        let logout = "21:30:55".to_string();
        let result : String = "11:7:5".to_string();
        let test_res : String = match hours_sub_total::total_hours(login.chars(), logout.chars()) {
            Err(why) => panic!("TOTAL_HOURS ERROR: {}", why),
            Ok(res) => res,
        };
        assert_eq!(test_res, result);
    }

    #[test]
    fn test_sub_total3() {
        let login  = "00:30:59".to_string();
        let logout = "00:31:01".to_string();
        let result : String = "0:0:2".to_string();
        let test_res : String = match hours_sub_total::total_hours(login.chars(), logout.chars()) {
            Err(why) => panic!("TOTAL_HOURS ERROR: {}", why),
            Ok(res) => res,
        };
        assert_eq!(test_res, result);
    }

}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    #[should_panic(expected = "ERROR PARSING STRING INTO I8, SEC: cc")]
    fn test_add_error() {
        let login = "01:01:cc";
        let logout = "02:02:04";
        let res : String = "3:4:3".to_string();
        let test_res = match add_times::add_times(login, logout) {
            Err(why) => panic!("{}", why),
            Ok(res) => res,
        };
        assert_eq!(res,test_res);
    }
    #[test]
    fn test_add() {
        let login = "01:01:01";
        let logout = "02:02:02";
        let res : String = "3:3:3".to_string();
        let test_res = match add_times::add_times(login, logout) {
            Err(why) => panic!("{}", why),
            Ok(res) => res,
        };
        assert_eq!(res,test_res);
    }

    #[test]
    fn test_add2() {
        let login = "01:01:59";
        let logout = "02:02:04";
        let res : String = "3:4:3".to_string();
        let test_res = match add_times::add_times(login, logout) {
            Err(why) => panic!("{}", why),
            Ok(res) => res,
        };
        assert_eq!(res,test_res);
    }

    #[test]
    fn test_total() {
        let times = total::hours::find::Times {
            login_list: ["01:00:00".to_string()].to_vec(),
            logout_list: ["02:00:00".to_string()].to_vec(),
        };
        let test_res = total::show_total(times);
        let res = "1:0:0";
        assert_eq!(test_res, res);
    }

    #[test]
    fn test_carry() {
        let times = total::hours::find::Times {
            login_list: ["09:57:39".to_string()].to_vec(),
            logout_list: ["10:57:20".to_string()].to_vec(),
        };
        let res = "0:59:19";
        let test_res = match total::show_total(times) {
            Err(why) => panic!("{}", why),
            Ok(res) => res,
        };
        assert_eq!(res,test_res);
    }

}
