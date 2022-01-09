
#[path = "../hours/err.rs"]
pub mod err;

pub fn add_times(login: &str, logout: &str) -> Result<String,err::Errors> {
    // gather into numeric litterals
    let in_hours = match login[0..2].parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrHour(login[0..2].to_string())),
        Ok(res) => res,
    };
    let in_min   = match login[3..5].parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrMin(login[3..5].to_string())),
        Ok(res) => res,
    };
    let in_sec   = match login[6..8].parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrSec(login[6..8].to_string())),
        Ok(res) => res,
    };

    let out_hours = match logout[0..2].parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrHour(logout[0..2].to_string())),
        Ok(res) => res,
    };
    let out_min   = match logout[3..5].parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrMin(logout[3..5].to_string())),
        Ok(res) => res,
    };
    let out_sec   = match logout[6..8].parse::<i8>() {
        Err(_why) => return Err(err::Errors::ParseErrSec(logout[6..8].to_string())),
        Ok(res) => res,
    };
    // logic
    // - sec
    let mut carry_sec : i8 = 0;
    let mut res_sec = out_sec + in_sec;
    if res_sec > 60 {
        carry_sec = 1;
        res_sec -= 60;
    }
    // - min
    let mut carry_min : i8 = 0;
    let mut res_min = out_min + in_min + carry_sec;
    if res_min > 60 {
        carry_min = 1;
        res_min -= 60;
    }
    // - hours
    let res_hours = out_hours + in_hours + carry_min;

    Ok( res_hours.to_string() + &":".to_string() + &res_min.to_string() + &":".to_string()
        + &res_sec.to_string())
}
