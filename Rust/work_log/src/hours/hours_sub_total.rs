use std::fmt;

pub enum Errors {
    LoginCapacity,
    LogoutCapacity,
    ParseError(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Errors::LoginCapacity  => write!(f, "LOGIN HAS TO MANY CHAR'S"),
            Errors::LogoutCapacity => write!(f, "LOGOUT HAS TO MANY CHAR'S"),
            Errors::ParseError(s)     => write!(f, "ERROR PARSING STRING INTO I8, NUMBER: {}"
                                                , s),
        }
    }
}

pub fn total_hours(login: impl Iterator<Item = char>,
                   logout: impl Iterator<Item = char>) -> Result<String, Errors> {
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
        Err(_why) => return Err(Errors::ParseError(in_hours_str)),
        Ok(res) => res,
    };
    let in_min    = match in_min_str.parse::<i8>() {
        Err(_why) => return Err(Errors::ParseError(in_min_str)),
        Ok(res) => res,
    };
    let in_sec    = match in_sec_str.parse::<i8>() {
        Err(_why) => return Err(Errors::ParseError(in_sec_str)),
        Ok(res) => res,
    };

    let out_hours = match out_hours_str.parse::<i8>() {
        Err(_why) => return Err(Errors::ParseError(out_hours_str)),
        Ok(res) => res,
    };
    let out_min   = match out_min_str.parse::<i8>() {
        Err(_why) => return Err(Errors::ParseError(out_min_str)),
        Ok(res) => res,
    };
    let out_sec   = match out_sec_str.parse::<i8>() {
        Err(_why) => return Err(Errors::ParseError(out_sec_str)),
        Ok(res) => res,
    };
    // maths
    let res_hours = out_hours - in_hours;
    let res_min   = out_min - in_min;
    let res_sec   = out_sec - in_sec;
    // back to string

    return Ok( res_hours.to_string() + &":".to_string()
               + &res_min.to_string() + &":".to_string()
               + &res_sec.to_string());
}

pub fn find_times(itter: impl Iterator<Item = char>) -> String {
    let file : Vec<char> =  itter.collect();
    let mut login = String::new();
    let mut logout = String::new();

    let mut count : i8 = 0;
    let mut index = 0;
    // capture login logout times
    for c in &file {
        if *c == ':' {
            if count == 0 {
                for idx in index+2..index+10 {
                    login.push(file[idx]);
                }
            }
            if count == 3 {
                for idx in index+2..index+10 {
                    logout.push(file[idx]);
                }
                break;
            }
            count = if count == 5 {0} else {count+1};
        }
        index += 1;
    }

    let result = match total_hours(login.chars(),logout.chars()) {
        Err(why) => panic!("ERROR IN TOTAL_HOURS: {}", why),
        Ok(res) => res,
    };

    return result;
}
