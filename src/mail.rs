mod open_browser_delegate;

use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use google_gmail1::{
    api::ListMessagesResponse,
    oauth2::{
        authenticator::{Authenticator, AuthenticatorBuilder},
        authenticator_delegate::{DefaultInstalledFlowDelegate, InstalledFlowDelegate},
        authorized_user::AuthorizedUserSecret,
        storage::{TokenInfo, TokenStorage},
        ApplicationSecret, AuthorizedUserAuthenticator, InstalledFlowAuthenticator,
        InstalledFlowReturnMethod,
    },
    Gmail,
};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tokio::task::JoinSet;

use self::open_browser_delegate::OpenBrowserDelegate;

static GMAIL_SCOPES: &[&str] = &[
    "https://mail.google.com/",                         // email
    "https://www.googleapis.com/auth/userinfo.email",   // email address
    "https://www.googleapis.com/auth/userinfo.profile", // G+ profile
    "https://www.googleapis.com/auth/contacts",         // contacts
    "https://www.googleapis.com/auth/calendar",         // calendar
];

pub async fn make_client() -> Gmail<HttpsConnector<HttpConnector>> {
    // yes, you do indeed distribute your client secret with your app
    // it's fine to do this, but please don't abuse our API access <3
    let secret = google_gmail1::oauth2::read_application_secret("data/client_secret.json")
        .await
        .unwrap();

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("data/sensitive/tokencache.json") // todo: implement a secure custom token storage provider
        .flow_delegate(Box::new(OpenBrowserDelegate))
        .build()
        .await
        .unwrap();

    let client = hyper::Client::builder().build(
        hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build(),
    );

    let hub = Gmail::new(client, auth);

    // do a quick query to make sure we're authenticated
    let (result, _) = hub
        .users()
        .get_profile("me")
        .add_scope("https://mail.google.com/")
        .doit()
        .await
        .unwrap();

    hub
}

#[tokio::test]
async fn list_messages_works() {
    let hub = make_client().await;

    let (result, messages) = hub
        .users()
        .messages_list("jkelleyrtp@gmail.com")
        .add_scope("https://mail.google.com/")
        .doit()
        .await
        .unwrap();

    std::fs::write(
        "data/messages/sensitive.json",
        serde_json::to_string_pretty(&messages).unwrap(),
    )
    .unwrap();

    dbg!(result);
}

#[tokio::test]
async fn load_next_page() {
    let hub = make_client().await;

    let (result, messages) = hub
        .users()
        .messages_list("jkelleyrtp@gmail.com")
        .page_token("06141333176801119710")
        .add_scope("https://mail.google.com/")
        // .add_scope("https://www.googleapis.com/auth/gmail.metadata")
        .doit()
        .await
        .unwrap();

    std::fs::write(
        "data/messages2/sensitive.json",
        serde_json::to_string_pretty(&messages).unwrap(),
    )
    .unwrap();
}

pub async fn download_recent_messages() -> Vec<google_gmail1::api::Message> {
    let messages = std::fs::read_to_string("data/sensitive/messages.json").unwrap();
    let messages = serde_json::from_str::<ListMessagesResponse>(&messages).unwrap();

    let hub = make_client().await;

    let mut set: JoinSet<google_gmail1::Result<google_gmail1::api::Message>> = JoinSet::new();

    let hubbed = Arc::new(hub);
    for msg in messages.messages.unwrap().iter().take(500).cloned() {
        let hub = hubbed.clone();
        set.spawn(async move {
            let id = msg.id.as_ref().unwrap();

            let res = hub
                .users()
                .messages_get("jkelleyrtp@gmail.com", id)
                .add_scope("https://mail.google.com/")
                .doit()
                .await?;

            Ok(res.1)
        });
    }

    let mut messages = vec![];

    while let Some(Ok(msg)) = set.join_next().await {
        if let Ok(msg) = msg {
            messages.push(msg)
        }
    }

    messages.sort_by(|l, r| l.internal_date.cmp(&r.internal_date).reverse());

    // save
    std::fs::File::create("data/sensitive/index.json").unwrap();
    std::fs::write(
        "data/sensitive/index.json",
        serde_json::to_string_pretty(&messages).unwrap(),
    )
    .unwrap();

    messages
}

#[tokio::test]
async fn read_messages() {
    let messages = std::fs::read_to_string("data/sensitive/messages2.json").unwrap();
    let messages = serde_json::from_str::<ListMessagesResponse>(&messages).unwrap();

    let hub = make_client().await;

    let mut set: JoinSet<google_gmail1::Result<google_gmail1::api::Message>> = JoinSet::new();

    let hubbed = Arc::new(hub);
    for msg in messages.messages.unwrap().iter().take(200).cloned() {
        let hub = hubbed.clone();
        set.spawn(async move {
            let id = msg.id.as_ref().unwrap();

            let res = hub
                .users()
                .messages_get("jkelleyrtp@gmail.com", id)
                .add_scope("https://mail.google.com/")
                .doit()
                .await?;

            Ok(res.1)
        });
    }

    let mut messages = vec![];

    while let Some(Ok(msg)) = set.join_next().await {
        if let Ok(msg) = msg {
            messages.push(msg)
        }
    }

    messages.sort_by(|l, r| l.internal_date.cmp(&r.internal_date).reverse());

    for msg in messages {
        println!(
            "{:?} - {}",
            msg.internal_date,
            msg.snippet.unwrap_or_default()
        );
    }
}

#[test]
fn decode() {
    let body = "SGkgSm9uYXRoYW4sDQpJdCB3YW50ZWQgdG8gbWFrZSBzdXJlIHRoYXQgeW91IHJlc3BvbmQgdG8gVmlja2kncyBpbnF1aXJ5Lg0KVGhlIHN1cHBsaWVyIGRvY3VtZW50cyBuZWVkIHRvIGJlIGZpbGxlZCBvdXQgc28gdGhhdCBwcm9jdXJlbWVudCBjYW4gcHJvY2VlZCB3aXRoIHRoZSBhZ3JlZW1lbnQuDQpJcyB0aGVyZSBhbiBpc3N1ZSB3aXRoIHRoZSBkb2N1bWVudHM_DQoNClRoeCwNCi1TaWQuDQoNCkZyb206IFZpY2tpIFpob3UgPHZ6aG91QGZ1dHVyZXdlaS5jb20-DQpTZW50OiBUaHVyc2RheSwgTm92ZW1iZXIgMTAsIDIwMjIgMzowMSBQTQ0KVG86IEpvbmF0aGFuIEtlbGxleSA8amtlbGxleXJ0cEBnbWFpbC5jb20-DQpDYzogU2lkIEFza2FyeSA8c2Fza2FyeUBmdXR1cmV3ZWkuY29tPg0KU3ViamVjdDogUkU6IEludHJvZHVjaW5nIEpvbmF0aGFuIEtlbGx5DQoNCkhpIEpvbmF0aGFuLA0KDQpJIGhvcGUgdGhpcyBlbWFpbCBmaW5kcyB5b3Ugd2VsbC4gSSBhbSB3b3JraW5nIHdpdGggU2lkIGZvciB5b3VyIEZ1dHVyZXdlaSBjb25zdWx0aW5nIHByb2plY3QuIENhbiB5b3UgcGxlYXNlIHByb3ZpZGUgZm9sbG93aW5nIHN1cHBsaWVyIHF1YWxpZmljYXRpb24gZG9jdW1lbnRzPw0KDQoNCiAgMS4gIFctOQ0KICAyLiAgQmFuayBhY2NvdW50IGluZm8NCiAgMy4gIFN1cHBsaWVyIG9mIENvbmZsaWN0IG9mIEludGVyZXN0IChBdHRhY2hlZCkNCiAgNC4gIFN1cHBsaWVyIEludGVncml0eSBQb2xpY3kgKGF0dGFjaGVkKQ0KDQpUaGFua3MNClZpY2tpDQoNCkZyb206IFNpZCBBc2thcnkgPHNhc2thcnlAZnV0dXJld2VpLmNvbTxtYWlsdG86c2Fza2FyeUBmdXR1cmV3ZWkuY29tPj4NClNlbnQ6IFRodXJzZGF5LCBOb3ZlbWJlciAxMCwgMjAyMiA0OjQzIFBNDQpUbzogSm9uYXRoYW4gS2VsbGV5IDxqa2VsbGV5cnRwQGdtYWlsLmNvbTxtYWlsdG86amtlbGxleXJ0cEBnbWFpbC5jb20-PjsgVmlja2kgWmhvdSA8dnpob3VAZnV0dXJld2VpLmNvbTxtYWlsdG86dnpob3VAZnV0dXJld2VpLmNvbT4-DQpTdWJqZWN0OiBJbnRyb2R1Y2luZyBKb25hdGhhbiBLZWxseQ0KDQpIaSBWaWNraSwNCkpvbmF0aGFuIHdpbGwgYmUgZG9pbmcgYW4gU09XIGZvciBSdXN0IEdVSS4NCg0KSm9uYXRoYW4sDQpNZWV0IFZpY2tpLCBvdXIgcHJvY3VyZW1lbnQgbWFuYWdlciwgd2hvIHdpbGwgYmUgdGFraW5nIHVzIHRocm91Z2ggdGhlIGFncmVlbWVudCBwcm9jZXNzLg0KDQpUYWtlIGNhcmUsDQotU2lkLg0K";

    // base64::decode_config(body, base64::BINHEX).map(|_| println!("ok, BINHEX"));
    // base64::decode_config(body, base64::BCRYPT).map(|_| println!("ok, BCRYPT"));
    // base64::decode_config(body, base64::CRYPT).map(|_| println!("ok, CRYPT"));
    // base64::decode_config(body, base64::IMAP_MUTF7).map(|_| println!("ok, IMAP_MUTF7"));
    // base64::decode_config(body, base64::STANDARD).map(|_| println!("ok, STANDARD"));
    // base64::decode_config(body, base64::STANDARD_NO_PAD).map(|_| println!("ok, STANDARD_NO_PAD"));
    // base64::decode_config(body, base64::URL_SAFE).map(|_| println!("ok, URL_SAFE"));
    // base64::decode_config(body, base64::URL_SAFE_NO_PAD).map(|_| println!("ok, URL_SAFE_NO_PAD"));
}
