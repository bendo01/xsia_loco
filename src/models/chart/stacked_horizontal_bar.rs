use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    show: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emphasis {
    focus: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YAxis {
    r#type: String,
    data: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    name: String,
    r#type: String,
    stack: String,
    label: Label,
    emphasis: Emphasis,
    data: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartData {
    #[serde(rename = "yAxis")]
    y_axis: YAxis,
    series: Vec<Series>,
}
