mod list;
mod sub;

pub use list::html_to_course_abstracts;
pub use sub::{html_to_course, Course, Day, Period, Semester};

#[derive(Debug)]
pub struct CourseTitle {
    pub title: String,
    pub url: String,
}

#[derive(Debug)]
pub struct Lecturer {
    pub name: String,
    pub url: String,
}
