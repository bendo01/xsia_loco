use chrono::Local;
use colored::Colorize;
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::feeder::master::komponen_evaluasi_kelas::_entities::komponen_evaluasi_kelas as FeederMasterKomponenEvaluasiKelas;
use crate::models::feeder::master::komponen_evaluasi_kelas::feeder_model::ModelInput as FeederModel;
use crate::tasks::feeder_dikti::downstream::request_only_data::{InputRequestData, RequestData};

// Configuration constants
const WORKER_NAME: &str = "GetListKomponenEvaluasiKelas";
const DEFAULT_LIMIT: i32 = 100;

#[derive(Debug)]
pub enum UpsertError {
    MissingRequiredField(String),
    DatabaseError(String),
    ValidationError(String),
}

impl From<UpsertError> for Error {
    fn from(err: UpsertError) -> Self {
        match err {
            UpsertError::MissingRequiredField(field) => {
                Error::Message(format!("Missing required field: {}", field))
            }
            UpsertError::DatabaseError(msg) => Error::Message(format!("Database error: {}", msg)),
            UpsertError::ValidationError(msg) => {
                Error::Message(format!("Validation error: {}", msg))
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputData;

impl InputData {
    /// Validate required fields from FeederModel
    fn validate_input(input: &FeederModel) -> Result<(Uuid, Uuid, i32), UpsertError> {
        let id_komponen_evaluasi = input
            .id_komponen_evaluasi
            .ok_or_else(|| UpsertError::MissingRequiredField("id_komponen_evaluasi".to_string()))?;

        let id_kelas_kuliah = input
            .id_kelas_kuliah
            .ok_or_else(|| UpsertError::MissingRequiredField("id_kelas_kuliah".to_string()))?;

        let id_jenis_evaluasi = input
            .id_jenis_evaluasi
            .ok_or_else(|| UpsertError::MissingRequiredField("id_jenis_evaluasi".to_string()))?;

        Ok((id_komponen_evaluasi, id_kelas_kuliah, id_jenis_evaluasi))
    }

    /// Find existing record by composite key
    async fn find_existing_record<C>(
        db: &C,
        id_komponen_evaluasi: Uuid,
        id_kelas_kuliah: Uuid,
        id_jenis_evaluasi: i32,
    ) -> Result<Option<FeederMasterKomponenEvaluasiKelas::Model>, UpsertError>
    where
        C: ConnectionTrait,
    {
        FeederMasterKomponenEvaluasiKelas::Entity::find()
            .filter(FeederMasterKomponenEvaluasiKelas::Column::DeletedAt.is_null())
            .filter(
                FeederMasterKomponenEvaluasiKelas::Column::IdKomponenEvaluasi
                    .eq(id_komponen_evaluasi),
            )
            .filter(FeederMasterKomponenEvaluasiKelas::Column::IdKelasKuliah.eq(id_kelas_kuliah))
            .filter(
                FeederMasterKomponenEvaluasiKelas::Column::IdJenisEvaluasi.eq(id_jenis_evaluasi),
            )
            .one(db)
            .await
            .map_err(|e| UpsertError::DatabaseError(e.to_string()))
    }

    /// Create ActiveModel from FeederModel input
    fn create_active_model(
        input: &FeederModel,
        id_komponen_evaluasi: Uuid,
        id_kelas_kuliah: Uuid,
        id_jenis_evaluasi: i32,
        existing_id: Option<Uuid>,
    ) -> FeederMasterKomponenEvaluasiKelas::ActiveModel {
        let now = Local::now().naive_local();
        let today = now.date();

        let mut active_model = FeederMasterKomponenEvaluasiKelas::ActiveModel {
            id_komponen_evaluasi: Set(id_komponen_evaluasi),
            id_kelas_kuliah: Set(id_kelas_kuliah),
            id_jenis_evaluasi: Set(id_jenis_evaluasi),
            nomor_urut: Set(input.nomor_urut.unwrap_or_default()),
            bobot_evaluasi: Set(input
                .bobot_evaluasi
                .map(|v| v.to_string())
                .unwrap_or_default()),
            nama: Set(input.nama.clone()),
            nama_inggris: Set(input.nama_inggris.clone()),
            last_update: Set(input.last_update.unwrap_or(today)),
            tgl_create: Set(input.tgl_create.unwrap_or(today)),
            ..Default::default()
        };

        if let Some(id) = existing_id {
            // Updating existing record
            active_model.id = Set(id);
            active_model.updated_at = Set(Some(now));
            active_model.sync_at = Set(Some(now));
        } else {
            // Creating new record
            let uuidv7_string = uuid7::uuid7().to_string();
            let pk_id = Uuid::parse_str(&uuidv7_string).expect("Invalid UUID format");
            active_model.id = Set(pk_id);
            active_model.created_at = Set(Some(now));
            active_model.updated_at = Set(Some(now));
            active_model.sync_at = Set(Some(now));
        }

        active_model
    }

    /// Perform upsert operation with transaction safety
    pub async fn upsert(ctx: &AppContext, input: FeederModel) -> Result<(), Error> {
        // Validate input
        let (id_komponen_evaluasi, id_kelas_kuliah, id_jenis_evaluasi) =
            Self::validate_input(&input)?;

        // Use transaction for atomic operation
        let txn = ctx
            .db
            .begin()
            .await
            .map_err(|e| UpsertError::DatabaseError(e.to_string()))?;

        // Check if record exists
        let existing_record = Self::find_existing_record(
            &txn,
            id_komponen_evaluasi,
            id_kelas_kuliah,
            id_jenis_evaluasi,
        )
        .await?;

        let (operation, result) = match existing_record {
            Some(existing) => {
                // Update existing record
                let active_model = Self::create_active_model(
                    &input,
                    id_komponen_evaluasi,
                    id_kelas_kuliah,
                    id_jenis_evaluasi,
                    Some(existing.id),
                );

                let update_result = active_model.update(&txn).await;
                ("updated", update_result)
            }
            None => {
                // Insert new record
                let active_model = Self::create_active_model(
                    &input,
                    id_komponen_evaluasi,
                    id_kelas_kuliah,
                    id_jenis_evaluasi,
                    None,
                );

                let insert_result = active_model.insert(&txn).await;
                ("inserted", insert_result)
            }
        };

        // Handle the result
        match result {
            Ok(_) => {
                // Commit transaction
                txn.commit()
                    .await
                    .map_err(|e| UpsertError::DatabaseError(e.to_string()))?;

                // Silently succeed to reduce noise
                Ok(())
            }
            Err(err) => {
                // Transaction will auto-rollback on drop
                Err(
                    UpsertError::DatabaseError(format!("Failed to {} record: {}", operation, err))
                        .into(),
                )
            }
        }
    }
}

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub act: String,
    pub filter: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    /// Creates a new instance of the Worker with the given application context.
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    /// Returns the class name of the worker.
    fn class_name() -> String {
        WORKER_NAME.to_string()
    }

    /// Returns tags associated with this worker.
    fn tags() -> Vec<String> {
        vec!["feeder_dikti".to_string(), "master_data".to_string()]
    }

    /// Performs the actual work when a job is processed.
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("================={}=======================", WORKER_NAME);

        let req_result = RequestData::get::<FeederModel>(
            &self.ctx,
            InputRequestData {
                act: args.act,
                filter: args.filter,
                order: args.order,
                limit: Some(args.limit.unwrap_or(DEFAULT_LIMIT)),
                offset: args.offset,
            },
        )
        .await;

        if let Ok(response) = req_result {
            match response.data {
                Some(data_vec) if !data_vec.is_empty() => {
                    println!("📦 Processing {} items", data_vec.len());
                    let mut success_count = 0;
                    let mut error_count = 0;

                    for item in data_vec {
                        match InputData::upsert(&self.ctx, item).await {
                            Ok(_) => success_count += 1,
                            Err(e) => {
                                error_count += 1;
                                eprintln!("❌ Failed to upsert item: {}", e);
                            }
                        }
                    }

                    println!(
                        "{}",
                        format!(
                            "✅ Completed: {} successful, {} errors",
                            success_count, error_count
                        )
                        .green()
                    );
                }
                Some(_) => println!("📭 Received empty data vector"),
                None => println!("📭 No data in response"),
            }
        } else {
            eprintln!("❌ Failed to get data: {:#?}", req_result);
        }

        Ok(())
    }
}
