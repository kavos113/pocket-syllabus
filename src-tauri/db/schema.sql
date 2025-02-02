CREATE TABLE IF NOT EXISTS courses(
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    university VARCHAR(255) NOT NULL,
    title TEXT NOT NULL,
    english_title TEXT,
    department TEXT NOT NULL,
    lecture_type TEXT,
    code TEXT NOT NULL,
    credit INTEGER NOT NULL,
    year INTEGER NOT NULL,
    language TEXT,
    url TEXT,
    abstract TEXT,
    goal TEXT,
    experience BOOLEAN,
    flow TEXT,
    out_of_class TEXT,
    textbook TEXT,
    reference_book TEXT,
    assessment TEXT,
    prerequisite TEXT,
    contact TEXT,
    office_hour TEXT,
    note TEXT
);

CREATE TABLE IF NOT EXISTS lecturers(
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    course_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    url TEXT,
    FOREIGN KEY (course_id) REFERENCES courses(id)
);

CREATE TABLE IF NOT EXISTS timetables(
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    course_id INTEGER NOT NULL,
    day INTEGER NOT NULL,
    periods INTEGER NOT NULL,
    room TEXT NOT NULL,
    FOREIGN KEY (course_id) REFERENCES courses(id)
);

CREATE TABLE IF NOT EXISTS semesters(
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    course_id INTEGER NOT NULL,
    semester INTEGER NOT NULL,
    FOREIGN KEY (course_id) REFERENCES courses(id)
);

CREATE TABLE IF NOT EXISTS keywords(
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    course_id INTEGER NOT NULL,
    keyword TEXT NOT NULL,
    FOREIGN KEY (course_id) REFERENCES courses(id)
);

CREATE TABLE IF NOT EXISTS competencies(
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    course_id INTEGER NOT NULL,
    competency TEXT NOT NULL,
    FOREIGN KEY (course_id) REFERENCES courses(id)
);

CREATE TABLE IF NOT EXISTS related_courses(
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    course_id INTEGER NOT NULL,
    related_course_id INTEGER NOT NULL,
    FOREIGN KEY (course_id) REFERENCES courses(id),
    FOREIGN KEY (related_course_id) REFERENCES courses(id)
);