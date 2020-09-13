use std::fs::*;
use std::io::*;
use std::path::Path;
use structopt::StructOpt;

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
    // Output
    /// Outputfile
    #[structopt(short, long)]
    output: String,
}

// Search Wikipedia
// String -> String
// Gets a keyword and returns the wikipedia page
fn wiki_search(keyword: String) -> String {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let page = wiki.page_from_title(keyword);
    let content = page.get_content().unwrap();
    return content;
}

// Create Wikipedia wordlist
// String, String -> File
// Gets the wikipedia content, creates a wordlist
fn create_wordlist(content: String, outfile: String) {
    // Create new wordlist file in current directory
    let wordlist = content.split_whitespace();

      for word in wordlist {
        if word.len() >= 5 && 
        (!word.contains('\'') || !word.contains(',') || !word.contains('\"') || !word.contains('.')){
            let word = word.to_string() + "\n"; 
            edit_output_file(word.to_string(), outfile.to_string());
        }
    }
}

// Helper function: Create new wordlist file
fn create_output_file(path: String){
    let path = Path::new(&path);
    let display = path.display();

    match File::create(&path){
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
}

// Helper function: Edit wordlist file
fn edit_output_file(word: String, outfile: String) {
    let display_path = Path::new(&outfile).display();
    
    let mut file = OpenOptions::new()
         .append(true)
         .open(outfile.to_string())
         .expect("No such file or directory");
    
    match file.write_all(word.as_bytes()){
        Err(why) => panic!("couldn't write to {}: {}", display_path, why),
        Ok(file) => file,
    };
}

fn main() {
    let args = Cli::from_args();
    let wiki_content = wiki_search(args.search);
    create_output_file(args.output.clone());
    create_wordlist(wiki_content, args.output.clone());
}