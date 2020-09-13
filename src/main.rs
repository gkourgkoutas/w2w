use std::fs::*;
use std::io::*;
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
    // TODO: If page does not exist -> error or suggestion
    let page = wiki.page_from_title(keyword);
    let content = page.get_content().unwrap();
    return content;
}

// Create Wikipedia wordlist
// String, String -> File
// Gets the wikipedia content, creates a wordlist
fn create_wordlist(content: String, outfile: String){
    // Create new wordlist file in current directory
    //create_output_file(outfile.to_string());
    for word in content.split_whitespace() {
        if word.len() >= 5 && !word.contains('\''){
            let word = word.to_string() + "\n";
            edit_output_file(word.to_string(), outfile.to_string());
        }
    }
}

// Helper function: Create new wordlist file
fn create_output_file(outfile: String) -> std::io::Result<()> {
    File::create(outfile).expect_err("No such file or directory");
    Ok(())
}

// Helper function: Edit wordlist file
fn edit_output_file(word: String, outfile: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(outfile.to_string())
        .expect("No such file or directory");
    file.write_all(word.as_bytes()).expect("write failed");
}

// Get random wiki page
fn wiki_random() -> String {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let random_page_title = wiki.random().unwrap();
    let content = wiki_search(random_page_title.unwrap());
    return content;
}

fn main() {
    let args = Cli::from_args();
    let wiki_content = wiki_search(args.search);
    create_wordlist(wiki_content, args.output);
}