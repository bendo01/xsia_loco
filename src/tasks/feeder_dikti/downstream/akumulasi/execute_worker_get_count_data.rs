use loco_rs::prelude::*;
use crate::workers::feeder_dikti::downstream::akumulasi::get_count_data::{GetCountDataWorker, GetCountDataWorkerWorkerArgs};

pub struct ExecuteWorkerGetCountData;
#[async_trait]
impl Task for ExecuteWorkerGetCountData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ExecuteWorkerGetCountData".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task ExecuteWorkerGetCountData generated");
        // let _ = GetCountDataWorker::perform_later(&app_context, GetCountDataWorkerWorkerArgs{
        //     table_column_name: "FA0003GetCountMahasiswa".to_string(),
        //     action_name: "GetCountMahasiswa".to_string(),
        // }).await;
        // Define your table_name and action_name combinations
        let data_combinations = vec![
            ("FA0001GetCountAktivitasMahasiswa", "GetCountAktivitasMahasiswa"),
            ("FA0002GetCountPrestasiMahasiswa", "GetCountPrestasiMahasiswa"),
            ("FA0003GetCountMahasiswa", "GetCountMahasiswa"),
            ("FA0004GetCountRiwayatPendidikanMahasiswa", "GetCountRiwayatPendidikanMahasiswa"),
            ("FA0005GetCountNilaiTransferPendidikanMahasiswa", "GetCountNilaiTransferPendidikanMahasiswa"),
            ("FA0006GetCountRiwayatNilaiMahasiswa", "GetCountRiwayatNilaiMahasiswa"),
            ("FA0007GetCountDosen", "GetCountDosen"),
            ("FA0008GetCountPenugasanSemuaDosen", "GetCountPenugasanSemuaDosen"),
            ("FA0009GetCountAktivitasMengajarDosen", "GetCountAktivitasMengajarDosen"),
            ("FA0010GetCountSkalaNilaiProdi", "GetCountSkalaNilaiProdi"),
            ("FA0011GetCountPeriodePerkuliahan", "GetCountPeriodePerkuliahan"),
            ("FA0012GetCountDosenPembimbing", "GetCountDosenPembimbing"),
            ("FA0013GetCountKelasKuliah", "GetCountKelasKuliah"),
            ("FA0014GetCountKurikulum", "GetCountKurikulum"),
            ("FA0015GetCountMataKuliah", "GetCountMataKuliah"),
            ("FA0016GetCountMatkulKurikulum", "GetCountMatkulKurikulum"),
            ("FA0017GetCountNilaiPerkuliahanKelas", "GetCountNilaiPerkuliahanKelas"),
            ("FA0018GetCountSubstansiKuliah", "GetCountSubstansiKuliah"),
            ("FA0019GetCountPerguruanTinggi", "GetCountPerguruanTinggi"),
            ("FA0020GetCountProdi", "GetCountProdi"),
            ("FA0021GetCountDosenPengajarKelasKuliah", "GetCountDosenPengajarKelasKuliah"),
            ("FA0022GetCountMahasiswaLulusDO", "GetCountMahasiswaLulusDO"),
            ("FA0023GetCountPesertaKelasKuliah", "GetCountPesertaKelasKuliah"),
            ("FA0024GetCountPerkuliahanMahasiswa", "GetCountPerkuliahanMahasiswa"),
            ("FA0025GetCountMahasiswaBimbinganDosen", "GetCountMahasiswaBimbinganDosen"),
            ("FA0026GetCountKonversiKampusMerdeka", "GetCountKonversiKampusMerdeka"),
            ("FA0027GetCountRencanaPembelajaran", "GetCountRencanaPembelajaran"),
            ("FA0028GetCountRencanaEvaluasi", "GetCountRencanaEvaluasi"),
            ("FA0029GetCountBiodataMahasiswa", "GetCountBiodataMahasiswa"),
            // Add more combinations as needed
        ];

        println!("Enqueueing {} GetCountDataWorker jobs", data_combinations.len());

        for (table_name, action_name) in data_combinations {
            let worker_args = GetCountDataWorkerWorkerArgs {
                table_column_name: table_name.to_string(),
                action_name: action_name.to_string(),
            };

            match GetCountDataWorker::perform_later(app_context, worker_args).await {
                Ok(_) => {
                    println!("✅ Enqueued worker for table: {} with action: {}", table_name, action_name);
                }
                Err(err) => {
                    eprintln!("❌ Failed to enqueue worker for table: {} with action: {}. Error: {}",
                                table_name, action_name, err);
                    // Continue with other combinations or return error based on your needs
                }
            }
        }

        println!("Task ExecuteWorkerGetCountData completed - all workers enqueued");
        Ok(())
    }
}
