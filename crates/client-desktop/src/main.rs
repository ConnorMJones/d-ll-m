use dioxus::prelude::*;
use dllm_client::{
    ClientConfig, ClientSnapshot, ConnectionStatus, DEFAULT_DATABASE_NAME, DEFAULT_URI, DllmClient,
    StoredIdentity,
};
use std::time::Duration;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    dioxus::launch(app);
}

#[derive(Clone, Copy, PartialEq)]
enum CompendiumTab {
    Spells,
    Monsters,
    Items,
}

#[component]
fn app() -> Element {
    let client = use_hook(DllmClient::new);
    let mut snapshot = use_signal(ClientSnapshot::default);
    let mut stored_identities = use_signal(|| client.stored_identities());
    let mut uri = use_signal(|| DEFAULT_URI.to_string());
    let mut database_name = use_signal(|| DEFAULT_DATABASE_NAME.to_string());
    let mut identity_label = use_signal(|| client.preferred_identity_label());
    let mut display_name = use_signal(String::new);
    let mut message_text = use_signal(String::new);
    let mut compendium_tab = use_signal(|| CompendiumTab::Spells);
    let mut search = use_signal(String::new);

    use_future({
        let client = client.clone();
        move || {
            let client = client.clone();
            async move {
                loop {
                    snapshot.set(client.snapshot());
                    stored_identities.set(client.stored_identities());
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
    let is_ready = matches!(snapshot().connection_status, ConnectionStatus::Connected)
        && snapshot().subscription_applied;

    let connect_client = client.clone();
    let disconnect_client = client.clone();
    let set_name_client = client.clone();
    let send_message_client = client.clone();

    let search_lower = search().to_lowercase();

    let spells = snapshot()
        .spells
        .into_iter()
        .filter(|s| search_lower.is_empty() || s.name.to_lowercase().contains(&search_lower))
        .collect::<Vec<_>>();

    let monsters = snapshot()
        .monsters
        .into_iter()
        .filter(|m| search_lower.is_empty() || m.name.to_lowercase().contains(&search_lower))
        .collect::<Vec<_>>();

    let items = snapshot()
        .items
        .into_iter()
        .filter(|i| search_lower.is_empty() || i.name.to_lowercase().contains(&search_lower))
        .collect::<Vec<_>>();

    rsx! {
        div {
            style: "font-family: 'Iosevka', 'IBM Plex Sans', sans-serif; background: linear-gradient(135deg, #f3ecde 0%, #ddd4c0 100%); min-height: 100vh; color: #1f1c18; padding: 24px;",
            div {
                style: "max-width: 1200px; margin: 0 auto; display: flex; flex-direction: column; gap: 20px;",
                h1 { style: "font-size: 32px; margin: 0;", "d-ll-m desktop client" }

                if let Some(error) = snapshot().last_error.clone() {
                    div {
                        style: "background: #f8d7cf; border: 1px solid #d9907a; border-radius: 12px; padding: 12px 14px;",
                        "{error}"
                    }
                }

                // Connection + session row
                div {
                    style: "display: grid; grid-template-columns: 300px 1fr; gap: 20px;",

                    // Left: connection + identity
                    section {
                        style: card_style(),
                        h2 { style: "margin-top: 0;", "Connection" }
                        div { style: "font-size: 14px; color: #5b544b; margin-bottom: 12px;", "Status: {status_text}" }
                        label { style: label_style(), "Server URI" }
                        input {
                            style: input_style(),
                            value: "{uri}",
                            disabled: is_connected,
                            oninput: move |e| uri.set(e.value()),
                        }
                        label { style: label_style(), "Database" }
                        input {
                            style: input_style(),
                            value: "{database_name}",
                            disabled: is_connected,
                            oninput: move |e| database_name.set(e.value()),
                        }
                        label { style: label_style(), "Local identity label" }
                        input {
                            style: input_style(),
                            value: "{identity_label}",
                            disabled: is_connected,
                            oninput: move |e| identity_label.set(e.value()),
                        }
                        if !stored_identities().is_empty() {
                            div { style: "display: flex; flex-wrap: wrap; gap: 8px; margin: 4px 0 10px;",
                                for stored in stored_identities() {
                                    button {
                                        key: "{stored.label}",
                                        style: stored_identity_button_style(identity_label() == stored.label, stored.last_used),
                                        disabled: is_connected,
                                        onclick: {
                                            let label = stored.label.clone();
                                            move |_| identity_label.set(label.clone())
                                        },
                                        "{stored_identity_label(&stored)}"
                                    }
                                }
                            }
                        }
                        div { style: "display: flex; gap: 10px; margin-top: 14px;",
                            button {
                                style: primary_button_style(),
                                disabled: is_connected,
                                onclick: move |_| {
                                    let _ = connect_client.connect(ClientConfig {
                                        uri: uri(),
                                        database_name: database_name(),
                                        identity_label: identity_label(),
                                    });
                                },
                                "Connect"
                            }
                            button {
                                style: secondary_button_style(),
                                disabled: !is_connected,
                                onclick: move |_| { let _ = disconnect_client.disconnect(); },
                                "Disconnect"
                            }
                        }

                        h2 { style: "margin: 22px 0 8px;", "Identity" }
                        if let Some(profile) = snapshot().local_profile.clone() {
                            p { style: "margin: 0 0 8px; font-size: 13px; color: #5b544b;",
                                "Profile: {profile.display_name().to_string()}"
                            }
                        } else {
                            p { style: "margin: 0 0 8px; font-size: 13px; color: #5b544b;",
                                "No profile name set yet."
                            }
                        }
                        input {
                            style: input_style(),
                            value: "{display_name}",
                            disabled: !is_ready,
                            oninput: move |e| display_name.set(e.value()),
                        }
                        button {
                            style: primary_button_style(),
                            disabled: !is_ready,
                            onclick: move |_| { let _ = set_name_client.set_name(display_name()); },
                            "Set name"
                        }
                        if let Some(identity) = snapshot().local_identity.clone() {
                            p { style: "margin: 12px 0 0; font-size: 13px; color: #5b544b;", "Identity: {identity}" }
                        }

                        h2 { style: "margin: 22px 0 8px;", "Users" }
                        div { style: "display: flex; flex-direction: column; gap: 6px;",
                            for user in snapshot().users {
                                div {
                                    key: "{user.identity}",
                                    style: "padding: 8px 10px; border-radius: 10px; background: rgba(96,84,67,0.08); font-size: 13px;",
                                    div { style: "font-weight: 600;", "{user.display_name()}" }
                                    div { style: "color: #5b544b;", if user.online { "online" } else { "offline" } }
                                }
                            }
                            if snapshot().users.is_empty() {
                                div { style: "font-size: 13px; color: #5b544b;", "No users." }
                            }
                        }
                    }

                    // Right: messages
                    section {
                        style: format!("{} display: grid; grid-template-rows: 1fr auto; min-height: 400px;", card_style()),
                        h2 { style: "margin-top: 0;", "Messages" }
                        div {
                            style: "display: flex; flex-direction: column; gap: 10px; overflow-y: auto; padding-right: 8px;",
                            for message in snapshot().messages {
                                article {
                                    key: "{message.id}",
                                    style: "padding: 12px 14px; border-radius: 14px; background: rgba(33,28,24,0.06);",
                                    div { style: "display: flex; justify-content: space-between; font-size: 12px; color: #5b544b;",
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
                        div { style: "display: flex; gap: 10px; align-items: flex-end; margin-top: 14px;",
                            textarea {
                                style: "flex: 1; min-height: 72px; border-radius: 14px; border: 1px solid rgba(50,40,30,0.16); padding: 12px 14px; background: rgba(255,255,255,0.8); resize: vertical;",
                                value: "{message_text}",
                                disabled: !is_ready,
                                oninput: move |e| message_text.set(e.value()),
                            }
                            button {
                                style: primary_button_style(),
                                disabled: !is_ready,
                                onclick: move |_| {
                                    if send_message_client.send_message(message_text()).is_ok() {
                                        message_text.set(String::new());
                                    }
                                },
                                "Send"
                            }
                        }
                    }
                }

                // Compendium
                section {
                    style: card_style(),
                    div { style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 14px;",
                        div { style: "display: flex; gap: 6px;",
                            h2 { style: "margin: 0 16px 0 0;", "Compendium" }
                            {tab_button("Spells", compendium_tab() == CompendiumTab::Spells, move |_| compendium_tab.set(CompendiumTab::Spells))}
                            {tab_button("Monsters", compendium_tab() == CompendiumTab::Monsters, move |_| compendium_tab.set(CompendiumTab::Monsters))}
                            {tab_button("Items", compendium_tab() == CompendiumTab::Items, move |_| compendium_tab.set(CompendiumTab::Items))}
                        }
                        input {
                            style: "border-radius: 10px; border: 1px solid rgba(50,40,30,0.16); padding: 8px 12px; background: rgba(255,255,255,0.8); width: 240px;",
                            placeholder: "Search...",
                            value: "{search}",
                            oninput: move |e| search.set(e.value()),
                        }
                    }

                    match compendium_tab() {
                        CompendiumTab::Spells => rsx! {
                            div { style: compendium_grid_style(5),
                                // Header
                                div { style: compendium_header_style(), "Name" }
                                div { style: compendium_header_style(), "Level" }
                                div { style: compendium_header_style(), "School" }
                                div { style: compendium_header_style(), "Ritual" }
                                div { style: compendium_header_style(), "Conc." }
                                for spell in spells.iter().take(100) {
                                    div { key: "{spell.id}", style: compendium_cell_style(), "{spell.name}" }
                                    div { style: compendium_cell_style(),
                                        { if spell.level == 0 { "Cantrip".to_string() } else { spell.level.to_string() } }
                                    }
                                    div { style: compendium_cell_style(), "{spell.school}" }
                                    div { style: compendium_cell_style(), if spell.ritual { "Yes" } else { "" } }
                                    div { style: compendium_cell_style(), if spell.concentration { "Yes" } else { "" } }
                                }
                                if spells.is_empty() {
                                    div { style: "grid-column: 1/-1; color: #5b544b; font-size: 14px; padding: 12px 0;",
                                        if is_ready { "No spells found." } else { "Connect and subscribe to load data." }
                                    }
                                }
                            }
                            if spells.len() > 100 {
                                div { style: "font-size: 13px; color: #5b544b; margin-top: 10px;",
                                    "Showing 100 of {spells.len()} results. Refine your search to narrow down."
                                }
                            }
                        },
                        CompendiumTab::Monsters => rsx! {
                            div { style: compendium_grid_style(6),
                                div { style: compendium_header_style(), "Name" }
                                div { style: compendium_header_style(), "CR" }
                                div { style: compendium_header_style(), "Type" }
                                div { style: compendium_header_style(), "Size" }
                                div { style: compendium_header_style(), "AC" }
                                div { style: compendium_header_style(), "HP" }
                                for monster in monsters.iter().take(100) {
                                    div { key: "{monster.id}", style: compendium_cell_style(), "{monster.name}" }
                                    div { style: compendium_cell_style(), "{monster.cr}" }
                                    div { style: compendium_cell_style(), "{monster.creature_type}" }
                                    div { style: compendium_cell_style(), "{monster.size}" }
                                    div { style: compendium_cell_style(), "{monster.ac}" }
                                    div { style: compendium_cell_style(), "{monster.hp_average}" }
                                }
                                if monsters.is_empty() {
                                    div { style: "grid-column: 1/-1; color: #5b544b; font-size: 14px; padding: 12px 0;",
                                        if is_ready { "No monsters found." } else { "Connect and subscribe to load data." }
                                    }
                                }
                            }
                            if monsters.len() > 100 {
                                div { style: "font-size: 13px; color: #5b544b; margin-top: 10px;",
                                    "Showing 100 of {monsters.len()} results. Refine your search to narrow down."
                                }
                            }
                        },
                        CompendiumTab::Items => rsx! {
                            div { style: compendium_grid_style(3),
                                div { style: compendium_header_style(), "Name" }
                                div { style: compendium_header_style(), "Type" }
                                div { style: compendium_header_style(), "Rarity" }
                                for item in items.iter().take(100) {
                                    div { key: "{item.id}", style: compendium_cell_style(), "{item.name}" }
                                    div { style: compendium_cell_style(), "{item.item_type}" }
                                    div { style: compendium_cell_style(), "{item.rarity}" }
                                }
                                if items.is_empty() {
                                    div { style: "grid-column: 1/-1; color: #5b544b; font-size: 14px; padding: 12px 0;",
                                        if is_ready { "No items found." } else { "Connect and subscribe to load data." }
                                    }
                                }
                            }
                            if items.len() > 100 {
                                div { style: "font-size: 13px; color: #5b544b; margin-top: 10px;",
                                    "Showing 100 of {items.len()} results. Refine your search to narrow down."
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

fn tab_button(
    label: &'static str,
    active: bool,
    onclick: impl FnMut(MouseEvent) + 'static,
) -> Element {
    let style = if active {
        "border: none; border-radius: 8px; padding: 6px 14px; background: #7f4b2a; color: #fffdf8; font-weight: 600; cursor: pointer; font-size: 14px;"
    } else {
        "border: 1px solid rgba(50,40,30,0.18); border-radius: 8px; padding: 6px 14px; background: rgba(255,255,255,0.6); color: #1f1c18; font-weight: 500; cursor: pointer; font-size: 14px;"
    };
    rsx! {
        button { style, onclick, "{label}" }
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

fn card_style() -> &'static str {
    "background: rgba(255,255,255,0.72); border: 1px solid rgba(50,40,30,0.12); border-radius: 18px; padding: 18px; box-shadow: 0 20px 50px rgba(80,60,30,0.08);"
}

fn input_style() -> &'static str {
    "display: block; width: 100%; box-sizing: border-box; border-radius: 14px; border: 1px solid rgba(50,40,30,0.16); padding: 10px 12px; background: rgba(255,255,255,0.8); margin-bottom: 8px;"
}

fn label_style() -> &'static str {
    "display: block; font-size: 13px; margin-bottom: 4px; color: #5b544b;"
}

fn primary_button_style() -> &'static str {
    "border: none; border-radius: 999px; padding: 10px 16px; background: #7f4b2a; color: #fffdf8; font-weight: 600; cursor: pointer;"
}

fn secondary_button_style() -> &'static str {
    "border: 1px solid rgba(50,40,30,0.18); border-radius: 999px; padding: 10px 16px; background: rgba(255,255,255,0.75); color: #1f1c18; font-weight: 600; cursor: pointer;"
}

fn stored_identity_button_style(active: bool, last_used: bool) -> &'static str {
    match (active, last_used) {
        (true, _) => {
            "border: none; border-radius: 999px; padding: 6px 12px; background: #7f4b2a; color: #fffdf8; font-size: 12px; cursor: pointer;"
        }
        (false, true) => {
            "border: 1px solid rgba(127,75,42,0.4); border-radius: 999px; padding: 6px 12px; background: rgba(127,75,42,0.08); color: #7f4b2a; font-size: 12px; cursor: pointer;"
        }
        (false, false) => {
            "border: 1px solid rgba(50,40,30,0.18); border-radius: 999px; padding: 6px 12px; background: rgba(255,255,255,0.75); color: #1f1c18; font-size: 12px; cursor: pointer;"
        }
    }
}

fn stored_identity_label(stored: &StoredIdentity) -> String {
    if stored.last_used {
        format!("{} (last used)", stored.label)
    } else {
        stored.label.clone()
    }
}

fn compendium_grid_style(columns: u8) -> String {
    let remaining = columns - 1;
    format!(
        "display: grid; grid-template-columns: 2fr repeat({remaining}, 1fr); gap: 1px; background: rgba(50,40,30,0.08); border-radius: 10px; overflow: hidden;"
    )
}

fn compendium_header_style() -> &'static str {
    "padding: 8px 12px; background: rgba(50,40,30,0.06); font-size: 12px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: #5b544b;"
}

fn compendium_cell_style() -> &'static str {
    "padding: 8px 12px; background: rgba(255,255,255,0.7); font-size: 13px;"
}
