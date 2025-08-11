use chrono::NaiveDateTime;
use insta::assert_debug_snapshot;
use loco_rs::testing::prelude::*;
use sea_orm::QueryOrder;
use sea_orm::{DeleteResult, Set, entity::prelude::*};
use serial_test::serial;
use uuid::uuid;
use xsia_loco::app::App;
use xsia_loco::models::person::reference::hair_colors::_entities::hair_colors as ReferenceModel;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn test_insert_and_find_hair_color() {
    configure_insta!();

    let boot = boot_test::<App>().await.expect("Failed to boot app");
    let db = &boot.app_context.db;

    // Delete all records
    let _delete_result: DeleteResult = ReferenceModel::Entity::delete_many()
        .exec(db)
        .await
        .expect("Failed to delete all records");

    let datetime_str = "2025-03-24T12:23:43";
    // Parse the string into NaiveDateTime
    let naive_datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M:%S")
        .expect("Failed to parse date");
    let hair_color = ReferenceModel::ActiveModel {
        id: Set(uuid!("0195c81b-d709-7205-b685-690630dbc6a9")),
        code: Set(1),
        alphabet_code: Set("BLK".to_string()),
        name: Set("Black".to_string()),
        created_at: Set(Some(naive_datetime)),
        updated_at: Set(Some(naive_datetime)),
        sync_at: Set(None),
        deleted_at: Set(None),
        created_by: Set(None),
        updated_by: Set(None),
        ..Default::default()
    };

    let inserted = hair_color
        .insert(db)
        .await
        .expect("Failed to insert hair color");

    // Remove timestamp fields or redact them for consistent snapshots
    // let mut snapshot_data = format!("{:?}", inserted);
    // Optionally redact timestamps and UUIDs for consistent snapshots

    assert_debug_snapshot!(inserted);

    // Optional: Also test retrieving the inserted record
    let found = ReferenceModel::Entity::find()
        .filter(ReferenceModel::Column::DeletedAt.is_null())
        .order_by_desc(ReferenceModel::Column::CreatedAt)
        .one(db)
        .await
        .expect("Failed to find hair color")
        .expect("Hair color not found");

    assert_eq!(found.name, "Black");
    assert_eq!(found.alphabet_code, "BLK");
    assert_eq!(found.code, 1);
}
