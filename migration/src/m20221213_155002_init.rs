use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Campus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Campus::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Campus::Name).string().not_null())
                    .col(ColumnDef::new(Campus::Address).string().not_null())
                    .col(ColumnDef::new(Campus::Distance).string().not_null())
                    .to_owned()
            ).await.unwrap();
        manager
            .create_table(
                Table::create()
                    .table(Building::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Building::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Building::Name).string().not_null())
                    .col(ColumnDef::new(Building::Detail).string())
                    .to_owned(),
            ).await.unwrap();
        manager
            .create_table(
                Table::create()
                    .table(Bus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bus::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Bus::PlatNumber).integer().not_null())
                    .col(ColumnDef::new(Bus::CampusId).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-campus-bus")
                            .from_tbl(Campus::Table)
                            .from_col(Campus::Id)
                            .to_tbl(Bus::Table)
                            .to_col(Bus::CampusId)
                    )
                    .to_owned(),
            ).await.unwrap();
        manager
            .create_table(
                Table::create()
                    .table(Faculty::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Faculty::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Faculty::CampusId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Faculty::Name)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Faculty::Dean)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Faculty::BuildingId)
                            .integer()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-campus-faculty")
                            .from_tbl(Campus::Table)
                            .from_col(Campus::Id)
                            .to_tbl(Faculty::Table)
                            .to_col(Faculty::CampusId),
                        // ForeignKeyCreateStatement::new()
                        //     .name("fk-building-faculty")
                        //     .from_tbl(Building)
                        //     .from_col(Building::Column::Id)
                        //     .to_tbl(Faculty)
                        //     .to_col(Faculty::Column::BuildingId)
                    )
                    .to_owned()
            ).await.unwrap();
        manager
            .create_table(
                Table::create()
                    .table(School::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(School::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(School::Name)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(School::CampusId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(School::FacultyId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(School::BuildingId)
                            .integer()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-campus-school")
                            .from_tbl(Campus::Table)
                            .from_col(Campus::Id)
                            .to_tbl(School::Table)
                            .to_col(School::CampusId),
                        // ForeignKeyCreateStatement::new()
                        //     .name("fk-faculty-school")
                        //     .from_tbl(Faculty)
                        //     .from_col(Faculty::Column::Id)
                        //     .to_tbl(School)
                        //     .to_col(School::Column::FacultyId),
                        // ForeignKeyCreateStatement::new()
                        //     .name("fk-building-school")
                        //     .from_tbl(Building)
                        //     .from_col(Building::Column::Id)
                        //     .to_tbl(School)
                        //     .to_col(School::Column::BuildingId)
                    )
                    .to_owned()
            ).await.unwrap();
        manager
            .create_table(
                Table::create()
                    .table(Programme::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Programme::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Programme::Name)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Programme::Level)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Programme::Duration)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Programme::SchoolId)
                            .integer()
                            .not_null()
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Programme::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden

#[derive(Iden)]
enum Campus {
    Table,
    Id,
    Name,
    Address,
    Distance
}

#[derive(Iden)]
enum Building {
    Table,
    Id,
    Name,
    Detail
}

#[derive(Iden)]
enum Bus {
    Table,
    Id,
    #[iden ="plat_number"]
    PlatNumber,
    #[iden="campus_id"]
    CampusId
}

#[derive(Iden)]
enum Faculty {
    Table,
    Id,
    Name,
    #[iden="campus_id"]
    CampusId,
    Dean,
    BuildingId
}

#[derive(Iden)]
enum School {
    Table,
    Id,
    Name,
    #[iden="campus_id"]
    CampusId,
    #[iden="faculty_id"]
    FacultyId,
    #[iden="building_id"]
    BuildingId
}

#[derive(Iden)]
enum Programme {
    Table,
    Id,
    Name,
    Level,
    Duration,
    #[iden="school_id"]
    SchoolId,
}

// #[derive(Iden)]
// enum Student {
//     Table,
//     Id,
//     Name,
//     Birthday,
//     EnrollDate,
//     #[iden="programme_id"]
//     ProgrammeId
// }


// #[derive(Iden)]
// enum Course {
//     Table,
//     Id,
//     Name,
//     Prerequite,
//     #[iden="programme_id"]
//     ProgrammeId
// }

// #[derive(Iden)]
// enum CourseStudent {
//     Table,
//     Id,
//     #[iden="studen_id"]
//     StudenId,
//     #[iden="course_id"]
//     CourseId,
//     #[iden="taken_date"]
//     TakenDate,
//     #[iden="semester_taken"]
//     SemesterTaken,
//     Award
// }

// #[derive(Iden)]
// enum Club {
//     Table,
//     Id,
//     Name,
//     Phone,
//     #[iden="campus_id"]
//     CampusId,
//     #[iden="building_id"]
//     BuildingId
// }

// #[derive(Iden)]
// enum Sport {
//     Table,
//     Id,
//     Name,
//     #[iden="club_id"]
//     ClubId
// }

// #[derive(Iden)]
// enum Prerequite {
//     Table,
//     CourseId,
//     CourseStudentId,
//     Name
// }

// #[derive(Iden)]
// enum Lecture {
//     Table,
//     Id,
//     SchoolId,
//     Name,
//     Title,
//     OfficeRoom,
//     LectureId
// }

// #[derive(Iden)]
// enum Comitee {
//     Table,
//     Id,
//     FacultyId,
//     Title
// }

// #[derive(iden)]
// enum LectureComitee {
//     Table,
//     Id,
//     LectureId,
//     ComiteeId,
//     FacultyId
// }