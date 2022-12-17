use dioxus::prelude::UnboundedReceiver as Ur;
use std::{rc::Rc, time::Duration};
use tokio::sync::mpsc::UnboundedReceiver;

use fermi::{AtomRoot, Readable};
use futures_util::{Stream, StreamExt};
use google_gmail1::api::{ListMessagesResponse, Message};
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport,
    SmtpTransport, Tokio1Executor, Transport,
};

use crate::{mail, state::MESSAGES};

pub enum MailAction {
    UndoSend,
    AddAccount { email: String },
    Send { body: lettre::Message },
}

type GmailClient = google_gmail1::Gmail<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>;

/// Called when the app is launched
pub async fn main_loop(mut cx: Ur<MailAction>, root: Rc<AtomRoot>) {
    // Boot up the client, make sure we're logged in
    // Waits for the user to log in if they aren't already
    let mut client: GmailClient = crate::mail::make_client().await;

    // load the index from disk
    let last_id = {
        let messages = root.read(MESSAGES);
        if messages.is_empty() {
            let new_messages = mail::download_recent_messages().await;
            root.set(MESSAGES.unique_id(), new_messages);
        }

        messages.last().cloned().unwrap().id.unwrap()
    };

    let mut rx = wait_for_task(cx);

    while let Some(action) = rx.recv().await {
        match action {
            SelectResult::Poll => {
                let new_messages = fetch_messages(&mut client, &last_id).await;
                if !new_messages.is_empty() {
                    let mut messages = root.read(MESSAGES);
                    let mut new = messages.to_vec();
                    new.extend(new_messages);

                    root.set(MESSAGES.unique_id(), new);
                }
            }
            SelectResult::Message(_) => todo!(),
        }
    }
}

/// A dedicated watcher that transforms all actions the client might take
fn wait_for_task(mut cx: Ur<MailAction>) -> UnboundedReceiver<SelectResult> {
    // todo: dynamic poll based on battery power
    let mut poll_interval = tokio::time::interval(Duration::from_secs(10));

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                tick = poll_interval.tick() => {
                    tx.send(SelectResult::Poll);
                },
                msg = cx.next() => {
                    if let Some(msg) = msg {
                        tx.send(SelectResult::Message(msg));
                    }
                }
            };
        }
    });

    rx
}

enum SelectResult {
    Poll,
    Message(MailAction),
}

async fn send_message(email: lettre::Message) {
    const USERNAME: &str = "jkelleyrtp@gmail.com";
    const PASSWORD: &str = "rqgvegctdaproiue";

    let creds = Credentials::new(USERNAME.into(), PASSWORD.into());

    // Open a remote connection to gmail
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(email).await {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

async fn fetch_messages(client: &mut GmailClient, last_id: &str) -> Vec<Message> {
    // query the gmail api for new messages
    // if there are new messages, add them to the index
    // if there are messages that have been deleted, remove them from the index
    // if there are messages that have been modified, update them in the index
    let (mut result, mut messages) = client
        .users()
        .messages_list("jkelleyrtp@gmail.com")
        .add_scope("https://mail.google.com/")
        .doit()
        .await
        .unwrap();

    // Queue all the new messages to be added the index
    let mut messages_to_add = vec![];

    'main: loop {
        for message in messages.messages.as_mut().unwrap().drain(..) {
            if last_id == message.id.as_ref().unwrap() {
                break 'main;
            }

            messages_to_add.push(message);
        }

        if messages_to_add.len() < 100 {
            break;
        }

        // load the next page

        (result, messages) = client
            .users()
            .messages_list("jkelleyrtp@gmail.com")
            .add_scope("https://mail.google.com/")
            .page_token(messages.next_page_token.as_ref().unwrap())
            .doit()
            .await
            .unwrap();
    }

    messages_to_add
}
