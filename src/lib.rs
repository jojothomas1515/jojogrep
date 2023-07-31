use std::{
    env,
    error::Error,
    fs,
    io::{self, ErrorKind},
};

pub struct Grep {
    pub query: String,
    pub file_path: String,
    pub results: Option<Vec<String>>,
    case_insensitivity: bool,
}

impl Grep {
    /// return an instance of Grep from env::args()
    ///
    /// # Parameters
    /// - args: reference to a vector string
    /// # Examples
    /// ```rust
    /// use std::env;
    /// use jojogrep::Grep;
    /// //let args: Vec<String> = env::args().collect();
    /// let args = vec!("dummy".to_string(), "get".to_string(),"file.txt".to_string());
    /// let grep: Grep = Grep::new(&args).unwrap();
    /// ```
    pub fn new(args: &[String]) -> Result<Self, ErrorKind> {
        if args.len() < 3 {
            return Err(ErrorKind::InvalidInput);
        }
        return Ok(Grep {
            query: args[1].clone(),
            file_path: args[2].clone(),
            results: None,
            case_insensitivity: match env::var("CASE_INSENSITIVE") {
                Ok(res) => {
                    let mut ans: bool = false;
                    if res == "true" {
                        ans = true;
                    }
                    ans
                }
                Err(e) => false,
            },
        });
    }

    pub fn read_file(&self) -> Result<String, io::Error> {
        return fs::read_to_string(&self.file_path);
    }

    pub fn print_content(&self) {
        println!("{}", self.read_file().unwrap());
    }

    pub fn search(&mut self) -> Vec<String> {
        let content = self.read_file().unwrap();
        let mut res: Vec<String> = Vec::new();
        for i in content.lines() {
            if i.contains(&self.query) {
                res.push(i.to_owned());
            }
        }
        self.results = Some(res);
        return self.results.clone().unwrap();
    }
    pub fn i_search(&mut self) -> Vec<String> {
        let content = self.read_file().unwrap();
        let mut res: Vec<String> = Vec::new();
        for i in content.lines() {
            if i.to_lowercase().contains(&self.query.to_lowercase()) {
                res.push(i.to_owned());
            }
        }
        self.results = Some(res);
        return self.results.clone().unwrap();
    }

    pub fn print_matches(&mut self) {
        if self.case_insensitivity {
            self.i_search();
            for i in self.results.as_ref().unwrap() {
                println!("{}", i);
            }
        } else {
            self.search();
            for i in self.results.as_ref().unwrap() {
                println!("{}", i);
            }
        }
    }
}
