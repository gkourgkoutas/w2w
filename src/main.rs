use std::fs::*;
use std::io::*;
use std::path::Path;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    let content = Wikipedia::wiki_search(args.search);
    Wikipedia::wiki_random();
    Wordlist::create_output_file(&args.output);
    Wordlist::create_wordlist(&content, args.output);
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

struct Wordlist {}
impl Wordlist {
    // Create Wikipedia wordlist
    // String, String -> File
    // Gets the wikipedia content, creates a wordlist
    fn create_wordlist(content: &str, outfile: String) {
        // Create new wordlist file in current directory
        let wordlist = content.split_whitespace();
        let reg = regex::Regex::new(r"[^0-9a-zA-Z]+").unwrap();

        for word in wordlist {
            if reg.is_match(word) && (word.len() >= 5) {
                let word = reg.replace_all(word, "") + "\n";
                Wordlist::edit_output_file(&word, &outfile);
            } else if word.len() <= 4 {
                let word = word.replace(word, "");
                Wordlist::edit_output_file(&word, &outfile);
            }
        }
    }

    // Helper function: Create new wordlist file
    fn create_output_file(path: &str) {
        let path = Path::new(&path);
        let display = path.display();

        match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
    }

    // Helper function: Edit wordlist file
    fn edit_output_file(word: &str, outfile: &str) {
        let display_path = Path::new(&outfile).display();
        let mut file = OpenOptions::new()
            .append(true)
            .open(outfile)
            .expect("No such file or directory");

        match file.write_all(word.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display_path, why),
            Ok(file) => file,
        };
    }
}
