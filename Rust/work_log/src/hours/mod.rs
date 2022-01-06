
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

pub mod hours_sub_total;

pub fn hours_out(time: &String) {

    let buffer = "Sub_Total: ".to_string() + &time + &"\n".to_string();
    
    let path = Path::new("log.txt");
    let display = path.display();

    let mut out_file = match OpenOptions::new()
            .append(true).open(&path) {
        Err(why) => panic!("ERROR: {}, FILE TO BE OPENED: {}", why, display),
        Ok(h_file) => h_file,
    };

    match out_file.write_all(buffer.as_bytes()) {
        Err(_why) => panic!("ERROR IN FILE WRITE"),
        Ok(_) => (),
    }
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

    hours_out( &hours_sub_total::find_times(h_string.chars()) );
}
