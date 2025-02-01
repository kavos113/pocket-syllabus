mod list;
mod sub;

pub use list::html_to_course_abstracts;
pub use sub::html_to_course;

#[derive(Debug)]
pub struct CourseTitle {
    title: String,
    url: String,
}

#[derive(Debug)]
pub struct Lecturer {
    name: String,
    url: String,
}
