
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Read;

pub fn file_to_string() -> String{

    let mut h_string = String::new();
    // caclulate hours of session of logout
    let h_path = Path::new("log.txt");
    let display = h_path.display();

    let mut h_file = match OpenOptions::new()
            .read(true).open(&h_path) {
        Err(why) => panic!("ERROR: {}, FILE TO BE OPENED: {}", why, display),
        Ok(h_file) => h_file,
    };

    // h_string = log.txt as a String struct
    match h_file.read_to_string(&mut h_string) {
        Err(why) => panic!("ERROR: {}, WITH READ_TO_STRING", why),
        Ok(h_string) => h_string,
    };

    h_string

}
