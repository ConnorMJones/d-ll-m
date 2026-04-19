use dioxus::prelude::*;
use dllm_client::{
    ClientConfig, ClientSnapshot, ConnectionStatus, DEFAULT_DATABASE_NAME, DEFAULT_URI, DllmClient,
};
use std::time::Duration;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    let client = use_hook(DllmClient::new);
    let mut snapshot = use_signal(ClientSnapshot::default);
    let mut uri = use_signal(|| DEFAULT_URI.to_string());
    let mut database_name = use_signal(|| DEFAULT_DATABASE_NAME.to_string());
    let mut display_name = use_signal(String::new);
    let mut message_text = use_signal(String::new);

    use_future({
        let client = client.clone();
        move || {
            let client = client.clone();
            async move {
                loop {
                    snapshot.set(client.snapshot());
                    tokio::time::sleep(Duration::from_millis(150)).await;
                }
            }
        }
    });

    let status_text = status_text(
        snapshot().connection_status,
        snapshot().subscription_applied,
    );
    let is_connected = matches!(
        snapshot().connection_status,
        ConnectionStatus::Connected | ConnectionStatus::Connecting
    );
    let connect_client = client.clone();
    let disconnect_client = client.clone();
    let set_name_client = client.clone();
    let send_message_client = client.clone();

    rsx! {
        div {
            style: "font-family: 'Iosevka', 'IBM Plex Sans', sans-serif; background: linear-gradient(135deg, #f3ecde 0%, #ddd4c0 100%); min-height: 100vh; color: #1f1c18; padding: 24px;",
            div {
                style: "max-width: 1100px; margin: 0 auto;",
                h1 { style: "font-size: 32px; margin-bottom: 8px;", "d-ll-m desktop client" }
                p { style: "margin-top: 0; color: #4f4a43;", "First client slice: connect, identify, watch presence, send messages." }

                if let Some(error) = snapshot().last_error.clone() {
                    div {
                        style: "background: #f8d7cf; border: 1px solid #d9907a; border-radius: 12px; padding: 12px 14px; margin-bottom: 16px;",
                        "{error}"
                    }
                }

                div {
                    style: "display: grid; grid-template-columns: 320px 1fr; gap: 20px;",
                    section {
                        style: "background: rgba(255,255,255,0.72); border: 1px solid rgba(50,40,30,0.12); border-radius: 18px; padding: 18px; box-shadow: 0 20px 50px rgba(80, 60, 30, 0.08);",
                        h2 { style: "margin-top: 0;", "Connection" }
                        div { style: "font-size: 14px; color: #5b544b; margin-bottom: 12px;", "Status: {status_text}" }
                        label { style: "display: block; font-size: 13px; margin-bottom: 6px; color: #5b544b;", "Server URI" }
                        input {
                            style: input_style(),
                            value: "{uri}",
                            disabled: is_connected,
                            oninput: move |event| uri.set(event.value()),
                        }
                        label { style: "display: block; font-size: 13px; margin: 12px 0 6px; color: #5b544b;", "Database" }
                        input {
                            style: input_style(),
                            value: "{database_name}",
                            disabled: is_connected,
                            oninput: move |event| database_name.set(event.value()),
                        }
                        div { style: "display: flex; gap: 10px; margin-top: 14px;",
                            button {
                                style: primary_button_style(),
                                disabled: is_connected,
                                onclick: move |_| {
                                    let _ = connect_client.connect(ClientConfig {
                                        uri: uri(),
                                        database_name: database_name(),
                                    });
                                },
                                "Connect"
                            }
                            button {
                                style: secondary_button_style(),
                                disabled: !is_connected,
                                onclick: move |_| {
                                    let _ = disconnect_client.disconnect();
                                },
                                "Disconnect"
                            }
                        }

                        h2 { style: "margin: 22px 0 8px;", "Identity" }
                        label { style: "display: block; font-size: 13px; margin-bottom: 6px; color: #5b544b;", "Display name" }
                        input {
                            style: input_style(),
                            value: "{display_name}",
                            disabled: !matches!(snapshot().connection_status, ConnectionStatus::Connected),
                            oninput: move |event| display_name.set(event.value()),
                        }
                        button {
                            style: primary_button_style(),
                            disabled: !matches!(snapshot().connection_status, ConnectionStatus::Connected),
                            onclick: move |_| {
                                let _ = set_name_client.set_name(display_name());
                            },
                            "Set name"
                        }

                        div { style: "margin-top: 18px; font-size: 13px; color: #5b544b;",
                            if let Some(identity) = snapshot().local_identity.clone() {
                                p { style: "margin: 0;", "Identity: {identity}" }
                            } else {
                                p { style: "margin: 0;", "Identity: not connected" }
                            }
                        }
                    }

                    section {
                        style: "background: rgba(255,255,255,0.72); border: 1px solid rgba(50,40,30,0.12); border-radius: 18px; padding: 18px; box-shadow: 0 20px 50px rgba(80, 60, 30, 0.08); display: grid; grid-template-rows: auto 1fr auto; min-height: 640px;",
                        div {
                            style: "display: grid; grid-template-columns: 220px 1fr; gap: 18px;",
                            div {
                                h2 { style: "margin-top: 0;", "Users" }
                                div { style: "display: flex; flex-direction: column; gap: 8px;" ,
                                    for user in snapshot().users {
                                        div {
                                            key: "{user.identity}",
                                            style: "padding: 10px 12px; border-radius: 12px; background: rgba(96, 84, 67, 0.08);",
                                            div { style: "font-weight: 600;", "{user.display_name()}" }
                                            div { style: "font-size: 12px; color: #5b544b; overflow: hidden; text-overflow: ellipsis;", "{user.identity}" }
                                            div { style: "font-size: 12px; color: #5b544b; margin-top: 4px;", if user.online { "online" } else { "offline" } }
                                        }
                                    }
                                    if snapshot().users.is_empty() {
                                        div { style: "font-size: 14px; color: #5b544b;", "No users yet." }
                                    }
                                }
                            }
                            div {
                                h2 { style: "margin-top: 0;", "Messages" }
                                div {
                                    style: "display: flex; flex-direction: column; gap: 10px; max-height: 460px; overflow-y: auto; padding-right: 8px;",
                                    for message in snapshot().messages {
                                        article {
                                            key: "{message.id}",
                                            style: "padding: 12px 14px; border-radius: 14px; background: rgba(33, 28, 24, 0.06);",
                                            div { style: "display: flex; justify-content: space-between; gap: 12px; font-size: 12px; color: #5b544b;",
                                                strong { style: "color: #1f1c18;", "{message.sender_name}" }
                                                span { "{message.sent}" }
                                            }
                                            p { style: "margin: 8px 0 0; white-space: pre-wrap;", "{message.text}" }
                                        }
                                    }
                                    if snapshot().messages.is_empty() {
                                        div { style: "font-size: 14px; color: #5b544b;", "No messages yet." }
                                    }
                                }
                            }
                        }

                        div {}

                        div {
                            style: "display: flex; gap: 10px; align-items: flex-end;",
                            textarea {
                                style: "flex: 1; min-height: 88px; border-radius: 14px; border: 1px solid rgba(50,40,30,0.16); padding: 12px 14px; background: rgba(255,255,255,0.8); resize: vertical;",
                                value: "{message_text}",
                                disabled: !matches!(snapshot().connection_status, ConnectionStatus::Connected),
                                oninput: move |event| message_text.set(event.value()),
                            }
                            button {
                                style: primary_button_style(),
                                disabled: !matches!(snapshot().connection_status, ConnectionStatus::Connected),
                                onclick: move |_| {
                                    let message = message_text();
                                    if send_message_client.send_message(message.clone()).is_ok() {
                                        message_text.set(String::new());
                                    }
                                },
                                "Send"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn status_text(status: ConnectionStatus, subscription_applied: bool) -> &'static str {
    match (status, subscription_applied) {
        (ConnectionStatus::Disconnected, _) => "disconnected",
        (ConnectionStatus::Connecting, _) => "connecting",
        (ConnectionStatus::Connected, false) => "connected, waiting for subscription",
        (ConnectionStatus::Connected, true) => "connected",
    }
}

fn input_style() -> &'static str {
    "width: 100%; border-radius: 14px; border: 1px solid rgba(50,40,30,0.16); padding: 10px 12px; background: rgba(255,255,255,0.8);"
}

fn primary_button_style() -> &'static str {
    "border: none; border-radius: 999px; padding: 10px 16px; background: #7f4b2a; color: #fffdf8; font-weight: 600; cursor: pointer;"
}

fn secondary_button_style() -> &'static str {
    "border: 1px solid rgba(50,40,30,0.18); border-radius: 999px; padding: 10px 16px; background: rgba(255,255,255,0.75); color: #1f1c18; font-weight: 600; cursor: pointer;"
}
