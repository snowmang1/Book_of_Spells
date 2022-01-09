
#[path = "hours/add_times.rs"]
pub mod add_times;
#[path = "hours/mod.rs"]
pub mod hours;

fn fix_fmt(s: impl Iterator<Item = char>) -> String{

    let mut itr : Vec<char> = s.collect();

    if itr[1] == ':' {
        itr.insert(0, '0');
    }
    if itr[3] == ':' || itr[4] == ':' {
        itr.insert(3, '0');
    }
    if itr[itr.len()-2] == ':' {
        itr.insert(itr.len()-1, '0');
    }
    let ret : String = itr.iter().collect();

    ret
}

pub fn show_total(times: hours::find::Times) -> String {
    // protocal for total hours
    let mut total : String = "00:00:00".to_string();
    for idx in 0..times.login_list.len() {
        let mut hours_sub = match hours::hours_sub_total::total_hours(times.login_list[idx].chars(),
                                                            times.logout_list[idx].chars()) {
            Err(why) => panic!("SUB_TOTAL: {}", why),
            Ok(res) => res,
        };
        hours_sub = fix_fmt(hours_sub.chars());
        total = fix_fmt(total.chars());
        total = match add_times::add_times(total.as_str(), hours_sub.as_str()) {
            Err(why) => panic!("ADD_TIMES: {}", why),
            Ok(res) => res,
        };
    }
    println!("Total Hours this month: {}", total);

    total
}

#[cfg(test)]
mod fix_fmt_tests {
    use super::*;

    #[test]
    fn test1() {
        let test_str = "0:0:0".to_string();
        let ret_test = fix_fmt(test_str.chars());
        let ret = "00:00:00".to_string();
        assert_eq!(ret_test, ret);
    }

    #[test]
    fn test2() {
        let test_str = "00:0:0".to_string();
        let ret_test = fix_fmt(test_str.chars());
        let ret = "00:00:00".to_string();
        assert_eq!(ret_test, ret);
    }

    #[test]
    fn test3() {
        let test_str = "0:00:0".to_string();
        let ret_test = fix_fmt(test_str.chars());
        let ret = "00:00:00".to_string();
        assert_eq!(ret_test, ret);
    }

    #[test]
    fn test4() {
        let test_str = "0:0:00".to_string();
        let ret_test = fix_fmt(test_str.chars());
        let ret = "00:00:00".to_string();
        assert_eq!(ret_test, ret);
    }

    #[test]
    fn test5() {
        let test_str = "00:00:00".to_string();
        let ret_test = fix_fmt(test_str.chars());
        let ret = "00:00:00".to_string();
        assert_eq!(ret_test, ret);
    }
}
