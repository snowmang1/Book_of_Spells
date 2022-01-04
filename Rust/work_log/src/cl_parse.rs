
pub fn get_cmd(iter: impl Iterator<Item = String>,
               fl: &mut String, dt: &mut String,
               tim: &mut String)
{
    let args: Vec<String> = iter.skip(1).collect();
    *fl  = args[0].to_owned();
    *dt  = args[1].to_owned();
    *tim = args[2].to_owned();
}

pub fn flag_check(flags: &mut i8, cl: String)
{
    let _login = String::from("i");
    let _logout = String::from("o");
    let _total = String::from("t");
    let _new_month = String::from("n");
    // checks
    if _login == cl {
        *flags =        1; // insert login time in the log
    }
    else if _logout == cl {
        *flags =        2; // insert logout time to log
    }
    else if _total == cl {
        *flags =        4; // print total hours this month
    }
    else if _new_month == cl {
        *flags =        8; // truncates log file to 0 and starts a new month
    }
    else {
        panic!("NON-EXISTANT FLAG ERROR");
    }
}

