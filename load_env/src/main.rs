use clap::Parser;
use dotenvy::from_path_iter;
use std::path::PathBuf;
use std::process;

/// Load environment variables from a .env file into the current shell session
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "Load environment variables from a .env file into the current shell session.\n\nThis tool reads environment variables from a .env file and outputs shell export \ncommands that can be evaluated to set those variables in the current shell.\n\nUsage example:\n\tload_env .env\n\tload_env /path/to/your/.env\n\nTo actually set the environment variables in your current shell, you need to \nevaluate the output of this tool:\n\tsource <(load_env .env)         # for bash/zsh\n\tload_env .env | source          # for fish",
    after_help = "Note: To actually set the environment variables in your current shell, you need to \nevaluate the output of this tool:\n\tsource <(load_env .env)         # for bash/zsh\n\tload_env .env | source          # for fish"
)]
struct Args {
    /// Path to the .env file
    #[clap(value_parser)]
    env_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    // Check if the file exists
    if !args.env_file.exists() {
        eprintln!(
            "Error: The specified .env file does not exist: {:?}",
            args.env_file
        );
        process::exit(1);
    }

    // Check if the path is a file
    if !args.env_file.is_file() {
        eprintln!(
            "Error: The specified path is not a file: {:?}",
            args.env_file
        );
        process::exit(1);
    }

    // Try to parse the .env file
    match from_path_iter(&args.env_file) {
        Ok(iter) => {
            // Output export commands for each environment variable
            // This allows the shell to evaluate the output and set the variables
            for item in iter {
                match item {
                    Ok((key, value)) => {
                        // Skip empty keys
                        if key.is_empty() {
                            continue;
                        }
                        // Properly escape the value for shell export
                        let escaped_value = escape_for_shell(&value);
                        println!("export {}={}", key, escaped_value);
                    }
                    Err(e) => {
                        eprintln!("Error parsing .env file: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading .env file: {}", e);
            process::exit(1);
        }
    }
}

/// Escape a string for safe use in a shell export command
fn escape_for_shell(value: &str) -> String {
    // For simplicity, we'll wrap the value in single quotes
    // and escape any single quotes within the value
    let escaped = value.replace("'", "'\"'\"'");
    format!("'{}'", escaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_for_shell() {
        // Test basic string without quotes
        assert_eq!(escape_for_shell("hello"), "'hello'");

        // Test string with single quotes
        assert_eq!(escape_for_shell("hello'world"), "'hello'\"'\"'world'");

        // Test string with multiple single quotes
        assert_eq!(
            escape_for_shell("hello''world"),
            "'hello'\"'\"''\"'\"'world'"
        );

        // Test empty string
        assert_eq!(escape_for_shell(""), "''");

        // Test string with special characters
        assert_eq!(escape_for_shell("hello$world"), "'hello$world'");
    }

    #[test]
    fn test_escape_for_shell_complex() {
        // Test string with double quotes
        assert_eq!(escape_for_shell("hello\"world"), "'hello\"world'");

        // Test string with mixed quotes
        assert_eq!(escape_for_shell("hello'\"world"), "'hello'\"'\"'\"world'");

        // Test string with spaces
        assert_eq!(escape_for_shell("hello world"), "'hello world'");

        // Test string with special shell characters
        assert_eq!(escape_for_shell("hello $world !@#"), "'hello $world !@#'");
    }
}
