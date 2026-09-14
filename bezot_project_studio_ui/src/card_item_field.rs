use common::CardItem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CardItemField {
    Title,
    Text,
}

pub(crate) fn field_mut(item: &mut CardItem, field: CardItemField) -> &mut String {
    match field {
        CardItemField::Title => &mut item.title,
        CardItemField::Text => &mut item.text,
    }
}
