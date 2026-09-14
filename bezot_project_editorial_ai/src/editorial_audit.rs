use std::{io, path::Path};

use crate::content_loader::load_content_entries;

pub fn audit_editorial_content(project_root: &Path) -> io::Result<()> {
    let entries = load_content_entries(project_root)?;
    let editorial_posts = entries
        .iter()
        .filter(|entry| entry.kind == "post" && entry.id.starts_with("ai-editorial"))
        .collect::<Vec<_>>();

    println!("editorial posts: {}", editorial_posts.len());

    for post in editorial_posts {
        println!("{} [{}]", post.id, post.status);
        for locale in &post.locales {
            println!("  {} {}", locale.locale, locale.title);
        }
    }

    Ok(())
}
