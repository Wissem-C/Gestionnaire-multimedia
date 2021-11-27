// use reqwest::Client;
// use reqwest::Request;
// use serde_json::Value;
// use std::io::{self, Read};
// use std::result;

// pub async fn scrap() {
//     let client = request::Client::builder().user_agent("Coucou").build()?;

//     println!("\nScrapping from the site MusicBrain");

//     println!("Input song artist");
//     let mut save1 = String::new();
//     io::stdin().read_line(&mut save1).unwrap();

//     println!("Input song artist");
//     let mut save2 = String::new();
//     io::stdin().read_line(&mut save2).unwrap();

//     let url = "https://musicbrainz.org/ws/2/recording/?query=recording:".to_owned()
//         + &save1
//         + "AND artist:"
//         + &save2
//         + "&limit=16fmt=json";

//     let result = client.get(url).send().await?.text().await?;
//     let v: Value = serde_json::from_str(&result)?;

//     let title = &v["recordings"][0]["title"]
//         .to_string()
//         .trim_matches('\"')
//         .to_string();

//     let mut artist = v["recordings"][0]["artist-credit"][0]["name"]
//         .to_string()
//         .trim_matches('\"')
//         .to_string();

//     let title = v["recordings"][0]["releases"][0]["title"]
//         .to_string()
//         .trim_matches('\"')
//         .to_string();

//     let year = v["recordings"][0]["first-release-date"]
//         .to_string()
//         .trim_matches('\"')
//         .split('-')
//         .next()
//         .unwrap()
//         .parse::<u32>()
//         .unwrap();

//     if v["recordings"][0]["artist-credit"][0]["joinphrase"].is_null() {
//         println!(
//             "\nSearch result :\nTitle : {}\nArtist :{}\nYear : {}",
//             title, artist, album, year
//         );
//         else {
//             artist = artist
//         }
//     }
// }
