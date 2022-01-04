
pub mod cl_parse;
pub mod logio;
pub mod hours;

fn main()
{
    let mut flags : i8 = 0x00000000;
    let mut date = String::new();
    let mut time = String::new();
    let mut cmd_flag = String::new();
    cl_parse::get_cmd(std::env::args(), &mut cmd_flag, &mut date, &mut time);
    // gather the arguments passed via cmd line in a single string
    cl_parse::flag_check(&mut flags, cmd_flag);
    // check for flags for needed action
    logio::log_prot(&flags, &date, &time);
    //open file stream gets closed when it goes out of scope.
    hours::hours();
}

// TODO version 0.2.0
// -calculate hours per logio
// -calculate total hours

