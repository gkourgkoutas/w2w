use std::fs::*;
use std::io::*;
use std::{fmt,error,result};
use std::path::PathBuf;
use structopt::StructOpt;

// custom Result type
type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()>{
    let args = Cli::from_args();
    
    let content = if args.random {
        Wikipedia::search(Wikipedia::random_search()?)?
    } else {
        Wikipedia::search(args.search.unwrap())?
    };

    Wordlist::new(&args.output)?
        .write_contents(&content)?;

    println!("[+] wordlist generated");
    Ok(())
}

/// CLI Tool to generate wordlists based on wikipedia articles
#[derive(StructOpt, Debug)]
#[structopt(name = "w2w")]
struct Cli {
    // Search
    /// Search wikipedia by keyword
    #[structopt(short, long, required_unless("random"))]
    search: Option<String>,
    // Language
    /// Set the article language
    #[structopt(short = "l", long, default_value = "en")]
    lang: String,
    // Random
    /// Get random article
    #[structopt(short = "r", long, conflicts_with("search"))]
    random: bool,
    // Output
    /// Outputfile
    #[structopt(short, long)]
    output: String,
}

// Custom Error because Wikipedia doesn't implement std::error::Error
#[derive(Debug)]
struct WikipediaError(wikipedia::Error);
impl fmt::Display for WikipediaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl error::Error for WikipediaError {}

impl std::convert::From<wikipedia::Error> for WikipediaError {
    fn from(e: wikipedia::Error) -> WikipediaError {
        WikipediaError(e)
    }
}

type WikipediaResult<T> = result::Result<T, WikipediaError>;

struct Wikipedia {}
impl Wikipedia {
    // Search Wikipedia
    fn search(search: String) -> WikipediaResult<String> {
        let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        let page = wiki.page_from_title(search);
        Ok(page.get_content()?)
    }

    // Get random wiki page
    fn random_search() -> WikipediaResult<String> {
        let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        Ok(wiki.random()?.unwrap_or("".to_string()))
    }
}

struct Wordlist {
    writer: BufWriter<File>,
}

impl Wordlist {
    // Create Wikipedia wordlist
    fn new(path_str: &str) -> Result<Wordlist> {
        let path = PathBuf::from(&path_str);
        let file = OpenOptions::new().create(true).write(true).open(path)?;
        let writer = BufWriter::new(file);

        Ok(Wordlist { writer })
    }
    // Sort wordlist + write content
    fn write_contents(&mut self, content: &str) -> Result<()> {
        let words = content.split_whitespace();
        let reg = regex::Regex::new(r"[^0-9a-zA-Z]+")?;

        let mut sorted_list: Vec<String> = Vec::new();
        for word in words {
            if reg.is_match(word) && (word.len() >= 5) {
                let cont = reg.replace_all(word, "") + "\n";
                sorted_list.push(cont.to_string());
            }
        }
        sorted_list.sort();
        sorted_list.dedup_by(|b, a| a.eq_ignore_ascii_case(b));
        for word in sorted_list {
            self.write(&word.as_bytes())?;
        }
        Ok(())
    }
}

impl Write for Wordlist {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}