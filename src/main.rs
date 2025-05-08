use chrono::Utc;
use dotenv::dotenv;
use serde::Serialize;
use std::{thread, time::Duration};

const BTC_URL: &str = "https://blockchain.info/ticker";

#[derive(Serialize)]
pub struct DiscordWebhookPayload<'a> {
    pub content: &'a str,
    pub embeds: Vec<DiscordEmbed<'a>>,
}

#[derive(Serialize)]
pub struct DiscordEmbed<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub color: u32,
    pub fields: Vec<DiscordEmbedField<'a>>,
    pub footer: Option<DiscordEmbedFooter<'a>>,
    pub timestamp: Option<&'a str>,
}

#[derive(Serialize)]
pub struct DiscordEmbedField<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub inline: bool,
}

#[derive(Serialize)]
pub struct DiscordEmbedFooter<'a> {
    pub text: &'a str,
}

fn main() {
    dotenv().ok();

    // get the price at which we send the webhook
    let price_alert: f32 = dotenv::var("ALERT_PRICE")
        .unwrap_or(String::from("70000"))
        .to_string()
        .parse()
        .expect("ERROR: couldnt parse value ALERT_PRICE");

    // check how many times we check (wait interval)
    let check_time: u64 = dotenv::var("CHECK_TIME")
        .unwrap_or((60 * 15).to_string())
        .to_string()
        .parse()
        .expect("ERROR: couldnt parse value CHECK_TIME");

    // timeout before another alert is sent
    let alert_timeout: u64 = dotenv::var("ALERT_TIMEOUT")
        .unwrap_or((60 * 60 * 24).to_string())
        .to_string()
        .parse()
        .expect("ERROR: couldnt parse value ALERT_TIMEOUT");

    // url to the discord webhook
    let webhook_url: String = dotenv::var("WEBHOOK").expect("ERROR: couldnt parse value WEBHOOK");

    loop {
        let (trigger_alert, btc_price) = check_btc_price(price_alert);
        if trigger_alert {
            send_discord_message(&webhook_url, price_alert, btc_price);
            thread::sleep(Duration::from_secs(alert_timeout));
        }

        thread::sleep(Duration::from_secs(check_time));
    }
}

fn check_btc_price(price_alert: f32) -> (bool, f32) {
    let resp = match reqwest::blocking::get(BTC_URL) {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("ERROR: failed to make API request: {}", err),
    };

    let mut json_root = match json::parse(&resp) {
        Ok(j_root) => j_root,

        Err(err) => panic!("ERROR: failed to parse json: {}", err),
    };

    let mut eur = json_root.remove("EUR");
    let btc_price = match eur.remove("buy").as_f32() {
        Some(btc_value) => btc_value,
        None => panic!("ERROR: couldnt find price in request"),
    };

    if btc_price < price_alert {
        (true, btc_price)
    } else {
        (false, btc_price)
    }
}

fn send_discord_message(webhook_url: &str, price_alert: f32, btc_price: f32) {
    let client = reqwest::blocking::Client::new();
    let time = Utc::now().to_rfc3339();
    let price_alert_text = format!("${price_alert}");
    let btc_price_text = format!("${btc_price}");

    let payload = DiscordWebhookPayload {
        content: "@here ⚠️ BTC Price Alert",
        embeds: vec![DiscordEmbed {
            title: "Bitcoin Price Drop",
            description: "The price of BTC has dropped below your threshold!",
            color: 16711680,
            fields: vec![
                DiscordEmbedField {
                    name: "Current Price",
                    value: &btc_price_text,
                    inline: true,
                },
                DiscordEmbedField {
                    name: "Threshold",
                    value: &price_alert_text,
                    inline: true,
                },
            ],
            footer: Some(DiscordEmbedFooter {
                text: "BTC Alert Bot",
            }),
            timestamp: Some(&time),
        }],
    };

    let res = match client.post(webhook_url).json(&payload).send() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("ERROR: failed to send webhook {e}");
            return;
        }
    };

    if !res.status().is_success() {
        eprintln!("ERROR: failed to send webhook incorrect status code");
    }
}
