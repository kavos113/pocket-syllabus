use crate::scrape::{Day, Period, Semester};
use crate::Course;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::{FromRow, Row, SqlitePool};
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
                    sylbs_update,
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
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
    .bind(&course.sylbs_update)
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
    .execute(&mut *tx)
    .await?;

    let last_id = sqlx::query("SELECT last_insert_rowid() as id")
        .fetch_one(&mut *tx)
        .await?
        .try_get::<i64, _>("id")?;

    for teacher in &course.lecturer {
        sqlx::query(
            "INSERT INTO lecturers (
                course_id,
                name,
                url
            ) VALUES (?, ?, ?)",
        )
        .bind(last_id)
        .bind(&teacher.name)
        .bind(&teacher.url)
        .execute(&mut *tx)
        .await?;
    }

    for timetable in &course.time_table {
        sqlx::query(
            "INSERT INTO timetables (
                course_id,
                day,
                periods,
                room
            ) VALUES (?, ?, ?, ?)",
        )
        .bind(last_id)
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
        .execute(&mut *tx)
        .await?;
    }

    for sem in &course.semester {
        sqlx::query(
            "INSERT INTO semesters (
                course_id,
                semester
            ) VALUES (?, ?)",
        )
        .bind(last_id)
        .bind(match &sem {
            Semester::First => 1,
            Semester::Second => 2,
            Semester::Third => 3,
            Semester::Fourth => 4,
        })
        .execute(&mut *tx)
        .await?;
    }

    for key in &course.course_detail.keyword {
        sqlx::query(
            "INSERT INTO keywords (
                course_id,
                keyword
            ) VALUES (?, ?)",
        )
        .bind(last_id)
        .bind(&key)
        .execute(&mut *tx)
        .await?;
    }

    for competency in &course.course_detail.competencies {
        sqlx::query(
            "INSERT INTO competencies (
                course_id,
                competency
            ) VALUES (?, ?)",
        )
        .bind(last_id)
        .bind(&competency)
        .execute(&mut *tx)
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
        .bind(last_id)
        .bind(&schedule.count)
        .bind(&schedule.plan)
        .bind(&schedule.assignment)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    let mut tx = pool.begin().await?;

    for related in &course.course_detail.related_course {
        let related_code = related.chars().take(8).collect::<String>();

        sqlx::query(
            "INSERT INTO related_courses (
                course_id,
                related_course_code
            ) VALUES (?, ?)",
        )
        .bind(last_id)
        .bind(related_code)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}

pub async fn check_sylbs_update(
    pool: &SqlitePool,
    code: &str,
    title: &str,
    sylbs_update: &str,
) -> DbResult<bool> {
    let mut tx = pool.begin().await?;

    let row = sqlx::query("SELECT sylbs_update FROM courses WHERE code = ? AND title = ?")
        .bind(code)
        .bind(title)
        .fetch_optional(&mut *tx)
        .await?;

    let result = match row {
        Some(row) => {
            let db_sylbs_update: String = row.try_get("sylbs_update")?;
            db_sylbs_update == sylbs_update
        }
        None => false,
    };

    tx.commit().await?;

    Ok(result)
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub university: Vec<String>,
    pub department: Vec<String>,
    pub year: Vec<String>,
    pub title: Vec<String>,
    pub lecturer: Vec<String>,
    pub grade: Vec<String>,
    pub quarter: Vec<String>,
    pub timetable: Vec<TimetableQuery>,
}

#[derive(Debug, Deserialize)]
pub struct TimetableQuery {
    pub day: Day,
    pub period: Period,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CourseListItem {
    id: i32,
    university: String,
    code: String,
    title: String,
    lecturer: String,
    timetable: String,
    semester: String,
    department: String,
    credit: i32,
}

#[derive(FromRow)]
struct CourseListItemRow {
    id: i32,
    university: String,
    code: String,
    title: String,
    department: String,
    credit: i32,
    year: i32,
}

#[derive(FromRow)]
struct TimetableRow {
    day: i32,
    periods: i32,
}

pub async fn search_courses(pool: &SqlitePool, search_query: SearchQuery) -> Vec<CourseListItem> {
    let mut tx = pool.begin().await.unwrap();

    let query = "SELECT id, university, code, title, department, credit, year FROM courses";
    let mut constraints = Vec::new();

    if search_query.university.len() > 0 {
        constraints.push(format!(
            "university IN ({})",
            search_query
                .university
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(",")
        ));
    }

    if search_query.department.len() > 0 {
        constraints.push(format!(
            "department IN ({})",
            search_query
                .department
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(",")
        ));
    }

    if search_query.year.len() > 0 {
        constraints.push(format!(
            "year IN ({})",
            search_query
                .year
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(",")
        ));
    }

    if search_query.title.len() > 0 {
        constraints.push(format!(
            "language IN ({})",
            search_query
                .title
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(",")
        ));
    }

    if search_query.lecturer.len() > 0 {
        constraints.push(format!(
            "lecturer IN ({})",
            search_query
                .lecturer
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(",")
        ));
    }

    if search_query.grade.len() > 0 {
        constraints.push(format!(
            "grade IN ({})",
            search_query
                .grade
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(",")
        ));
    }

    if search_query.quarter.len() > 0 {
        constraints.push(format!(
            "quarter IN ({})",
            search_query
                .quarter
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(",")
        ));
    }

    let query = format!(
        "{}{}{}",
        query,
        if constraints.len() > 0 { "WHERE " } else { "" },
        constraints.join(" AND ")
    );

    println!("query: {}", query);

    Vec::new()
}
