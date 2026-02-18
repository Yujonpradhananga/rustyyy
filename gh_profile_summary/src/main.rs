use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
struct Repo {
    name: String,
    stargazers_count: u32,
    forks_count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gh_name = user_input().await;
    let gh_name = gh_name.trim().to_string();
    let mut body = get_req(&gh_name).await?;
    body.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
    println!("{gh_name}'s top 5 starred repos:");
    for repo in body.iter().take(5) {
        println!("Name: {}", repo.name);
        println!("Stars: {}", repo.stargazers_count);
        println!("Forks: {}", repo.forks_count);
        println!("---");
    }
    Ok(())
}

async fn user_input() -> String {
    println!("Enter the Github username: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

async fn get_req(gh_name: &str) -> Result<Vec<Repo>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!("https://api.github.com/users/{}/repos", gh_name))
        .header("User-Agent", "gh_profile_summary")
        .send()
        .await?
        .json::<Vec<Repo>>()
        .await?;
    Ok(body)
}
