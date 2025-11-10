#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityTimeline {
    timeline: Vec<ActivityTimelineItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityTimelineItem {
    year: i32,
    month: u32,
    title: String,
}
