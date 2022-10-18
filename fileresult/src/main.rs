//! Simulating files one step at a time.
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

/// Represents a "file",
/// which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    /// Creates a new, empty `File`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = File::new("name.txt");
    /// ```
    pub fn new(name: &str) -> Self {
        Self { 
            name: String::from(name), 
            data: vec![],
            state: FileState::Closed,
        }
    }

    /// Returns the file's length in bytes
    pub fn len(&self) -> usize { self.data.len() }

    /// Returns the file's name.
    pub fn name(&self) -> String { self.name.clone() }

    fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(&mut self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            Err(String::from("File must be open for reading"))
        } else {
            let mut tmp = self.data.clone();
            let read_length = self.data.len();
            save_to.reserve(read_length);
            save_to.append(&mut tmp);
            Ok(read_length)
        }
    }
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

fn open(mut f: File) -> Result<File, String> { 
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> { 
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let data = vec![114, 117, 115, 116, 33];
    let mut f = File::new_with_data("file.txt", &data); 
    let mut buffer: Vec<u8> = vec![];

    if f.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f = open(f).unwrap();
    println!("{}", f);
    let size = f.read(&mut buffer).unwrap();
    f = close(f).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f);
    println!("{} is {} bytes long", f.name, size);
    println!("{}", text);
    println!("{}", f)
}
