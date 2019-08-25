//! This module handles sending push notifications to clients.

use futures::{future::Future, lazy};
use lazy_static::lazy_static;
use std::{fs::File, io::Read, thread, time::Duration};
use web_push::{
    ContentEncoding, SubscriptionInfo, VapidSignatureBuilder, WebPushClient, WebPushMessage,
    WebPushMessageBuilder,
};

/// Sends the message to the client with a delay.
pub fn add_to_queue(
    subscription_info: SubscriptionInfo,
    message: serde_json::Value,
    delay: Option<u16>,
) -> Result<(), NotificationSendError> {
    let (client, message) = prepare_push(subscription_info, &message.to_string())?;

    thread::spawn(move || {
        if let Some(delay) = delay {
            thread::sleep(Duration::from_secs(delay.into()));
        }

        send_push(client, message);
    });

    Ok(())
}

/// Represents the possible errors when sending a notification.
pub enum NotificationSendError {
    /// No private key is available for sending a notification.
    NoPrivateKey,
    /// Building the VAPID signature failed.
    SignatureBuildFailed,
    /// Building the message failed.
    MessageBuildFailed,
}

/// Send a push notification to the given subscription info with the given message.
fn send_push(client: WebPushClient, message: WebPushMessage) {
    tokio::run(lazy(move || {
        tokio::spawn(
            client
                .send_with_timeout(message, std::time::Duration::from_secs(4))
                .map(|_| ())
                .map_err(|_| ()),
        )
    }));
}

/// Prepare everything to send a push message.
fn prepare_push(
    subscription_info: SubscriptionInfo,
    message: &str,
) -> Result<(WebPushClient, WebPushMessage), NotificationSendError> {
    lazy_static! {
        // Read the file once instead of every time a notification is sent.
        static ref VAPID_FILE_CONTENT: Option<Vec<u8>> = File::open(&*crate::configuration::VAPID_PRIVATE_FILE)
            .and_then(|mut file| {
                let mut result = Vec::new();
                file.read_to_end(&mut result)?;

                Ok(result)
            })
            .ok();
    }

    // Get a reference to the file content.
    let file_content = match &*VAPID_FILE_CONTENT {
        Some(content) => content,
        None => return Err(NotificationSendError::NoPrivateKey),
    };
    let file_content = &file_content[..];

    // Build a VAPID signature.
    let mut signature_builder = VapidSignatureBuilder::from_pem(file_content, &subscription_info)
        .map_err(|_| NotificationSendError::SignatureBuildFailed)?;
    signature_builder.add_claim(
        "sub",
        format!(
            "{}{}",
            "mailto:",
            &*crate::configuration::CONTACT_EMAIL_ADDRESS
        ),
    );
    let signature = signature_builder
        .build()
        .map_err(|_| NotificationSendError::SignatureBuildFailed)?;

    // Build the actual message.
    let mut message_builder = WebPushMessageBuilder::new(&subscription_info)
        .map_err(|_| NotificationSendError::MessageBuildFailed)?;
    message_builder.set_payload(ContentEncoding::AesGcm, message.as_bytes());
    message_builder.set_ttl(0);
    message_builder.set_vapid_signature(signature);
    let message = message_builder
        .build()
        .map_err(|_| NotificationSendError::MessageBuildFailed)?;

    // Create a client.
    let client = WebPushClient::new().map_err(|_| NotificationSendError::MessageBuildFailed)?;

    Ok((client, message))
}
