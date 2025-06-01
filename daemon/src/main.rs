use action::Action;
use futures_util::StreamExt;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::time::Duration;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        Message,
        client::IntoClientRequest,
        protocol::{CloseFrame, frame::coding::CloseCode},
    },
};

mod action;
mod buttons;
mod macros;

static SECURE_TROLLWORD: &str = "my trollword";

#[tokio::main]
async fn main() {
    loop {
        let uri = mdns_loop();
        println!("{uri}");
        ws_loop(uri).await;
    }
}

fn mdns_loop() -> String {
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");
    let service_type = "_trollboard._tcp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");

    loop {
        if let Ok(ServiceEvent::ServiceResolved(info)) =
            receiver.recv_timeout(Duration::from_secs(10))
        {
            if let Some(ip) = info.get_addresses_v4().iter().next() {
                return format!("ws://{}:{}", ip, info.get_port());
                // return match ip {
                //     IpAddr::V4(v4) => format!("ws://{}:{}", v4, info.get_port()),
                //     IpAddr::V6(v6) => format!("ws://[{}]:{}", v6, info.get_port()),
                // };
            }
        }
    }
}

async fn ws_loop(uri: String) {
    let mut request = uri.clone().into_client_request().unwrap();
    request.headers_mut().insert(
        "X-Trollbuttons",
        serde_json::to_string(&buttons::vec())
            .unwrap()
            .parse()
            .unwrap(),
    );

    let (mut ws_stream, response) = match connect_async(request).await {
        Ok(ok) => ok,
        Err(_) => return,
    };

    if response
        .headers()
        .get("X-Trollword")
        .and_then(|h| h.to_str().ok())
        != Some(SECURE_TROLLWORD)
    {
        let _ = ws_stream
            .close(Some(CloseFrame {
                code: CloseCode::Policy,
                reason: "WRONG_TROLLWORD".into(),
            }))
            .await;
    }

    while let Some(thing) = ws_stream.next().await {
        println!("{thing:?}");

        match thing {
            Ok(Message::Text(msg)) => {
                let result = serde_json::from_str::<Action>(&msg);
                if let Ok(action) = result {
                    action.run();
                }
            }
            Err(_) => {
                println!("dying");
                break;
            }
            _ => {}
        }
    }
}
