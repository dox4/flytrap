pub trait Schema {
    fn table_name() -> &'static str;
    fn column_names() -> &'static [&'static str];
    // fn delete_by_ids(ids: &[String]) -> String;
    // fn update_by_id(id: String) -> String;
    // fn insert(self) -> String;
    // fn select_by_id(id: String) -> String;
}
