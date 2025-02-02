use crate::scrape::Lecturer;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::fmt;
use std::fmt::Formatter;

pub struct Course {
    pub university: String,
    pub title: String,
    pub english_title: String,
    pub department: String,
    pub lecturer: Vec<Lecturer>,
    pub lecture_type: String,
    pub time_table: Vec<TimeTable>,
    pub code: String,
    pub credit: i32,
    pub year: i32,
    pub semester: Vec<Semester>,
    pub language: String,
    pub url: String,
    pub course_detail: CourseDetail,
}

pub struct CourseDetail {
    pub abst: String,
    pub goal: String,
    pub experience: bool,
    pub keyword: Vec<String>,
    pub competencies: Vec<String>,
    pub flow: String,
    pub schedule: Vec<LecturePlan>,
    pub out_of_class: String,
    pub textbook: String,
    pub reference_book: String,
    pub assessment: String,
    pub related_course: Vec<String>,
    pub prerequisite: String,
    pub contact: String,
    pub office_hour: String,
    pub note: String,
}

impl fmt::Debug for CourseDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CourseDetail {{\n  abst: {},\n  goal: {},\n  experience: {},\n  keyword: {:?},\n  competencies: {:?},\n  flow: {},\n  schedule: {:?},\n  out_of_class: {},\n  textbook: {},\n  reference_book: {},\n  assessment: {},\n  related_course: {:?},\n  prerequisite: {},\n  contact: {},\n  office_hour: {},\n  note: {}\n}}",
            self.abst, self.goal, self.experience, self.keyword, self.competencies, self.flow, self.schedule, self.out_of_class, self.textbook, self.reference_book, self.assessment, self.related_course, self.prerequisite, self.contact, self.office_hour, self.note
        )
    }
}

impl fmt::Debug for Course {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Course {{\n  university: {},\n  title: {},\n  english_title: {},\n  department: {},\n  lecturer: {:?},\n  lecture_type: {},\n  time_table: {:?},\n  code: {},\n  credit: {},\n  year: {},\n  semester: {:?},\n  language: {},\n  url: {},\n  course_detail: {:?}\n}}",
            self.university, self.title, self.english_title, self.department, self.lecturer, self.lecture_type, self.time_table, self.code, self.credit, self.year, self.semester, self.language, self.url, self.course_detail
        )
    }
}

#[derive(Debug)]
pub struct TimeTable {
    pub day: Day,
    pub period: Period,
    pub room: String,
}

#[derive(Debug)]
pub struct LecturePlan {
    pub count: i32,
    pub plan: String,
    pub assignment: String,
}

#[derive(Debug, Clone)]
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
    let dl_selector = Selector::parse("dl").unwrap();
    let dls = abst_data.select(&dl_selector);
    let dd_selector = Selector::parse("dd").unwrap();

    let department = get_department(
        dls.clone()
            .next()
            .unwrap()
            .select(&dd_selector)
            .next()
            .unwrap(),
    );
    let lecturer = get_lecturer(
        dls.clone()
            .nth(1)
            .unwrap()
            .select(&dd_selector)
            .next()
            .unwrap(),
    );
    let lecture_type = get_lecture_type(
        dls.clone()
            .nth(2)
            .unwrap()
            .select(&dd_selector)
            .next()
            .unwrap(),
    );
    let time_table = get_timetable(
        dls.clone()
            .nth(4)
            .unwrap()
            .select(&dd_selector)
            .next()
            .unwrap(),
    );
    let code = get_code(
        dls.clone()
            .nth(6)
            .unwrap()
            .select(&dd_selector)
            .next()
            .unwrap(),
    );
    let credit = get_credit(
        dls.clone()
            .nth(7)
            .unwrap()
            .select(&dd_selector)
            .next()
            .unwrap(),
    );
    let year = get_year(
        dls.clone()
            .nth(8)
            .unwrap()
            .select(&dd_selector)
            .next()
            .unwrap(),
    );
    let semester = get_semester(
        dls.clone()
            .nth(9)
            .unwrap()
            .select(&dd_selector)
            .next()
            .unwrap(),
    );
    let language = get_language(
        dls.clone()
            .nth(12)
            .unwrap()
            .select(&dd_selector)
            .next()
            .unwrap(),
    );

    let url = "".to_string();

    let course_detail = get_details(
        document
            .select(&Selector::parse("#overview").unwrap())
            .next()
            .unwrap(),
    );

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
    let title = titles[0].to_string().chars().skip(7).collect::<String>();
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
            url: format!(
                "https://www.ocw.titech.ac.jp/{}",
                a.value().attr("href").unwrap()
            ),
        });
    }

    lecturers
}

fn get_lecture_type(dd: ElementRef) -> String {
    let mut text = dd
        .inner_html()
        .trim()
        .to_string()
        .replace("\n", "")
        .replace("&nbsp;", "");
    text.retain(|c| !c.is_whitespace());
    text
}

fn get_timetable(dd: ElementRef) -> Vec<TimeTable> {
    let mut time_tables = Vec::new();

    let str = dd.inner_html().trim().to_string();
    let time_table_strs = str.split("&nbsp;&nbsp;").collect::<Vec<&str>>();

    let room_re = Regex::new(r"\(([^()]*)([()])").unwrap();

    for time_table_str in time_table_strs {
        let day = match time_table_str.chars().next() {
            Some(c) => c,
            None => {
                break;
            }
        };
        let day = match day {
            '月' => Day::Monday,
            '火' => Day::Tuesday,
            '水' => Day::Wednesday,
            '木' => Day::Thursday,
            '金' => Day::Friday,
            '土' => Day::Saturday,
            '日' => Day::Sunday,
            _ => Day::Monday,
        };

        let room = room_re
            .captures(time_table_str)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        let period = time_table_str
            .chars()
            .skip(1)
            .take_while(|c| c.is_ascii_digit() || c == &'-')
            .collect::<String>();
        let period = match period.as_str() {
            "1-2" => Period::First,
            "3-4" => Period::Second,
            "5-6" => Period::Third,
            "7-8" => Period::Fourth,
            "9-10" => Period::Fifth,
            "11-12" => Period::Sixth,
            "1-4" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::First,
                    room: room.clone(),
                });
                Period::Second
            }
            "3-6" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Second,
                    room: room.clone(),
                });
                Period::Third
            }
            "5-8" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Third,
                    room: room.clone(),
                });
                Period::Fourth
            }
            "7-10" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Fourth,
                    room: room.clone(),
                });
                Period::Fifth
            }
            "9-12" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Fifth,
                    room: room.clone(),
                });
                Period::Sixth
            }
            "1-6" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::First,
                    room: room.clone(),
                });
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Second,
                    room: room.clone(),
                });
                Period::Third
            }
            "3-8" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Second,
                    room: room.clone(),
                });
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Third,
                    room: room.clone(),
                });
                Period::Fourth
            }
            "5-10" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Third,
                    room: room.clone(),
                });
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Fourth,
                    room: room.clone(),
                });
                Period::Fifth
            }
            "7-12" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Fourth,
                    room: room.clone(),
                });
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Fifth,
                    room: room.clone(),
                });
                Period::Sixth
            }
            "1-8" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::First,
                    room: room.clone(),
                });
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Second,
                    room: room.clone(),
                });
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Third,
                    room: room.clone(),
                });
                Period::Fourth
            }
            "3-10" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Second,
                    room: room.clone(),
                });
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Third,
                    room: room.clone(),
                });
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Fourth,
                    room: room.clone(),
                });
                Period::Fifth
            }
            "2-4" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::First,
                    room: room.clone(),
                });
                Period::Second
            }
            "5-7" => {
                time_tables.push(TimeTable {
                    day: day.clone(),
                    period: Period::Third,
                    room: room.clone(),
                });
                Period::Fourth
            }
            _ => Period::First,
        };

        time_tables.push(TimeTable { day, period, room });
    }

    time_tables
}

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

fn get_details(overview: ElementRef) -> CourseDetail {
    let mut details = CourseDetail {
        abst: "".to_string(),
        goal: "".to_string(),
        experience: false,
        keyword: Vec::new(),
        competencies: Vec::new(),
        flow: "".to_string(),
        schedule: Vec::new(),
        out_of_class: "".to_string(),
        textbook: "".to_string(),
        reference_book: "".to_string(),
        assessment: "".to_string(),
        related_course: Vec::new(),
        prerequisite: "".to_string(),
        contact: "".to_string(),
        office_hour: "".to_string(),
        note: "".to_string(),
    };

    let div_selector = Selector::parse("div").unwrap();
    let mut divs = overview.select(&div_selector);
    let p_selector = Selector::parse("p").unwrap();
    let ul_selector = Selector::parse("ul").unwrap();
    let tbody_selector = Selector::parse("tbody").unwrap();
    let h3_selector = Selector::parse("h3").unwrap();

    details.abst = match divs.next() {
        Some(div) => get_abstract(div.select(&p_selector).next().unwrap()),
        None => "".to_string(),
    };

    details.goal = match divs.next() {
        Some(div) => get_goal(div.select(&p_selector).next().unwrap()),
        None => "".to_string(),
    };

    details.keyword = match divs.next() {
        Some(div) => get_keywords(div.select(&p_selector).next().unwrap()),
        None => Vec::new(),
    };

    details.competencies = match divs.next() {
        Some(div) => get_competencies(div),
        None => Vec::new(),
    };

    details.flow = match divs.next() {
        Some(div) => {
            let h3 = div.select(&h3_selector).next().unwrap();
            println!("{:?}", h3.inner_html());
            get_flow(div.select(&p_selector).next().unwrap())
        }
        None => "".to_string(),
    };

    details.schedule = match divs.next() {
        Some(div) => get_schedule(div.select(&tbody_selector).next().unwrap()),
        None => Vec::new(),
    };

    details.out_of_class = match divs.next() {
        Some(div) => get_out_of_class(div.select(&p_selector).next().unwrap()),
        None => "".to_string(),
    };

    details.textbook = match divs.next() {
        Some(div) => get_textbook(div.select(&p_selector).next().unwrap()),
        None => "".to_string(),
    };

    details.reference_book = match divs.next() {
        Some(div) => get_reference_book(div.select(&p_selector).next().unwrap()),
        None => "".to_string(),
    };

    details.assessment = match divs.next() {
        Some(div) => get_assessment(div.select(&p_selector).next().unwrap()),
        None => "".to_string(),
    };

    details.related_course = match divs.next() {
        Some(div) => get_related_course(div.select(&ul_selector).next().unwrap()),
        None => Vec::new(),
    };

    details.prerequisite = match divs.next() {
        Some(div) => get_prerequisite(div.select(&p_selector).next().unwrap()),
        None => "".to_string(),
    };

    details.contact = match divs.next() {
        Some(div) => get_contact(div.select(&p_selector).next().unwrap()),
        None => "".to_string(),
    };

    details.office_hour = match divs.next() {
        Some(div) => get_office_hour(div.select(&p_selector).next().unwrap()),
        None => "".to_string(),
    };

    details.note = match divs.next() {
        Some(div) => get_note(div.select(&p_selector).next().unwrap()),
        None => "".to_string(),
    };
    details
}

fn get_abstract(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_goal(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_keywords(p: ElementRef) -> Vec<String> {
    let keywords = p.inner_html().trim().to_string();
    keywords
        .as_str()
        .split(&['、', '，', ','][..])
        .map(|s| s.trim().to_string())
        .collect()
}

fn get_competencies(div: ElementRef) -> Vec<String> {
    let mut competencies = Vec::new();
    for element in div.select(&Selector::parse(".skill_checked2").unwrap()) {
        competencies.push(element.inner_html().trim().to_string());
    }

    competencies
}

fn get_flow(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_schedule(tbody: ElementRef) -> Vec<LecturePlan> {
    let mut lecture_plans = Vec::new();

    let count_re = Regex::new(r"\d+").unwrap();

    let tr_selector = Selector::parse("tr").unwrap();
    for tr in tbody.select(&tr_selector) {
        let count = tr
            .select(&Selector::parse(".number_of_times").unwrap())
            .next()
            .unwrap();
        let plan = tr
            .select(&Selector::parse(".plan").unwrap())
            .next()
            .unwrap();
        let assignment = tr
            .select(&Selector::parse(".assignment").unwrap())
            .next()
            .unwrap();

        let count = count.inner_html().trim().to_string();
        let count = count_re.find(&count).unwrap().as_str().parse().unwrap();
        lecture_plans.push(LecturePlan {
            count,
            plan: plan.inner_html().trim().to_string(),
            assignment: assignment.inner_html().trim().to_string(),
        });
    }

    lecture_plans
}

fn get_out_of_class(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_textbook(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_reference_book(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_assessment(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_related_course(ul: ElementRef) -> Vec<String> {
    let mut related_courses = Vec::new();
    for li in ul.select(&Selector::parse("li").unwrap()) {
        related_courses.push(li.inner_html().trim().to_string());
    }

    related_courses
}

fn get_prerequisite(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_contact(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_office_hour(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}

fn get_note(p: ElementRef) -> String {
    p.inner_html().trim().to_string()
}
