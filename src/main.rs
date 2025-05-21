use std::fs::*;
use std::io::*;
use std::{fmt,error,result};
use std::path::PathBuf;
use structopt::StructOpt;

// custom Result type
type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()>{
    let args = Cli::from_args();
    
    let content: String = if args.random {
        let random_title = Wikipedia::random_search()?;
        println!("[*] Selected random article: {}", random_title);
        let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        wiki.page_from_title(random_title)
            .get_content()
            .map_err(WikipediaError::from)?
    } else {
        let search_term = args.search.clone().unwrap(); // clone to use it in messages
        let search_results: Vec<String> = if args.lang != "en" {
            Wikipedia::language_search(search_term.clone(), args.lang.clone())?
        } else {
            Wikipedia::search(search_term.clone())?
        };

        if search_results.is_empty() {
            println!("No results found for '{}'.", search_term);
            return Ok(()); // Exit gracefully
        }

        let selected_title: String;

        if search_results.len() == 1 {
            selected_title = search_results.into_iter().next().unwrap();
            println!("[*] Using first result: \"{}\"", selected_title);
        } else {
            println!("Found multiple results for '{}', please choose one:", search_term);
            for (idx, title) in search_results.iter().enumerate() {
                println!("{}. {}", idx + 1, title);
            }

            loop {
                print!("Enter number (1-{}): ", search_results.len());
                std::io::stdout().flush()?; 
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                
                match input.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= search_results.len() => {
                        selected_title = search_results.get(num - 1).unwrap().clone(); // Clone to get String
                        break;
                    }
                    _ => {
                        println!("Invalid input. Please enter a number between 1 and {}.", search_results.len());
                    }
                }
            }
            println!("[*] Selected article: \"{}\"", selected_title);
        }
        
        let mut wiki_client = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        if args.lang != "en" {
            let base = "https://".to_owned() + &args.lang.to_lowercase() + ".wikipedia.org/w/api.php";
            wiki_client.set_base_url(&base);
        }
        
        println!("[*] Fetching content for \"{}\"", selected_title);
        wiki_client.page_from_title(selected_title)
            .get_content()
            .map_err(WikipediaError::from)?
    };
    
    Wordlist::new(&args.output)?
        .write_contents(&content, args.count)?;

    println!("[+] Wordlist generated");
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
    // Count
    /// Set the maximum number of words for the wordlist
    #[structopt(short = "c", long)]
    count: Option<usize>,
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
    fn search(search: String) -> WikipediaResult<Vec<String>> {
        let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        println!("[*] Searching Wikipedia");
        let search_results = wiki.search(&search)?;
        Ok(search_results)
    }

    // Get random wiki page
    fn random_search() -> WikipediaResult<String> {
        let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        println!("[*] Using random search");
        Ok(wiki.random()?.unwrap_or("".to_string()))
    }
    
    // search in specific language
    fn language_search(search: String, language: String) -> WikipediaResult<Vec<String>>{
        let mut wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        let base = "https://".to_owned() + &language.to_lowercase() + ".wikipedia.org/w/api.php";
        wiki.set_base_url(&base);
        println!("[*] Searching Wikipedia with language: {:}", language.to_uppercase());
        let search_results = wiki.search(&search)?;
        Ok(search_results)
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
    fn write_contents(&mut self, content: &str, count: Option<usize>) -> Result<()> {
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

        if let Some(c) = count {
            sorted_list.truncate(c);
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{Read, Write}; // Read needed for verifying file content

    #[test]
    fn test_write_contents_truncates_correctly() -> Result<()> {
        let test_file_path = "test_truncate.txt";
        // Content words are chosen to be valid (alphanumeric, length >= 5)
        // and to test sorting as well.
        let content = "zzzzz yyyyy xxxxx wwwww vvvvv uuuuu"; // 6 words
        
        // Expected output after filtering, sorting, deduplication, and truncation to 3 words.
        // Original: "zzzzz yyyyy xxxxx wwwww vvvvv uuuuu"
        // Split: ["zzzzz", "yyyyy", "xxxxx", "wwwww", "vvvvv", "uuuuu"]
        // Filtered (all are valid): same as split
        // Sorted: ["uuuuu", "vvvvv", "wwwww", "xxxxx", "yyyyy", "zzzzz"]
        // Truncated to 3: ["uuuuu", "vvvvv", "wwwww"]
        // Joined with newline:
        let expected_output = "uuuuu\nvvvvv\nwwwww\n";

        // Scope to ensure wordlist and its file handle are dropped before reading
        {
            let mut wordlist = Wordlist::new(test_file_path)?;
            wordlist.write_contents(content, Some(3))?;
            wordlist.flush()?; // Ensure all buffered data is written to the file
        }

        let mut file = File::open(test_file_path)?;
        let mut actual_output = String::new();
        file.read_to_string(&mut actual_output)?;

        fs::remove_file(test_file_path)?; // Clean up

        assert_eq!(actual_output, expected_output);
        Ok(())
    }
}