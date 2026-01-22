use chrono::Datelike;
use tidlers::TidalClient;

pub async fn execute(
    tidal: &mut TidalClient,
    year: Option<i32>,
    month: Option<u32>,
) -> eyre::Result<()> {
    let (year, month) = validate_date(year, month)?;

    let timeline = tidal.get_activity_timeline().await?;
    println!("Timeline:\n{:#?}\n", timeline);

    let top_artists = tidal.get_activity_top_artists(year, month).await?;
    println!("Top Artists:\n{:#?}", top_artists);

    Ok(())
}

fn validate_date(year: Option<i32>, month: Option<u32>) -> eyre::Result<(i32, u32)> {
    let now = chrono::Utc::now();
    let year = year.unwrap_or_else(|| now.year());
    let month = month.unwrap_or_else(|| now.month());

    if !(1..=12).contains(&month) {
        return Err(eyre::Report::msg(format!(
            "Invalid month: {}. Must be between 1 and 12",
            month
        )));
    }

    Ok((year, month))
}
