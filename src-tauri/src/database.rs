use crate::scrape::{Day, Period, Semester};
use crate::Course;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlitePool;
use std::str::FromStr;

type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

pub async fn create_sqlite_pool(database_path: &str) -> DbResult<SqlitePool> {
    let connection_options = SqliteConnectOptions::from_str(database_path)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    Ok(pool)
}

pub async fn migrate(pool: &SqlitePool) -> DbResult<()> {
    sqlx::migrate!("./db").run(pool).await?;

    Ok(())
}

pub async fn insert_course(pool: &SqlitePool, course: &Course) -> DbResult<()> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO courses (
                    university, 
                    title,
                    english_title,
                    department,
                    lecture_type,
                    code,
                    credit,
                    year,
                    language,
                    url,
                    abstract,
                    goal,
                    experience,
                    flow,
                    out_of_class,
                    textbook,
                    reference_book,
                    assessment,
                    prerequisite,
                    contact,
                    office_hour,
                    note
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&course.university)
    .bind(&course.title)
    .bind(&course.english_title)
    .bind(&course.department)
    .bind(&course.lecture_type)
    .bind(&course.code)
    .bind(&course.credit)
    .bind(&course.year)
    .bind(&course.language)
    .bind(&course.url)
    .bind(&course.course_detail.abst)
    .bind(&course.course_detail.goal)
    .bind(&course.course_detail.experience)
    .bind(&course.course_detail.flow)
    .bind(&course.course_detail.out_of_class)
    .bind(&course.course_detail.textbook)
    .bind(&course.course_detail.reference_book)
    .bind(&course.course_detail.assessment)
    .bind(&course.course_detail.prerequisite)
    .bind(&course.course_detail.contact)
    .bind(&course.course_detail.office_hour)
    .bind(&course.course_detail.note)
    .execute(&mut tx)
    .await?;

    let last_id = sqlx::query("SELECT last_insert_rowid() as id")
        .fetch_one(&mut tx)
        .await?;

    for teacher in &course.lecturer {
        sqlx::query(
            "INSERT INTO lecturers (
                course_id,
                name,
                url
            ) VALUES (?, ?, ?)",
        )
        .bind(last_id.id)
        .bind(&teacher.name)
        .bind(&teacher.url)
        .execute(&mut tx)
        .await?;
    }

    for timetable in &course.time_table {
        sqlx::query(
            "INSERT INTO timetables (
                course_id,
                day,
                period,
                room
            ) VALUES (?, ?, ?, ?)",
        )
        .bind(last_id.id)
        .bind(match &timetable.day {
            Day::Sunday => 0,
            Day::Monday => 1,
            Day::Tuesday => 2,
            Day::Wednesday => 3,
            Day::Thursday => 4,
            Day::Friday => 5,
            Day::Saturday => 6,
        })
        .bind(match &timetable.period {
            Period::First => 1,
            Period::Second => 2,
            Period::Third => 3,
            Period::Fourth => 4,
            Period::Fifth => 5,
            Period::Sixth => 6,
        })
        .bind(&timetable.room)
        .execute(&mut tx)
        .await?;
    }

    for sem in &course.semester {
        sqlx::query(
            "INSERT INTO semesters (
                course_id,
                semester
            ) VALUES (?, ?)",
        )
        .bind(last_id.id)
        .bind(match &sem {
            Semester::First => 1,
            Semester::Second => 2,
            Semester::Third => 3,
            Semester::Fourth => 4,
        })
        .execute(&mut tx)
        .await?;
    }

    for key in &course.course_detail.keyword {
        sqlx::query(
            "INSERT INTO keywords (
                course_id,
                keyword
            ) VALUES (?, ?)",
        )
        .bind(last_id.id)
        .bind(&key)
        .execute(&mut tx)
        .await?;
    }

    for competency in &course.course_detail.competencies {
        sqlx::query(
            "INSERT INTO competencies (
                course_id,
                competency
            ) VALUES (?, ?)",
        )
        .bind(last_id.id)
        .bind(&competency)
        .execute(&mut tx)
        .await?;
    }

    for schedule in &course.course_detail.schedule {
        sqlx::query(
            "INSERT INTO schedules (
                course_id,
                count,
                plan,
                assignment
            ) VALUES (?, ?, ?, ?)",
        )
        .bind(last_id.id)
        .bind(&schedule.count)
        .bind(&schedule.plan)
        .bind(&schedule.assignment)
        .execute(&mut tx)
        .await?;
    }

    for related in &course.course_detail.related_course {
        let related_code = related.chars().take(8).collect::<String>();
        let related_id = sqlx::query("SELECT id FROM courses WHERE code = ?")
            .bind(&related_code)
            .fetch_one(&mut tx)
            .await?;

        sqlx::query(
            "INSERT INTO related_courses (
                course_id,
                related_id
            ) VALUES (?, ?)",
        )
        .bind(last_id.id)
        .bind(related_id.id)
        .execute(&mut tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
