use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use google_gmail1::{
    api::ListMessagesResponse,
    oauth2::{
        authenticator::{Authenticator, AuthenticatorBuilder},
        authenticator_delegate::{DefaultInstalledFlowDelegate, InstalledFlowDelegate},
        authorized_user::AuthorizedUserSecret,
        storage::{TokenInfo, TokenStorage},
        ApplicationSecret, AuthorizedUserAuthenticator, InstalledFlowAuthenticator,
    },
    Gmail,
};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tokio::task::JoinSet;

pub struct OpenBrowserDelegate;
impl InstalledFlowDelegate for OpenBrowserDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        _need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        Box::pin(async move {
            webbrowser::open(url).unwrap();
            Ok(String::new())
        }) as Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>>
    }
}
