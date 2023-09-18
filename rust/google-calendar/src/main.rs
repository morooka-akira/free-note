mod oauth;

use oauth::get_access_token;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Events {
    items: Vec<EventItem>,
}

#[derive(Deserialize, Debug)]
struct EventItem {
    summary: String,
    start: EventTime,
    end: EventTime,
}

#[derive(Deserialize, Debug)]
struct EventTime {
    date_time: Option<String>,
}

#[tokio::main]
async fn main() {
    let result = get_access_token().await;

    let token = result.unwrap();

    let resp = get_calenders(&token.access_token).await.unwrap();

    for event in resp.items {
        println!("Event: {}", event.summary);
        if let Some(start_time) = event.start.date_time {
            println!("Starts at: {}", start_time);
        }
    }
}
async fn get_calenders(access_token: &str) -> Result<Events, reqwest::Error> {
    let events_url = format!(
        "https://www.googleapis.com/calendar/v3/calendars/{}/events?access_token={}",
        "calender_url", access_token
    );
    let events: Events = Client::new().get(&events_url).send().await?.json().await?;
    println!(
        "{:?}",
        Client::new().get(&events_url).send().await?.text().await?
    );

    Ok(events)
}
