//! Generates nicely formatted HTML from a text message

use maud::{html, Markup};
use itertools::Itertools;

use super::log::{TextMessage, TextLog, MessageKind, BodyKind, MmsMessagePart};

const CSS: &str = include_str!("sms.css");

pub fn render_log(log: &TextLog, contact: &str) -> Markup {
    let mut messages = log.iter()
        .filter(|message| message.contact_name() == contact)
        .collect_vec();
    messages.sort_by_key(|message| message.date());
    html! {
        (::maud::DOCTYPE)
        html {
            /*
             * Shamelessly stolen from https://bootsnipp.com/snippets/featured/message-chat-box
             * I heavily bastardized the HTML into this codepen: https://codepen.io/Techcable/pen/vzNxyy
             * Thankfully, I didn't have to mess very much with the CSS......
             */
            head {
                link rel="stylesheet"
                    href="https://stackpath.bootstrapcdn.com/bootstrap/4.1.3/css/bootstrap.min.css"
                    integrity="sha384-MCw98/SFnGE8fJT3GXwEOngsV7Zt27NXFoaoApmYm81iuXoPkFOJwJ8ERdknLPMO"
                    crossorigin="anonymous";
                style { (CSS) }
            }
            body {
                div class="container" {
                    h3 class="text-center" { "Messages with " (contact) }

                }
                @for message in messages {
                    (render_message(message))
                }
            }
        }
    }
}

pub fn render_message(message: &TextMessage) -> Markup {
    match message.kind() {
        MessageKind::Sent => {
            html!(div class="outgoing_msg" {
                div class="sent_msg" {
                    ({ render_body(message) })
                    span class="time_date" { ({ render_date(message) }) }
                }
            })
        },
        MessageKind::Received { .. } => {
            html!(div class="incoming_msg" {
                // TODO: incoming_msg_img
                div class="received_msg" {
                    div class="received_withd_msg" {
                        ({ render_body(message) })
                        span class="time_date" { ({ render_date(message) }) }
                    }
                }
            })
        },
    }
}
pub fn render_date(message: &TextMessage) -> Markup {
    let date_format = message.date().date().format("%A %B %e %Y").to_string();
    let time_format = message.date().time().format("%-I:%M %p").to_string();
    html!((time_format) "    |    " (date_format))
}
pub fn render_body(message: &TextMessage) -> Markup {
    match message.body() {
        BodyKind::Sms(text) => html! { p { (text) } },
        BodyKind::Mms { ref parts } => html! {
            @for part in &**parts {
                (render_part(part))
            }
        },
    }
}
pub fn render_part(message: &MmsMessagePart) -> Markup {
    let text = message.text.as_ref();
    let data = message.data.as_ref();
    match &*message.content_type {
        "application/smil" => html!(),
        "text/plain" => html!(p { (text.unwrap()) }),
        "image/jpeg" | "image/png" => {
            let data = format!(
                "data:{};base64,{}",
                &message.content_type,
                ::base64::encode(&**data.unwrap())
            );
            html!(img src=(data) {})
        }
        "audio/amr" => html!(p { b { "Unsupported audio" } }),
        "video/mp4" => html!(p { b { "Unsupported video (mp4)" } }),
        _ => unimplemented!("Unknown content type: {}", message.content_type)
    }
}