use std::fs;
use std::env;

fn main () {
    // step 1: fetch file list from GitHub API
    // step 2: for each .txt file, download content
    // step 3: build the logos.rs string
    // step 4: write to OUT_DIR

    println!("cargo:rerun-if-changed=build.rs"); // Only rebuild if build.rs changes

    let response = reqwest::blocking::Client::new() // response to the HTTP get request to the GitHub API that fetches the directory listing
        .get("https://api.github.com/repos/fastfetch-cli/fastfetch/contents/src/logo/ascii")
        .header("User-Agent", "holefetch-build")
        .send().unwrap().text().unwrap();

    let client = reqwest::blocking::Client::new(); // create a new HTTP client for downloading the logos

    let json: serde_json::Value = serde_json::from_str(&response).unwrap(); // convert the response to a JSON value

    let mut logo_names: Vec<String> = Vec::new();
    let mut match_arms = String::new();

    for entry in json.as_array().unwrap() {
        // The GITHUB API has a guaranteed structure of a JSON array of objects, each with a name and download_url property
        let name = entry["name"].as_str().unwrap_or_default(); //get the distro name

        let logo_name = name.strip_suffix(".txt").unwrap_or(name); // strip the .txt extension from the name

        let url = entry["download_url"].as_str().unwrap_or_default(); //get the distro logo url

        let raw_logo = client.get(url).header("User-Agent", "holefetch-build").send().unwrap().text().unwrap(); // download the logo

        let logo = raw_logo.replace("\r", ""); // remove carriage returns

        logo_names.push(logo_name.to_string()); // add the logo name to the list

        match_arms.push_str(&format!("\"{}\" => Some(r#\"{}\"#),\n", logo_name, logo)); // add the logo to the match arm


    }
}
//
//let path = format!("src/logo/ascii/{}", name);
//std::fs::write(path, response).unwrap();