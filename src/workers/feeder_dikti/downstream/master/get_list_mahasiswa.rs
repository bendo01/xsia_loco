use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use chrono::Local;
use chrono::NaiveDate;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid; // Removed unused `uuid` macro import

use crate::models::feeder::master::mahasiswa::_entities::mahasiswa as FeederMasterMahasiswa;
use crate::tasks::feeder_dikti::downstream::request_data_pagination::{
    InputRequestData, RequestData,
};

// --- Deserializers ---

fn deserialize_date_opt<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Accept null or "DD-MM-YYYY"
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if !s.trim().is_empty() => NaiveDate::parse_from_str(&s, "%d-%m-%Y")
            .map(Some)
            .map_err(serde::de::Error::custom),
        _ => Ok(None),
    }
}

fn de_opt_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde_json::Value;
    let v = Option::<Value>::deserialize(deserializer)?;
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => n
            .as_u64()
            .map(|x| Some(x as u32))
            .ok_or_else(|| serde::de::Error::custom("expected unsigned integer")),
        Some(Value::String(s)) => {
            if s.trim().is_empty() {
                Ok(None)
            } else {
                s.parse::<u32>()
                    .map(Some)
                    .map_err(|e| serde::de::Error::custom(format!("invalid u32: {e}")))
            }
        }
        other => Err(serde::de::Error::custom(format!(
            "unexpected type for u32: {other:?}"
        ))),
    }
}

fn de_opt_f32<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde_json::Value;
    let v = Option::<Value>::deserialize(deserializer)?;
    match v {
        None => Ok(None),
        Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => n
            .as_f64()
            .map(|x| Some(x as f32))
            .ok_or_else(|| serde::de::Error::custom("expected float")),
        Some(Value::String(s)) => {
            if s.trim().is_empty() {
                Ok(None)
            } else {
                s.parse::<f32>()
                    .map(Some)
                    .map_err(|e| serde::de::Error::custom(format!("invalid f32: {e}")))
            }
        }
        other => Err(serde::de::Error::custom(format!(
            "unexpected type for f32: {other:?}"
        ))),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInput {
    pub nama_mahasiswa: String,
    pub jenis_kelamin: Option<String>,

    #[serde(deserialize_with = "deserialize_date_opt")]
    pub tanggal_lahir: Option<NaiveDate>,

    pub id_perguruan_tinggi: Option<String>,
    pub nipd: Option<String>,

    #[serde(deserialize_with = "de_opt_f32")]
    pub ipk: Option<f32>,

    #[serde(deserialize_with = "de_opt_u32")]
    pub total_sks: Option<u32>,

    pub id_sms: Option<String>,
    pub id_mahasiswa: String,

    #[serde(deserialize_with = "de_opt_u32")]
    pub id_agama: Option<u32>,   // FIXED

    pub nama_agama: Option<String>,
    pub id_prodi: Option<String>,
    pub nama_program_studi: Option<String>,

    #[serde(deserialize_with = "de_opt_u32")]
    pub id_status_mahasiswa: Option<u32>,   // FIXED

    pub nama_status_mahasiswa: Option<String>,
    pub nim: Option<String>,
    pub id_periode: Option<String>,
    pub nama_periode_masuk: Option<String>,
    pub id_registrasi_mahasiswa: Option<String>,
    pub id_periode_keluar: Option<String>,

    #[serde(deserialize_with = "deserialize_date_opt")]
    pub tanggal_keluar: Option<NaiveDate>,

    #[serde(deserialize_with = "deserialize_date_opt")]
    pub last_update: Option<NaiveDate>,

    #[serde(deserialize_with = "deserialize_date_opt")]
    pub tgl_create: Option<NaiveDate>,

    pub status_sync: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelData;

impl ModelData {
    // pub async fn upsert(ctx: &AppContext, input: ModelInput) -> Result<(), Error> {}
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
    ///
    /// This function is called when registering the worker with the queue system.
    ///
    /// # Parameters
    /// * `ctx` - The application context containing shared resources
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    /// Returns the class name of the worker.
    ///
    /// This name is used when enqueueing jobs and identifying the worker in logs.
    /// The implementation returns the struct name as a string.
    fn class_name() -> String {
        "GetListMahasiswa".to_string()
    }

    /// Returns tags associated with this worker.
    ///
    /// Tags can be used to filter which workers run during startup.
    /// The default implementation returns an empty vector (no tags).
    fn tags() -> Vec<String> {
        Vec::new()
    }

    /// Performs the actual work when a job is processed.
    ///
    /// This is the main function that contains the worker's logic.
    /// It gets executed when a job is dequeued from the job queue.
    ///
    /// # Returns
    /// * `Result<()>` - Ok if the job completed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================GetListMahasiswa=======================");
        // TODO: Some actual work goes here...
        println!("ARGS Data {:#?}", args);

        let req_result = RequestData::get::<ModelInput>(
            &self.ctx,
            InputRequestData {
                act: args.act,          // String
                filter: args.filter,    // Option<String>
                order: args.order,      // Option<String>
                limit: args.limit,      // Option<i32>
                offset: args.offset,    // Option<i32>
            },
        ).await;

        if let Ok(response) = req_result {
            match response.data {
                Some(data_vec) if !data_vec.is_empty() => {
                    println!("Processing {} items", data_vec.len());
                    // ...
                }
                Some(_) => println!("Received empty data vector"),
                None => println!("No data in response"),
            }
        } else {
            println!("Failed to get data: {:#?}", req_result);
        }

        Ok(())
    }
}
