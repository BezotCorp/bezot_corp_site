use common::PageBlock;

use crate::page_block_field::{PageBlockField, flag_mut, number_field_mut, text_field_mut};

#[test]
fn maps_hero_text_fields() {
    let mut block = PageBlock::Hero {
        title: "Titre".to_string(),
        subtitle: "Sous-titre".to_string(),
    };

    assert_eq!(
        text_field_mut(&mut block, PageBlockField::Title).map(|value| value.as_str()),
        Some("Titre")
    );
    assert_eq!(
        text_field_mut(&mut block, PageBlockField::Subtitle).map(|value| value.as_str()),
        Some("Sous-titre")
    );
    assert!(text_field_mut(&mut block, PageBlockField::Email).is_none());
}

#[test]
fn maps_post_list_number_and_flag_fields() {
    let mut block = PageBlock::PostList {
        limit: Some(10),
        page: None,
        show_description: false,
        show_author: true,
        show_date: false,
    };

    assert_eq!(
        number_field_mut(&mut block, PageBlockField::Limit).copied(),
        Some(Some(10))
    );
    assert_eq!(
        number_field_mut(&mut block, PageBlockField::Page).copied(),
        Some(None)
    );
    assert!(number_field_mut(&mut block, PageBlockField::Title).is_none());

    assert_eq!(
        flag_mut(&mut block, PageBlockField::ShowAuthor).copied(),
        Some(true)
    );
    assert_eq!(
        flag_mut(&mut block, PageBlockField::ShowDescription).copied(),
        Some(false)
    );
    assert!(flag_mut(&mut block, PageBlockField::Limit).is_none());
}

#[test]
fn writing_through_the_mapped_reference_mutates_the_block() {
    let mut block = PageBlock::Paragraph {
        text: "avant".to_string(),
    };

    if let Some(text) = text_field_mut(&mut block, PageBlockField::Text) {
        *text = "après".to_string();
    }

    assert_eq!(
        block,
        PageBlock::Paragraph {
            text: "après".to_string()
        }
    );
}
