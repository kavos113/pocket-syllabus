use crate::scrape::{CourseTitle, Lecturer};
use scraper::{ElementRef, Html, Selector};

#[derive(Debug)]
pub struct CourseAbstract {
    code: String,
    title: CourseTitle,
    lecturer: Vec<Lecturer>,
    department: String,
    start: String,
    sylbs_update: String,
}

pub fn html_to_course_abstracts(html: &str) -> Vec<CourseAbstract> {
    let mut courses = Vec::new();

    let document = Html::parse_document(html);
    let rankings_selector = Selector::parse(".ranking-list").unwrap();
    let tbody_selector = Selector::parse("tbody").unwrap();
    let row_selector = Selector::parse("tr").unwrap();

    let rankings = document.select(&rankings_selector).next().unwrap();
    let tbody = rankings.select(&tbody_selector).next().unwrap();
    let rows = tbody.select(&row_selector);

    let code_selector = Selector::parse(".code").unwrap();
    let course_title_selector = Selector::parse(".course_title").unwrap();
    let lecturer_selector = Selector::parse(".lecturer").unwrap();
    let opening_department_selector = Selector::parse(".opening_department").unwrap();
    let start_selector = Selector::parse(".start").unwrap();
    let sylbs_update_selector = Selector::parse(".sylbs").unwrap();

    for row in rows {
        let code = row.select(&code_selector).next().unwrap();
        let course_title = row.select(&course_title_selector).next().unwrap();
        let lecturer = row.select(&lecturer_selector).next().unwrap();
        let opening_department = row.select(&opening_department_selector).next().unwrap();
        let start = row.select(&start_selector).next().unwrap();
        let sylbs_update = row.select(&sylbs_update_selector).next().unwrap();

        courses.push(CourseAbstract {
            code: get_code(code),
            title: get_course_title(course_title),
            lecturer: get_lecturer(lecturer),
            department: get_opening_department(opening_department),
            start: get_start(start),
            sylbs_update: get_sylbs_update(sylbs_update),
        });
    }

    courses
}

fn get_code(td: ElementRef) -> String {
    td.inner_html().trim().to_string()
}

fn get_course_title(td: ElementRef) -> CourseTitle {
    let a_err = td.select(&Selector::parse("a").unwrap()).next();
    let a = match a_err {
        Some(a) => a,
        None => {
            return CourseTitle {
                title: "".to_string(),
                url: "".to_string(),
            };
        }
    };
    CourseTitle {
        title: a.inner_html().trim().to_string(),
        url: format!(
            "https://www.ocw.titech.ac.jp/{}",
            a.value().attr("href").unwrap()
        ),
    }
}

fn get_lecturer(td: ElementRef) -> Vec<Lecturer> {
    let mut ret = Vec::new();
    for a in td.select(&Selector::parse("a").unwrap()) {
        ret.push(Lecturer {
            name: a.inner_html().trim().to_string(),
            url: format!(
                "https://www.ocw.titech.ac.jp/{}",
                a.value().attr("href").unwrap()
            ),
        });
    }

    ret
}

fn get_opening_department(td: ElementRef) -> String {
    let a_err = td.select(&Selector::parse("a").unwrap()).next();
    let a = match a_err {
        Some(a) => a,
        None => {
            return "".to_string();
        }
    };
    a.inner_html().trim().to_string()
}

fn get_start(td: ElementRef) -> String {
    td.inner_html().trim().to_string()
}

fn get_sylbs_update(td: ElementRef) -> String {
    td.inner_html().trim().to_string()
}
