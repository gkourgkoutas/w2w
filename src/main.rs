use std::fs::*;
use std::io::*;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    let content = Wikipedia::wiki_search(args.search);
    Wikipedia::wiki_random();
    let mut wordlist = match Wordlist::new(&args.output) {
        Ok(wordlist) => wordlist,
        Err(e) => panic!("couldn't open {}: {}", &args.output, e),
    };

    match wordlist.write_contents(&content) {
        Ok(()) => {}
        Err(e) => panic!("couldn't write to {}: {}", &args.output, e),
    };
}

/// CLI Tool to generate wordlists based on wikipedia articles
#[derive(StructOpt, Debug)]
#[structopt(name = "w2w")]
struct Cli {
    // Search
    /// Search wikipedia by keyword
    #[structopt(short, long)]
    search: String,
    // Language
    /// Set the article language
    #[structopt(short = "l", long, default_value = "en")]
    lang: String,
    // Random
    /// Get random article
    #[structopt(short = "r", long)]
    random: Option<bool>,
    // Output
    /// Outputfile
    #[structopt(short, long)]
    output: String,
}

struct Wikipedia {}
impl Wikipedia {
    // Search Wikipedia
    // String -> String
    // Gets a keyword and returns the wikipedia page
    fn wiki_search(search: String) -> String {
        let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        let page = wiki.page_from_title(search);
        let content = page.get_content().unwrap();
        return content;
    }

    // Get random wiki page
    fn wiki_random() {
        let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        let random_title = wiki.random().unwrap().take().unwrap().to_string();
        println!("{}", random_title);
    }
}

struct Wordlist {
    writer: BufWriter<File>,
}

impl Wordlist {
    // Create Wikipedia wordlist
    // String, String -> File
    // Gets the wikipedia content, creates a wordlist
    fn new(path_str: &str) -> std::io::Result<Wordlist> {
        // Create new wordlist file in current directory

        let path = PathBuf::from(&path_str);
        let file = OpenOptions::new().create(true).write(true).open(path)?;
        let writer = BufWriter::new(file);

        Ok(Wordlist { writer })
    }

    fn write_contents(&mut self, content: &str) -> std::io::Result<()> {
        let words = content.split_whitespace();
        let reg = regex::Regex::new(r"[^0-9a-zA-Z]+").unwrap();

        for word in words {
            if reg.is_match(word) && (word.len() >= 5) {
                let word = reg.replace_all(word, "") + "\n";
                self.write(&word.as_bytes())?;
            } else if word.len() <= 4 {
                let word = word.replace(word, "");
                self.write(&word.as_bytes())?;
            }
        }

        Ok(())
    }
}

impl Write for Wordlist {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
