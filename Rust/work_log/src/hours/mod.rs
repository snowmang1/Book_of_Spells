
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub mod hours_sub_total;
pub mod find;

#[path = "../fts.rs"]
pub mod fts;

fn hours_out(time: &str) {

    let buffer = "Sub_Total: ".to_string() + time + &"\n".to_string();

    let path = Path::new("log.txt");
    let display = path.display();

    let mut out_file = match OpenOptions::new()
            .append(true).open(&path) {
        Err(why) => panic!("ERROR: {}, FILE TO BE OPENED: {}", why, display),
        Ok(h_file) => h_file,
    };

    if let Err(_why) = out_file.write_all(buffer.as_bytes()) {panic!("ERROR IN FILE WRITE");}
}

pub fn hours() -> find::Times{
    let h_string = fts::file_to_string();
    let times = find::find_times(h_string.chars());
    // protocal for sub_totals
    let hours_sub = match hours_sub_total::total_hours(times.login_list[times.login_list.len()-1].chars(),
                                                 times.logout_list[times.logout_list.len()-1].chars()) {
        Err(why) => panic!("{}", why),
        Ok(res) => res,
    };

    hours_out(hours_sub.as_str());

    times
}
