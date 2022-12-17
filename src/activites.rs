use futures::StreamExt;
use std::{rc::Rc, time::Duration};

use dioxus::prelude::UnboundedReceiver;
use fermi::{AtomRoot, Readable};
use google_gmail1::api::ListMessagesResponse;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    SmtpTransport, Tokio1Executor, Transport,
};

use crate::{mail, state::MESSAGES};

pub enum MailAction {
    UndoSend,
    AddAccount { email: String },
    Send { body: lettre::Message },
}

/// Called when the app is launched
pub async fn main_loop(mut cx: UnboundedReceiver<MailAction>, root: Rc<AtomRoot>) {
    // Boot up the client, make sure we're logged in
    // Waits for the user to log in if they aren't already
    let mut hub = crate::mail::make_client().await;

    // load the index from disk
    let last_id = {
        let messages = root.read(MESSAGES);
        if messages.is_empty() {
            let new_messages = mail::download_recent_messages().await;
            root.set(MESSAGES.unique_id(), new_messages);
        }

        messages.last().cloned().unwrap().id.unwrap()
    };

    // todo: dynamic poll based on battery power
    let mut poll_interval = tokio::time::interval(Duration::from_secs(60));

    loop {
        tokio::select! {
            tick = poll_interval.tick() => {
                // query the gmail api for new messages
                // if there are new messages, add them to the index
                // if there are messages that have been deleted, remove them from the index
                // if there are messages that have been modified, update them in the index
                let (result, messages) = hub
                    .users()
                    .messages_list("jkelleyrtp@gmail.com")
                    .add_scope("https://mail.google.com/")
                    .doit()
                    .await
                    .unwrap();


                // Queue all the new messages to be added the index
                let mut messages_to_add = vec![];
                for message in messages.messages.unwrap() {
                    if &last_id == message.id.as_ref().unwrap() {
                        break;
                    }
                    messages_to_add.push(message);
                }


            }
            msg = cx.next() => {
                //
            }
        }
    }
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
