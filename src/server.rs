#![allow(dead_code)]
pub mod server {
    extern crate ftp;
    use ftp::FtpStream;
    use std::io::Cursor;
    use std::str;

    pub struct Server {
        ip: String,
        port: u32,
        username: String,
        password: String,
    }

    impl Server {
        pub fn new(ip: String, port: u32, username: String, password: String) -> Server {
            Server {
                ip,
                port,
                username,
                password,
            }
        }

        pub fn connect(&self) -> FtpStream {
            let mut ftp_stream = FtpStream::connect(format!("{}:{}", self.ip, self.port)).unwrap();
            ftp_stream.login(&self.username, &self.password).unwrap();
            ftp_stream
        }

        pub fn get_file(stream: FtpStream, path: &str, file_name: &str) -> String {
            let mut ftp_stream = stream;

            ftp_stream.cwd(path).unwrap();

            let remote_file = ftp_stream.simple_retr(file_name).unwrap();

            let binding = remote_file.into_inner();

            let file_content = str::from_utf8(&binding).unwrap();

            file_content.to_string()
        }

        pub fn upload_file(stream: FtpStream, file_name: &str, file_content: &str, path: &str) {
            let mut ftp_stream = stream;

            ftp_stream.cwd(path).unwrap();

            let mut file = Cursor::new(file_content);
            ftp_stream
                .put(file_name, &mut file)
                .expect("Failed to upload file");

            ftp_stream.quit().unwrap();

            println!("File uploaded successfully");
        }
    }
}
