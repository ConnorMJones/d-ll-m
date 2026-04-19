use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Entry {
    Text(String),
    List(ListEntry),
    Entries(EntriesBlock),
    Table(TableEntry),
    Other(serde_json::Value),
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListEntry {
    #[serde(rename = "type")]
    pub _entry_type: ListType,
    pub items: Vec<Entry>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ListType {
    List,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EntriesBlock {
    #[serde(rename = "type")]
    pub _entry_type: EntriesType,
    #[serde(default)]
    pub name: Option<String>,
    pub entries: Vec<Entry>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EntriesType {
    Entries,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TableEntry {
    #[serde(rename = "type")]
    pub _entry_type: TableType,
    #[serde(default)]
    pub caption: Option<String>,
    #[serde(default, rename = "colLabels")]
    pub col_labels: Vec<String>,
    #[serde(default)]
    pub rows: Vec<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TableType {
    Table,
}

impl Entry {
    pub fn to_text(&self) -> (String, usize) {
        match self {
            Self::Text(s) => (s.clone(), 0),
            Self::List(list) => {
                let mut unsupported = 0;
                let lines = list
                    .items
                    .iter()
                    .filter_map(|e| {
                        let (text, count) = e.to_text();
                        unsupported += count;
                        (!text.is_empty()).then_some(format!("• {}", text))
                    })
                    .collect::<Vec<_>>();
                (lines.join("\n"), unsupported)
            }
            Self::Entries(block) => {
                let mut unsupported = 0;
                let header = block
                    .name
                    .as_ref()
                    .map(|n| format!("{}:\n", n))
                    .unwrap_or_default();
                let body = block
                    .entries
                    .iter()
                    .filter_map(|e| {
                        let (text, count) = e.to_text();
                        unsupported += count;
                        (!text.is_empty()).then_some(text)
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n");
                (format!("{}{}", header, body), unsupported)
            }
            Self::Table(table) => {
                let caption = table
                    .caption
                    .as_ref()
                    .map(|c| format!("{}\n", c))
                    .unwrap_or_default();
                let header = table.col_labels.join(" | ");
                let rows = table
                    .rows
                    .iter()
                    .map(|r| r.join(" | "))
                    .collect::<Vec<_>>()
                    .join("\n");
                (format!("{}{}\n{}", caption, header, rows), 0)
            }
            Self::Other(value) => {
                let kind = value
                    .as_object()
                    .and_then(|obj| obj.get("type"))
                    .and_then(|kind| kind.as_str())
                    .unwrap_or("unknown");
                (format!("[unsupported {} entry]", kind), 1)
            }
        }
    }
}

pub fn entries_to_string(entries: &[Entry]) -> (String, usize) {
    let mut unsupported = 0;
    let text = entries
        .iter()
        .filter_map(|e| {
            let (text, count) = e.to_text();
            unsupported += count;
            (!text.is_empty()).then_some(text)
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    (text, unsupported)
}
