use crate::scrape::Lecturer;
use scraper::{ElementRef, Html, Selector};

#[derive(Debug)]
pub struct Course {
    university: String,
    title: String,
    english_title: String,
    department: String,
    lecturer: Vec<Lecturer>,
    lecture_type: String,
    time_table: Vec<TimeTable>,
    code: String,
    credit: i32,
    year: i32,
    semester: Vec<Semester>,
    language: String,
    url: String,
    course_detail: CourseDetail,
}

#[derive(Debug)]
pub struct CourseDetail {
    abst: String,
    goal: String,
    experience: bool,
    keyword: Vec<String>,
    competencies: Vec<String>,
    flow: String,
    schedule: String,
    out_of_class: String,
    textbook: String,
    reference_book: String,
    assessment: String,
    related_course: String,
    prerequisite: String,
    note: String,
}

#[derive(Debug)]
pub struct TimeTable {
    day: Day,
    period: Period,
    room: String,
}

#[derive(Debug)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug)]
pub enum Period {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
}

#[derive(Debug)]
pub enum Semester {
    First,
    Second,
    Third,
    Fourth,
}

pub fn html_to_course(html: &str) -> Course {
    let document = Html::parse_document(html);

    let university = String::from("東京工業大学");

    let title = document
        .select(&Selector::parse(".page-title-area").unwrap())
        .next()
        .unwrap();
    let (title, english_title) = get_title(
        title
            .select(&Selector::parse("h3").unwrap())
            .next()
            .unwrap(),
    );

    let abst_data = document
        .select(&Selector::parse(".gaiyo-data").unwrap())
        .next()
        .unwrap();
    let mut dls = abst_data.select(&Selector::parse("dl").unwrap());
    let dd_selector = Selector::parse("dd").unwrap();

    let department = get_department(dls.nth(0).unwrap().select(&dd_selector).next().unwrap());
    let lecturer = get_lecturer(dls.nth(1).unwrap().select(&dd_selector).next().unwrap());
    let lecture_type = get_lecture_type(dls.nth(2).unwrap().select(&dd_selector).next().unwrap());
    let time_table = get_timetable(dls.nth(4).unwrap().select(&dd_selector).next().unwrap());
    let code = get_code(dls.nth(6).unwrap().select(&dd_selector).next().unwrap());
    let credit = get_credit(dls.nth(7).unwrap().select(&dd_selector).next().unwrap());
    let year = get_year(dls.nth(8).unwrap().select(&dd_selector).next().unwrap());
    let semester = get_semester(dls.nth(9).unwrap().select(&dd_selector).next().unwrap());
    let language = get_language(dls.nth(12).unwrap().select(&dd_selector).next().unwrap());

    let url = "".to_string();

    let course_detail = CourseDetail {
        abst: "".to_string(),
        goal: "".to_string(),
        experience: false,
        keyword: Vec::new(),
        competencies: Vec::new(),
        flow: "".to_string(),
        schedule: "".to_string(),
        out_of_class: "".to_string(),
        textbook: "".to_string(),
        reference_book: "".to_string(),
        assessment: "".to_string(),
        related_course: "".to_string(),
        prerequisite: "".to_string(),
        note: "".to_string(),
    };

    Course {
        university,
        title,
        english_title,
        department,
        lecturer,
        lecture_type,
        time_table,
        code,
        credit,
        year,
        semester,
        language,
        url,
        course_detail,
    }
}

// (title, english_title)
fn get_title(h3: ElementRef) -> (String, String) {
    let all = h3.inner_html().trim().to_string();

    let titles = all.split("&nbsp;&nbsp;&nbsp;").collect::<Vec<&str>>();
    let title = titles[0].to_string();
    let english_title = titles[1].to_string();

    (title, english_title)
}

fn get_department(dd: ElementRef) -> String {
    dd.inner_html().trim().to_string()
}

fn get_lecturer(dd: ElementRef) -> Vec<Lecturer> {
    let mut lecturers = Vec::new();

    for a in dd.select(&Selector::parse("a").unwrap()) {
        lecturers.push(Lecturer {
            name: a.inner_html().trim().to_string(),
            url: a.value().attr("href").unwrap().to_string(),
        });
    }

    lecturers
}

fn get_lecture_type(dd: ElementRef) -> String {
    dd.inner_html()
        .trim()
        .to_string()
        .replace("\n", "")
        .replace("&nbsp;", "")
}

fn get_timetable(dd: ElementRef) -> Vec<TimeTable> {}

fn get_code(dd: ElementRef) -> String {
    dd.inner_html().trim().to_string()
}

fn get_credit(dd: ElementRef) -> i32 {
    dd.inner_html().trim().to_string().parse().unwrap()
}

fn get_year(dd: ElementRef) -> i32 {
    dd.inner_html()
        .trim()
        .to_string()
        .replace("年度", "")
        .parse()
        .unwrap()
}

fn get_semester(dd: ElementRef) -> Vec<Semester> {
    let semester = dd.inner_html().trim().to_string();

    let mut ret = Vec::new();
    match semester.as_str() {
        "1Q" => ret.push(Semester::First),
        "2Q" => ret.push(Semester::Second),
        "3Q" => ret.push(Semester::Third),
        "4Q" => ret.push(Semester::Fourth),
        "1-2Q" => {
            ret.push(Semester::First);
            ret.push(Semester::Second);
        }
        "1-3Q" => {
            ret.push(Semester::First);
            ret.push(Semester::Second);
            ret.push(Semester::Third);
        }
        "1-4Q" => {
            ret.push(Semester::First);
            ret.push(Semester::Second);
            ret.push(Semester::Third);
            ret.push(Semester::Fourth);
        }
        "2-3Q" => {
            ret.push(Semester::Second);
            ret.push(Semester::Third);
        }
        "2-4Q" => {
            ret.push(Semester::Second);
            ret.push(Semester::Third);
            ret.push(Semester::Fourth);
        }
        "3-4Q" => {
            ret.push(Semester::Third);
            ret.push(Semester::Fourth);
        }
        _ => {}
    }

    ret
}

fn get_language(dd: ElementRef) -> String {
    dd.inner_html().trim().to_string()
}

fn get_details(div: ElementRef) -> CourseDetail {}

fn get_abstract(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_goal(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_keywords(p: ElementRef) -> Vec<String> {
    let keywords = p.inner_html().trim().to_string();
    keywords
        .split(&["、", "，", ", "][..])
        .map(|s| s.to_string())
        .collect()
}

fn get_competencies(div: ElementRef) -> Vec<String> {
    let mut competencies = Vec::new();
    for element in div.select(&Selector::parse("skill_checked2").unwrap()) {
        competencies.push(element.inner_html().trim().to_string());
    }

    competencies
}

fn get_flow(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}
