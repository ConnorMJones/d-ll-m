use dllm_core::dnd5e::ItemRarity;
use dllm_tools::dnd5e::parse_item_rarity_filter;

#[test]
fn parses_valid_item_rarity_filter() {
    let rarity =
        parse_item_rarity_filter(Some("very rare".to_string())).expect("valid rarity should parse");

    assert_eq!(rarity, Some(ItemRarity::VeryRare));
}

#[test]
fn rejects_invalid_item_rarity_filter() {
    let err = parse_item_rarity_filter(Some("shiny".to_string()))
        .expect_err("invalid rarity should be rejected");

    assert!(err.contains("invalid item rarity filter: shiny"));
}

#[test]
fn accepts_missing_item_rarity_filter() {
    let rarity = parse_item_rarity_filter(None).expect("missing rarity should be allowed");

    assert_eq!(rarity, None);
}
