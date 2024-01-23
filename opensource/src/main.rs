use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct Repository {
    name: String,
    full_name: String,
    private: bool,
    html_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get GitHub username, repository name, and personal access token from the user
    let owner = input("Enter your GitHub username:");
    let repo_name = input("Enter a GitHub repository name:");
    let token = input("Enter your GitHub personal access token:");

    // Build the GitHub API URL
    let url = format!("https://api.github.com/repos/{}/{}", owner.trim(), repo_name.trim());

    // Create a reqwest client with the personal access token
    let client = reqwest::Client::builder()
        .user_agent("My Rust Program")
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token.trim()))?,
            );
            headers
        })
        .build()?;

    // Make a GET request to the GitHub API
    let response = client.get(&url).send().await?;

    // Ensure the request was successful
    if !response.status().is_success() {
        eprintln!("Request failed with status: {:?}", response.status());
        return Err(Box::<dyn Error>::from("Request failed"));
    }

    // Parse the JSON response into a Repository struct
    let repository: Repository = response.json().await?;

    // Serialize the Repository struct to JSON string
    let json_output = serde_json::to_string(&repository)?;
    println!("Serialized Repository: {}", json_output);

    // Deserialize the JSON string back to a Repository struct
    let deserialized_repository: Repository = serde_json::from_str(&json_output)?;
    println!("Deserialized Repository: {:?}", deserialized_repository);

    Ok(())
}

fn input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
