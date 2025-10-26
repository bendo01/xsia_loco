use async_trait::async_trait;
use loco_openapi::prelude::*;
use loco_rs::{
    Result,
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{BootResult, StartMode, create_app},
    config::Config,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
};
use migration::Migrator;
use std::path::Path;

#[allow(unused_imports)]
use crate::{
    controllers, initializers, models::auth::users::_entities::users,
    models::person::reference::hair_colors::_entities::hair_colors as PersonReferenceHairColor,
    tasks, workers,
};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![
            Box::new(initializers::websocket::WebSocketInitializer),
            Box::new(initializers::view_engine::ViewEngineInitializer),
            Box::new(loco_openapi::OpenapiInitializerWithSetup::new(
                |ctx| {
                    #[derive(OpenApi)]
                    #[openapi(
                            modifiers(&SecurityAddon),
                            info(
                                title = "xSIA API",
                                description = "this xSIA API Documentation"
                            )
                        )]
                    struct ApiDoc;
                    set_jwt_location(ctx.into());

                    ApiDoc::openapi()
                },
                // When using automatic schema collection only
                None,
                // When using manual schema collection
                // Manual schema collection can also be used at the same time as automatic schema collection
                // Some(vec![controllers::album::api_routes()]),
            )),
        ])
    }

    #[allow(clippy::too_many_lines)]
    fn routes(ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::academic::campaign::reference::attend_types::routes(ctx))
            .add_route(controllers::academic::campaign::reference::calendar_categories::routes(ctx))
            .add_route(controllers::academic::campaign::reference::encounter_categories::routes(ctx))
            .add_route(controllers::academic::campaign::reference::encounter_types::routes(ctx))
            .add_route(controllers::academic::campaign::reference::evaluation_types::routes(ctx))
            .add_route(controllers::academic::campaign::reference::implementations::routes(ctx))
            .add_route(controllers::academic::campaign::reference::scopes::routes(ctx))
            .add_route(controllers::academic::campaign::reference::substances::routes(ctx))
            .add_route(controllers::academic::campaign::transaction::activities::routes())
            .add_route(controllers::academic::campaign::transaction::calendar_details::routes())
            .add_route(controllers::academic::campaign::transaction::calendars::routes())
            .add_route(controllers::academic::campaign::transaction::class_codes::routes())
            .add_route(controllers::academic::campaign::transaction::grades::routes())
            .add_route(controllers::academic::campaign::transaction::schedules::routes())
            .add_route(controllers::academic::campaign::transaction::teach_decrees::routes())
            .add_route(controllers::academic::campaign::transaction::teach_lecturers::routes())
            .add_route(controllers::academic::campaign::transaction::teaches::routes())
            .add_route(controllers::academic::candidate::master::candidates::routes(ctx))
            .add_route(controllers::academic::candidate::master::exam_classes::routes())
            .add_route(controllers::academic::candidate::reference::document_types::routes(ctx))
            .add_route(controllers::academic::candidate::reference::phases::routes(ctx))
            .add_route(controllers::academic::candidate::reference::registration_types::routes(ctx))
            .add_route(controllers::academic::candidate::transaction::documents::routes())
            .add_route(controllers::academic::candidate::transaction::exams::routes())
            .add_route(controllers::academic::candidate::transaction::unit_choices::routes())
            .add_route(controllers::academic::course::master::concentrations::routes())
            .add_route(controllers::academic::course::master::course_evaluation_plannings::routes())
            .add_route(controllers::academic::course::master::course_learn_plannings::routes())
            .add_route(controllers::academic::course::master::courses::routes())
            .add_route(controllers::academic::course::master::curriculum_details::routes())
            .add_route(controllers::academic::course::master::curriculums::routes())
            .add_route(controllers::academic::course::reference::competences::routes(ctx))
            .add_route(controllers::academic::course::reference::course_evaluation_bases::routes(ctx))
            .add_route(controllers::academic::course::reference::curriculum_types::routes(ctx))
            .add_route(controllers::academic::course::reference::groups::routes(ctx))
            .add_route(controllers::academic::course::reference::semesters::routes(ctx))
            .add_route(controllers::academic::course::reference::varieties::routes(ctx))
            .add_route(controllers::academic::general::reference::academic_year_categories::routes(ctx))
            .add_route(controllers::academic::general::reference::academic_years::routes())
            .add_route(controllers::academic::lecturer::master::lecturers::routes(ctx))
            .add_route(controllers::academic::lecturer::reference::contracts::routes(ctx))
            .add_route(controllers::academic::lecturer::reference::groups::routes(ctx))
            .add_route(controllers::academic::lecturer::reference::ranks::routes(ctx))
            .add_route(controllers::academic::lecturer::reference::statuses::routes(ctx))
            .add_route(controllers::academic::lecturer::transaction::academic_groups::routes())
            .add_route(controllers::academic::lecturer::transaction::academic_ranks::routes())
            .add_route(controllers::academic::lecturer::transaction::homebases::routes())
            .add_route(controllers::academic::prior_learning_recognition::transaction::recognitions::routes(ctx))
            .add_route(controllers::academic::student::adviser::counsellors::routes())
            .add_route(controllers::academic::student::adviser::decrees::routes())
            .add_route(controllers::academic::student::campaign::activities::routes(ctx))
            .add_route(controllers::academic::student::campaign::convertions::routes())
            .add_route(controllers::academic::student::campaign::detail_activities::routes())
            .add_route(controllers::academic::student::campaign::evaluation_components::routes())
            .add_route(controllers::academic::student::final_assignment::reference::adviser_categories::routes(ctx))
            .add_route(controllers::academic::student::final_assignment::reference::approval_types::routes(ctx))
            .add_route(controllers::academic::student::final_assignment::reference::categories::routes(ctx))
            .add_route(controllers::academic::student::final_assignment::reference::requirements::routes(ctx))
            .add_route(controllers::academic::student::final_assignment::reference::stages::routes(ctx))
            .add_route(controllers::academic::student::final_assignment::reference::varieties::routes(ctx))
            .add_route(controllers::academic::student::final_assignment::transaction::advisers::routes())
            .add_route(controllers::academic::student::final_assignment::transaction::evaluation_summaries::routes())
            .add_route(controllers::academic::student::final_assignment::transaction::evaluation_types::routes())
            .add_route(controllers::academic::student::final_assignment::transaction::final_assignment_decrees::routes())
            .add_route(controllers::academic::student::final_assignment::transaction::prerequisites::routes())
            .add_route(controllers::academic::student::final_assignment::transaction::schedules::routes())
            .add_route(controllers::academic::student::final_assignment::transaction::submissions::routes())
            .add_route(controllers::academic::student::master::students::routes(ctx))
            .add_route(controllers::academic::student::master::images::routes())
            .add_route(controllers::academic::student::reference::finances::routes(ctx))
            .add_route(controllers::academic::student::reference::registrations::routes(ctx))
            .add_route(controllers::academic::student::reference::resign_statuses::routes(ctx))
            .add_route(controllers::academic::student::reference::selection_types::routes(ctx))
            .add_route(controllers::academic::student::reference::statuses::routes(ctx))
            .add_route(controllers::academic::survey::master::answers::routes())
            .add_route(controllers::academic::survey::master::bundles::routes())
            .add_route(controllers::academic::survey::master::questions::routes())
            .add_route(controllers::academic::survey::reference::bundle_categories::routes(ctx))
            .add_route(controllers::academic::survey::reference::question_varieties::routes(ctx))
            .add_route(controllers::academic::survey::transaction::conducts::routes())
            .add_route(controllers::academic::survey::transaction::responds::routes())
            .add_route(controllers::auth::permissions::routes(ctx))
            .add_route(controllers::auth::roles::routes(ctx))
            .add_route(controllers::auth::users::routes())
            .add_route(controllers::building::master::buildings::routes())
            .add_route(controllers::building::master::rooms::routes())
            .add_route(controllers::building::reference::categories::routes(ctx))
            .add_route(controllers::building::reference::conditions::routes(ctx))
            .add_route(controllers::building::reference::room_types::routes(ctx))
            .add_route(controllers::building::reference::varieties::routes(ctx))
            .add_route(controllers::contact::master::electronic_mails::routes())
            .add_route(controllers::contact::master::phones::routes())
            .add_route(controllers::contact::master::residences::routes(ctx))
            .add_route(controllers::contact::master::websites::routes())
            .add_route(controllers::contact::reference::electronic_mail_types::routes(ctx))
            .add_route(controllers::contact::reference::phone_types::routes(ctx))
            .add_route(controllers::contact::reference::residence_types::routes(ctx))
            .add_route(controllers::contact::reference::website_types::routes(ctx))
            .add_route(controllers::experiment::example::routes(ctx))
            .add_route(controllers::institution::master::employees::routes(ctx))
            .add_route(controllers::institution::master::institutions::routes(ctx))
            .add_route(controllers::institution::master::staffes::routes(ctx))
            .add_route(controllers::institution::master::units::routes(ctx))
            .add_route(controllers::institution::reference::categories::routes(ctx))
            .add_route(controllers::institution::reference::position_types::routes(ctx))
            .add_route(controllers::institution::reference::unit_types::routes(ctx))
            .add_route(controllers::institution::reference::varieties::routes(ctx))
            .add_route(controllers::literate::categories::routes(ctx))
            .add_route(controllers::literate::educations::routes())
            .add_route(controllers::literate::groups::routes(ctx))
            .add_route(controllers::literate::levels::routes(ctx))
            .add_route(controllers::literate::varieties::routes(ctx))
            .add_route(controllers::location::continents::routes())
            .add_route(controllers::location::countries::routes())
            .add_route(controllers::location::provinces::routes())
            .add_route(controllers::location::regencies::routes())
            .add_route(controllers::location::regency_types::routes())
            .add_route(controllers::location::regions::routes())
            .add_route(controllers::location::sub_districts::routes())
            .add_route(controllers::location::villages::routes())
            .add_route(controllers::payment::midtrans::student::payment::routes(ctx))
            .add_route(controllers::person::master::family_cards::routes(ctx))
            .add_route(controllers::person::master::individuals::routes(ctx))
            .add_route(controllers::person::reference::blood_types::routes(ctx))
            .add_route(controllers::person::reference::eye_colors::routes(ctx))
            .add_route(controllers::person::reference::genders::routes(ctx))
            .add_route(controllers::person::reference::hair_colors::routes(ctx))
            .add_route(controllers::person::reference::hair_types::routes(ctx))
            .add_route(controllers::person::reference::identification_types::routes(ctx))
            .add_route(controllers::person::reference::marital_statuses::routes(ctx))
            .add_route(controllers::person::reference::occupations::routes(ctx))
            .add_route(controllers::person::reference::professions::routes(ctx))
            .add_route(controllers::person::reference::relative_types::routes(ctx))
            .add_route(controllers::person::reference::religions::routes(ctx))
            .add_route(controllers::broadcast::transmit::routes())
    }
    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(crate::workers::feeder_dikti::downstream::master::upsert::get_list_komponen_evaluasi_kelas::Worker::build(ctx)).await?;
        queue.register(crate::workers::feeder_dikti::downstream::master::upsert::get_list_mahasiswa::Worker::build(ctx)).await?;
        queue.register(crate::workers::feeder_dikti::downstream::master::upsert::get_biodata_mahasiswa::Worker::build(ctx)).await?;
        queue.register(crate::workers::feeder_dikti::downstream::master::upsert::get_list_perkuliahan_mahasiswa::Worker::build(ctx)).await?;
        queue.register(crate::workers::feeder_dikti::downstream::master::upsert::get_list_nilai_transfer_pendidikan_mahasiswa::Worker::build(ctx)).await?;
        queue.register(crate::workers::feeder_dikti::downstream::master::upsert::get_list_riwayat_pendidikan_mahasiswa::Worker::build(ctx)).await?;
        queue.register(crate::workers::feeder_dikti::downstream::master::upsert::get_krs_mahasiswa::Worker::build(ctx)).await?;
        queue.register(crate::workers::feeder_dikti::downstream::master::upsert::get_riwayat_nilai_mahasiswa::Worker::build(ctx)).await?;
        queue.register(crate::workers::feeder_dikti::downstream::master::upsert::get_transkrip_mahasiswa::Worker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
        tasks.register(tasks::feeder_dikti::downstream::estimasi::master::get_list_komponen_evaluasi_kelas::EstimateKomponenEvaluasiKelas);
        tasks.register(
            tasks::feeder_dikti::downstream::estimasi::master::get_list_mahasiswa::EstimateMahasiswa,
        );
        tasks.register(
            tasks::feeder_dikti::downstream::estimasi::master::get_biodata_mahasiswa::EstimateBiodataMahasiswa,
        );
        tasks.register(
            tasks::feeder_dikti::downstream::estimasi::master::get_list_perkuliahan_mahasiswa::EstimatePerkuliahanMahasiswa,
        );
        tasks.register(
            tasks::feeder_dikti::downstream::estimasi::master::get_list_nilai_transfer_pendidikan_mahasiswa::EstimateNilaiTransferPendidikanMahasiswa,
        );
        tasks.register(
            tasks::feeder_dikti::downstream::estimasi::master::get_list_riwayat_pendidikan_mahasiswa::EstimateRiwayatPendidikanMahasiswa,
        );
        tasks.register(
            tasks::feeder_dikti::downstream::estimasi::master::get_krs_mahasiswa::EstimateKRSMahasiswa,
        );
        tasks.register(
            tasks::feeder_dikti::downstream::estimasi::master::get_riwayat_nilai_mahasiswa::EstimateRiwayatNilaiMahasiswa,
        );
        tasks.register(
            tasks::feeder_dikti::downstream::estimasi::master::get_transkrip_mahasiswa::EstimateTranskripMahasiswa,
        );

        tasks.register(tasks::tui::generate_hash_password::GenerateHashPassword);
        tasks.register(tasks::tui::regenerate_all_student_detail_activities::RegenerateAllStudentDetailActivities);
        tasks.register(tasks::tui::generate_institution_unit_campaign_activities::GenerateInstitutionUnitCampaignActivities);
        tasks.register(
            tasks::tui::generate_student_campaign_activities::GenerateUnitStudentCampaignActivities,
        );
        tasks.register(tasks::tui::generate_student_payment_midtrans_transaction::GenerateStudentPaymentMidtransTransaction);
    }
    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, users::Entity).await?;
        Ok(())
    }
    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        // db::seed::<users::ActiveModel>(
        //     &ctx.db,
        //     &base.join("auth/users.yaml").display().to_string(),
        // )
        // .await?;
        db::seed::<PersonReferenceHairColor::ActiveModel>(
            &ctx.db,
            &base
                .join("person/reference/hair_colors.yaml")
                .display()
                .to_string(),
        )
        .await?;
        Ok(())
    }
}
