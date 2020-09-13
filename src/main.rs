use std::fs::*;
use std::io::*;
use structopt::StructOpt;

/// Commandline Struct
/// Defines parameters to use
#[derive(StructOpt, Debug)]
#[structopt(name = "CLI")]
struct Cli {
    // Search
    /// Set the keyword to search wikipedia for
    #[structopt(short, long)]
    search: String,
    // Language
    /// Set the languages to use for wikipedia pages
    #[structopt(short = "l", long, default_value = "de")]
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
fn create_wordlist(content: String, outfile: String) {
    // Create new wordlist file in current directory
    create_output_file(outfile.to_string()).expect("Couldn't create file");
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

fn main() {
    let args = Cli::from_args();
    let wiki_content = wiki_search(args.search);
    create_wordlist(wiki_content, args.output);
}