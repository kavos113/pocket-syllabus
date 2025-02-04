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
    pub quarter: Vec<Semester>,
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

#[derive(FromRow, Debug)]
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
    id: i32,
    course_id: i32,
    day: i32,
    periods: i32,
    room: String,
}

#[derive(FromRow)]
struct LecturerRow {
    id: i32,
    course_id: i32,
    name: String,
    url: String,
}

#[derive(FromRow)]
struct SemesterRow {
    id: i32,
    course_id: i32,
    semester: i32,
}

pub async fn search_courses(pool: &SqlitePool, search_query: SearchQuery) -> Vec<CourseListItem> {
    let mut tx = pool.begin().await.unwrap();

    let query = "SELECT id, university, code, title, department, credit, year FROM courses ";
    let mut constraints = Vec::new();

    if !search_query.university.is_empty() {
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

    if !search_query.department.is_empty() {
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

    if !search_query.year.is_empty() {
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

    if !search_query.quarter.is_empty() {
        constraints.push(format!(
            "quarter IN ({})",
            search_query
                .quarter
                .iter()
                .map(|s| format!(
                    "'{}'",
                    match s {
                        Semester::First => 1,
                        Semester::Second => 2,
                        Semester::Third => 3,
                        Semester::Fourth => 4,
                    }
                ))
                .collect::<Vec<String>>()
                .join(",")
        ));
    }

    let query = format!(
        "{}{}{}",
        query,
        if !constraints.is_empty() {
            "WHERE "
        } else {
            ""
        },
        constraints.join(" AND ")
    );

    println!("query: {}", query);

    let mut rows = sqlx::query_as::<_, CourseListItemRow>(&query)
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    tx.commit().await.unwrap();

    // grade
    if !search_query.grade.is_empty() {
        let grades = search_query
            .grade
            .iter()
            .map(|s| match s.as_str() {
                "100" => '1',
                "200" => '2',
                "300" => '3',
                "400" => '4',
                "500" => '5',
                "600" => '6',
                _ => '0',
            })
            .collect::<Vec<char>>();

        rows.retain(|row| {
            grades
                .iter()
                .any(|grade| row.code.chars().nth(5).unwrap() == *grade)
        });
    }

    // timetable
    if !search_query.timetable.is_empty() {
        let timetable_query = format!(
            "SELECT course_id FROM timetables WHERE day IN ({}) AND periods IN ({})",
            search_query
                .timetable
                .iter()
                .map(|t| format!(
                    "{}",
                    match t.day {
                        Day::Monday => {
                            1
                        }
                        Day::Tuesday => {
                            2
                        }
                        Day::Wednesday => {
                            3
                        }
                        Day::Thursday => {
                            4
                        }
                        Day::Friday => {
                            5
                        }
                        Day::Saturday => {
                            6
                        }
                        Day::Sunday => {
                            0
                        }
                    }
                ))
                .collect::<Vec<String>>()
                .join(","),
            search_query
                .timetable
                .iter()
                .map(|t| format!(
                    "{}",
                    match t.period {
                        Period::First => {
                            1
                        }
                        Period::Second => {
                            2
                        }
                        Period::Third => {
                            3
                        }
                        Period::Fourth => {
                            4
                        }
                        Period::Fifth => {
                            5
                        }
                        Period::Sixth => {
                            6
                        }
                    }
                ))
                .collect::<Vec<String>>()
                .join(",")
        );

        let mut tx = pool.begin().await.unwrap();

        let timetable_ids = sqlx::query_as::<_, TimetableRow>(&timetable_query)
            .fetch_all(&mut *tx)
            .await
            .unwrap();

        tx.commit().await.unwrap();

        rows.retain(|row| {
            timetable_ids
                .iter()
                .any(|timetable_id| timetable_id.course_id == row.id)
        });
    }

    let mut results = Vec::with_capacity(rows.len());

    for row in rows {
        let mut tx = pool.begin().await.unwrap();

        let lecturers =
            sqlx::query_as::<_, LecturerRow>("SELECT * FROM lecturers WHERE course_id = ?")
                .bind(row.id)
                .fetch_all(&mut *tx)
                .await
                .unwrap();

        let timetables =
            sqlx::query_as::<_, TimetableRow>("SELECT * FROM timetables WHERE course_id = ?")
                .bind(row.id)
                .fetch_all(&mut *tx)
                .await
                .unwrap();

        let semesters =
            sqlx::query_as::<_, SemesterRow>("SELECT * FROM semesters WHERE course_id = ?")
                .bind(row.id)
                .fetch_all(&mut *tx)
                .await
                .unwrap();

        tx.commit().await.unwrap();

        let lectures = lecturers
            .iter()
            .map(|lecturer| lecturer.name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        let timetables = timetables
            .iter()
            .map(|timetable| {
                format!(
                    "{}{}",
                    match timetable.day {
                        0 => "日",
                        1 => "月",
                        2 => "火",
                        3 => "水",
                        4 => "木",
                        5 => "金",
                        6 => "土",
                        _ => "",
                    },
                    match timetable.periods {
                        1 => "1-2",
                        2 => "3-4",
                        3 => "5-6",
                        4 => "7-8",
                        5 => "9-10",
                        6 => "11-12",
                        _ => "",
                    }
                )
            })
            .collect::<Vec<String>>()
            .join(", ");

        let semesters = semesters
            .iter()
            .map(|semester| {
                (match semester.semester {
                    1 => "1Q",
                    2 => "2Q",
                    3 => "3Q",
                    4 => "4Q",
                    _ => "",
                })
                .to_string()
            })
            .collect::<Vec<String>>()
            .join(", ");

        results.push(CourseListItem {
            id: row.id,
            university: row.university,
            code: row.code,
            title: row.title,
            lecturer: lectures,
            timetable: timetables,
            semester: semesters,
            department: row.department,
            credit: row.credit,
        });
    }

    // title
    if !search_query.title.is_empty() {
        results.retain(|result| {
            search_query
                .title
                .iter()
                .any(|title| result.title.contains(title))
        });
    }

    // lecturer
    if !search_query.lecturer.is_empty() {
        results.retain(|result| {
            search_query
                .lecturer
                .iter()
                .any(|lecturer| result.lecturer.contains(lecturer))
        });
    }

    results
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseResponse {
    id: i32,
    university: String,
    title: String,
    english_title: String,
    department: String,
    lecturer: Vec<LecturerResponse>,
    lecture_type: String,
    timetable: Vec<TimetableResponse>,
    code: String,
    credit: i32,
    year: i32,
    semester: Vec<i32>,
    language: String,
    course_detail: CourseDetailResponse,
    url: String,
    sylbs_update: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LecturerResponse {
    id: i32,
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimetableResponse {
    course_id: i32,
    day_of_week: i32,
    period: i32,
    room: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDetailResponse {
    abst: String,
    goal: String,
    experience: bool,
    keyword: Vec<String>,
    competencies: Vec<String>,
    flow: String,
    schedule: Vec<ScheduleResponse>,
    out_of_class: String,
    textbook: String,
    reference_book: String,
    assessment: String,
    related_course: Vec<String>,
    prerequisite: String,
    contact: String,
    office_hour: String,
    note: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleResponse {
    count: i32,
    plan: String,
    assignment: String,
}

#[derive(FromRow)]
struct CourseRow {
    id: i32,
    university: String,
    title: String,
    english_title: String,
    department: String,
    lecture_type: String,
    code: String,
    credit: i32,
    year: i32,
    language: String,
    url: String,
    r#abstract: String,
    goal: String,
    experience: bool,
    flow: String,
    out_of_class: String,
    textbook: String,
    reference_book: String,
    assessment: String,
    prerequisite: String,
    contact: String,
    office_hour: String,
    note: String,
    sylbs_update: String,
}

#[derive(FromRow)]
struct KeywordRow {
    id: i32,
    course_id: i32,
    keyword: String,
}

#[derive(FromRow)]
struct CompetencyRow {
    id: i32,
    course_id: i32,
    competency: String,
}

#[derive(FromRow)]
struct ScheduleRow {
    id: i32,
    course_id: i32,
    count: i32,
    plan: String,
    assignment: String,
}

#[derive(FromRow)]
struct RelatedCourseRow {
    id: i32,
    course_id: i32,
    related_course_code: String,
}

pub async fn get_course(pool: &SqlitePool, id: i32) -> CourseResponse {
    let mut tx = pool.begin().await.unwrap();

    let course = sqlx::query_as::<_, CourseRow>("SELECT * FROM courses WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    let lecturers = sqlx::query_as::<_, LecturerRow>("SELECT * FROM lecturers WHERE course_id = ?")
        .bind(id)
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    let timetables =
        sqlx::query_as::<_, TimetableRow>("SELECT * FROM timetables WHERE course_id = ?")
            .bind(id)
            .fetch_all(&mut *tx)
            .await
            .unwrap();

    let semesters = sqlx::query_as::<_, SemesterRow>("SELECT * FROM semesters WHERE course_id = ?")
        .bind(id)
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    let keywords = sqlx::query_as::<_, KeywordRow>("SELECT * FROM keywords WHERE course_id = ?")
        .bind(id)
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    let competencies =
        sqlx::query_as::<_, CompetencyRow>("SELECT * FROM competencies WHERE course_id = ?")
            .bind(id)
            .fetch_all(&mut *tx)
            .await
            .unwrap();

    let schedules = sqlx::query_as::<_, ScheduleRow>("SELECT * FROM schedules WHERE course_id = ?")
        .bind(id)
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    let related_courses =
        sqlx::query_as::<_, RelatedCourseRow>("SELECT * FROM related_courses WHERE course_id = ?")
            .bind(id)
            .fetch_all(&mut *tx)
            .await
            .unwrap();

    tx.commit().await.unwrap();

    CourseResponse {
        id: course.id,
        university: course.university,
        title: course.title,
        english_title: course.english_title,
        department: course.department,
        lecturer: lecturers
            .iter()
            .map(|lecturer| LecturerResponse {
                id: lecturer.id,
                name: lecturer.name.clone(),
                url: lecturer.url.clone(),
            })
            .collect(),
        lecture_type: course.lecture_type,
        timetable: timetables
            .iter()
            .map(|timetable| TimetableResponse {
                course_id: timetable.course_id,
                day_of_week: timetable.day,
                period: timetable.periods,
                room: timetable.room.clone(),
            })
            .collect(),
        code: course.code,
        credit: course.credit,
        year: course.year,
        semester: semesters.iter().map(|semester| semester.semester).collect(),
        language: course.language,
        course_detail: CourseDetailResponse {
            abst: course.r#abstract,
            goal: course.goal,
            experience: course.experience,
            keyword: keywords
                .iter()
                .map(|keyword| keyword.keyword.clone())
                .collect(),
            competencies: competencies
                .iter()
                .map(|competency| competency.competency.clone())
                .collect(),
            flow: course.flow,
            schedule: schedules
                .iter()
                .map(|schedule| ScheduleResponse {
                    count: schedule.count,
                    plan: schedule.plan.clone(),
                    assignment: schedule.assignment.clone(),
                })
                .collect(),
            out_of_class: course.out_of_class,
            textbook: course.textbook,
            reference_book: course.reference_book,
            assessment: course.assessment,
            related_course: related_courses
                .iter()
                .map(|related_course| related_course.related_course_code.clone())
                .collect(),
            prerequisite: course.prerequisite,
            contact: course.contact,
            office_hour: course.office_hour,
            note: course.note,
        },
        url: course.url,
        sylbs_update: course.sylbs_update,
    }
}
