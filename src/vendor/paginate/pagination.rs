use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PaginateResult {
    /// `Search` is search phrase
    pub search: Option<String>,
    /// `sort_by` is the name of the column to sort by
    pub sort_by: Option<String>,
    /// `column` is the name of the column to search in
    pub column: Option<String>,
    /// `sort_dir` is the direction to sort in ("ASC" or "DESC")
    pub sort_dir: Option<String>,
    /// `page` is the page that you want to see
    pub page: u64,
    /// `per_page` is items the page that you want to see
    pub per_page: u64,
    /// `total_page` is number of page
    pub total_page: u64,
    /// `last_page` is number of last page
    pub last_page: u64,
    /// `total_data` in database
    pub total_data: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PaginateInput {
    /// `search` is search phrase
    pub search: Option<String>,
    /// `sort_by` is the name of the column to sort by
    pub sort_by: Option<String>,
    /// `column` is the name of the column to search in
    pub column: Option<String>,
    /// `sort_dir` is the direction to sort in ("ASC" or "DESC")
    pub sort_dir: Option<String>,
    /// `page` is the page that you want to see
    pub page: u64,
    /// `per_page` is items the page that you want to see
    pub per_page: u64,
}
