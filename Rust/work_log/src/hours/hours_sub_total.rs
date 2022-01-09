
#[path = "err.rs"]
pub mod err;

pub fn total_hours(login: impl Iterator<Item = char>,
                   logout: impl Iterator<Item = char>) -> Result<String, err::Errors> {
    let in_vec  : Vec<char> = login.collect();
    let out_vec : Vec<char> = logout.collect();
    // parse into hours minutes seconds
    let in_hours_str  : String = in_vec[0].to_string() + &in_vec[1].to_string();
    let in_min_str    : String = in_vec[3].to_string() + &in_vec[4].to_string();
    let in_sec_str    : String = in_vec[6].to_string() + &in_vec[7].to_string();

    let out_hours_str : String = out_vec[0].to_string() + &out_vec[1].to_string();
    let out_min_str   : String = out_vec[3].to_string() + &out_vec[4].to_string();
    let out_sec_str   : String = out_vec[6].to_string() + &out_vec[7].to_string();
    // get integers
    let in_hours  = match in_hours_str.parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrHour(in_hours_str)),
        Ok(res) => res,
    };
    let in_min    = match in_min_str.parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrMin(in_min_str)),
        Ok(res) => res,
    };
    let in_sec    = match in_sec_str.parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrSec(in_sec_str)),
        Ok(res) => res,
    };

    let out_hours = match out_hours_str.parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrHour(out_hours_str)),
        Ok(res) => res,
    };
    let out_min   = match out_min_str.parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrMin(out_min_str)),
        Ok(res) => res,
    };
    let out_sec   = match out_sec_str.parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrSec(out_sec_str)),
        Ok(res) => res,
    };
    // maths
    let mut res_hours = out_hours - in_hours;
    let mut res_min   = out_min - in_min;
    if res_min < 0 {
        res_hours -= 1;
        res_min += 60;
    }
    let mut res_sec   = out_sec - in_sec;
    if res_sec < 0 {
        res_min -= 1;
        res_sec += 60;
    }
    // back to string

    Ok( res_hours.to_string() + &":".to_string()
               + &res_min.to_string() + &":".to_string()
               + &res_sec.to_string())
}

