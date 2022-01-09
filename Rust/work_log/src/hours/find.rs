

#[derive(Debug)]
pub struct Times {
    pub login_list : Vec<String>,
    pub logout_list : Vec<String>,
}

impl PartialEq for Times {
    fn eq(&self, other: &Self) -> bool {
        self.login_list == other.login_list &&
        self.logout_list == other.logout_list
    }
}

pub fn find_times(itter: impl Iterator<Item = char>) -> Times {
    let file : Vec<char> =  itter.collect();
    let mut login = String::new();
    let mut logout = String::new();
    let mut login_vec : Vec<String> = Vec::new();
    let mut logout_vec : Vec<String> = Vec::new();

    let mut skip_l : i8 = 0;
    // capture login logout times
    for (index,c) in file.iter().enumerate() {
        if *c == 'L' && skip_l == 0 {
            for idx in file.iter().take(index+15).skip(index+7) {
                login.push(*idx);
            }
            for idx in file.iter().take(index+32).skip(index+24) {
                logout.push(*idx);
            }
            login_vec.push(login.clone());
            logout_vec.push(logout.clone());
            login.clear();
            logout.clear();
        }
        if *c == 'L' {
            if let 1 = skip_l {skip_l = 0} else {skip_l = 1}
        }
    }

    Times {
            login_list: login_vec,
            logout_list: logout_vec,
    }
}
