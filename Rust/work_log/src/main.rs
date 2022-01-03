
pub mod cl_parse;
pub mod logio;

fn main()
{
    let mut flags : i8 = 0x00000000;
    let mut date : String = "".to_string();
    let mut time : String = "".to_string();
    let mut cmd_flag : String = "".to_string();
    cl_parse::get_cmd(std::env::args(), &mut cmd_flag, &mut date, &mut time);
    // gather the arguments passed via cmd line in a single string
    cl_parse::flag_check(&mut flags, cmd_flag);
    // check for flags for needed action
    logio::log_prot(&flags, &date, &time);
    //open file stream gets closed when it goes out of scope.
}

// TODO
// -archive file instead of truncate
// -calculate hours per logio
// -calculate total hours

