pub mod User {
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    pub struct User {
        name: String,
        email: String,
        active: bool,
    }

    impl User {
        pub fn new(name: String, email: String, active: bool) -> User {
            User {
                name,
                email,
                active,
            }
        }
    }

    pub fn add_user_in_file(user: User) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("user.txt")
            .unwrap();

        let user_data = format!("{} {} {}", user.name, user.email, user.active);

        if let Err(e) = writeln!(file, "{}", user_data) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

pub mod read_users {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn read_users_from_file() -> Vec<String> {
        let file = File::open("user.txt").unwrap();
        let reader = BufReader::new(file);

        let mut user_list: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            user_list.push(line);
        }

        return user_list;
    }
}
