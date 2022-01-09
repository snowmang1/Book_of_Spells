
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_prot(flag: &i8, date: &str, tim: &str)
{
    let buffer: String = match flag {
        1 | 8   => date.to_owned() + &"\n".to_string() 
            + &"Login: ".to_string() + tim + &"\n".to_string(),
        2       => "Logout: ".to_string() + tim + &"\n".to_string(),
        _       => panic!("ERROR IN LOGIO"),
    };

    // create file path
    let out_path = Path::new("log.txt");
    let display = out_path.display();

    // Write to file
    let mut out_file = match OpenOptions::new()
            .append(true).open(&out_path) {
        Err(why) => panic!("ERROR: {}, FILE TO BE OPENED: {}", why, display),
        Ok(out_file) => out_file,
    };

    if flag == &8 {
        match out_file.set_len(0) {
            Err(_) => panic!("ERROR IN TRUNCATION\n"),
            Ok(_) => println!("success on truncation!!!"),
        };
    }

    if let Err(_why) = out_file.write_all(buffer.as_bytes()) { panic!("ERROR IN FILE WRITE") }
}
