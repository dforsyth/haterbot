extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use slack::{api, Event, EventHandler, Message, RtmClient};

use handlers;

pub struct HaterBot {
    name:   String,
    icon:   String,
    token:  String,
    op_map: HashMap<&'static str, Box<handlers::Handler>>,
    client: api::requests::Client,
}

pub struct HaterBotConfig {
    pub name:  String,
    pub icon:  String,
    pub token: String,
}

impl HaterBot {
    pub fn new(config: HaterBotConfig) -> HaterBot {
        HaterBot {
            name:   config.name,
            icon:   config.icon,
            token:  config.token,
            op_map: HashMap::new(),
            client: api::requests::default_client().unwrap(),
        }
    }

    fn handle_message(&self, message: Message) {
        match message {
            Message::Standard(message) => {
                if let Some(parsed) = self.parse_command(message.text) {
                    match self.handle_command(parsed) {
                        Ok((response, attachments)) => {
                            if response.len() > 0 || attachments.is_some() {
                                if let Some(channel) = message.channel {
                                    info!("handle_message {:?}", attachments);
                                    self.send_message(
                                        channel,
                                        response,
                                        attachments,
                                    );
                                } else {
                                    error!("Missing channel.");
                                }
                            } else {
                                debug!("No response");
                            }
                        },
                        Err(err) => error!("{:?}", err),
                    }
                }
            },
            _ => debug!("Unsupported message type"),
        };
    }

    fn parse_command(&self, text: Option<String>) -> Option<Vec<String>> {
        match text {
            Some(text) => {
                if !text.starts_with("!") {
                    return None;
                }

                let tokens = text.split(' ').map(|s| s.to_string()).collect();
                info!("tokens: {:?}", tokens);
                Some(tokens)
            },
            _ => None,
        }
    }

    fn handle_command(
        &self,
        tokens: Vec<String>,
    ) -> Result<(String, Option<serde_json::Value>), String> {
        let command = &tokens[0][1..];
        let args = &tokens[1..];

        if let Some(handler) = self.op_map.get(command) {
            let (response, attachments) = handler.handle(args);
            Ok((response, attachments))
        } else {
            Err(format!("No handler for {}", command))
        }
    }

    fn send_message(
        &self,
        channel: String,
        response: String,
        attachments: Option<serde_json::Value>,
    ) {
        // TODO: Return something useful.
        debug!("Sending response to {}: {}", channel, response);
        info!("attachments {:?}", attachments);

        let _attachments = attachments.map(|v| v.to_string());
        let _attachments = _attachments.as_ref().map(String::as_str);

        let _ = api::chat::post_message(
            &self.client,
            &self.token,
            &api::chat::PostMessageRequest {
                channel: channel.as_str(),
                text: response.as_str(),
                username: Some(&self.name),
                icon_url: Some(&self.icon),
                attachments: _attachments,
                ..api::chat::PostMessageRequest::default()
            },
        );
    }

    pub fn run(&mut self) {
        let token = self.token.clone();
        RtmClient::login_and_run(token.as_str(), self)
            .expect("RTM client failed.");
    }

    pub fn add_command(
        &mut self,
        command: &'static str,
        handler: Box<handlers::Handler>,
    ) {
        self.op_map.insert(command, handler);
    }
}

#[allow(unused_variables)]
impl EventHandler for HaterBot {
    fn on_connect(&mut self, cli: &RtmClient) {
        info!("Connected")
    }

    fn on_event(&mut self, cli: &RtmClient, ev: Event) {
        match ev {
            Event::Message(m) => {
                // Need nightly to match on box
                self.handle_message(*m);
            },
            _ => debug!("Unsupported event."),
        };
    }

    fn on_close(&mut self, cli: &RtmClient) {
        info!("Closed")
    }
}
