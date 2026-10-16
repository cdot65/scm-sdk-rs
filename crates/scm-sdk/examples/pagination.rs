use scm_sdk::{
    RequestOptions, Session,
    pagination::{CollectOptions, Page, Pages},
};
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = RequestOptions::with_timeout(Duration::from_secs(5))?;
    let pages = Pages::new(
        0u64,
        options,
        Session::new(),
        100,
        |cursor, _options| async move {
            Ok(Page {
                items: vec![cursor, cursor + 1],
                next: if cursor < 4 { Some(cursor + 2) } else { None },
            })
        },
    )?;
    let collection = pages
        .collect(CollectOptions::default().with_max_items(3)?)
        .await?;
    println!(
        "items={} complete={} resumable={}",
        collection.items.len(),
        collection.is_complete(),
        collection.resume.is_some()
    );
    Ok(())
}
