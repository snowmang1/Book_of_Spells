
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Read;
use std::fmt;

#[derive(Debug)]
pub enum Errors {
    Logincapacity,
    Logoutcapacity,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Errors::Logincapacity  => write!(f, "LOGIN HAS TO MANY CHAR'S"), 
            Errors::Logoutcapacity => write!(f, "LOGOUT HAS TO MANY CHAR'S"),
        }
    }
}

pub fn total_hours(login: impl Iterator<Item = char>,
                   logout: impl Iterator<Item = char>) -> Result<String, Errors> {
    let in_vec  : Vec<char> = login.collect();
    let out_vec : Vec<char> = logout.collect();
    // parse into hours minutes seconds
    let in_hours_str  : String = in_vec[0].to_string() + &in_vec[1].to_string();
    let in_min_str    : String = in_vec[2].to_string() + &in_vec[3].to_string();
    let in_sec_str    : String = in_vec[5].to_string() + &in_vec[6].to_string();

    let out_hours_str : String = out_vec[0].to_string() + &out_vec[1].to_string();
    let out_min_str   : String = out_vec[2].to_string() + &out_vec[3].to_string();
    let out_sec_str   : String = out_vec[5].to_string() + &out_vec[6].to_string();
    // get integers
    let in_hours  = in_hours_str.parse::<i8>().unwrap();
    let in_min    = in_min_str.parse::<i8>().unwrap();
    let in_sec    = in_sec_str.parse::<i8>().unwrap();

    let out_hours = out_hours_str.parse::<i8>().unwrap();
    let out_min   = out_min_str.parse::<i8>().unwrap();
    let out_sec   = out_sec_str.parse::<i8>().unwrap();
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

    let result = login + &" ".to_string() + &logout;

    return result;
}

pub fn hours() {
    let mut h_string = String::new();
    // caclulate hours of session of logout
    let h_path = Path::new("log.txt");
    let display = h_path.display();

    let mut h_file = match OpenOptions::new()
            .read(true).open(&h_path) {
        Err(why) => panic!("ERROR: {}, FILE TO BE OPENED: {}", why, display),
        Ok(h_file) => h_file,
    };

    match h_file.read_to_string(&mut h_string) {
        Err(why) => panic!("ERROR: {}, WITH READ_TO_STRING", why),
        Ok(h_string) => h_string,
    };
    // h_string == log.txt
    let _test = find_times(h_string.chars());

}
