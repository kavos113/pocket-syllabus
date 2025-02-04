export interface Course {
  id: number;
  university: string;
  title: string;
  englishTitle: string;
  department: string;
  lecturer: Lecturer[];
  lectureType: string;
  timeTable: TimeTable[];
  code: string;
  credit: number;
  year: number;
  semester: number;
  language: string;
  courseDetail: CourseDetail;
  url: string;
}

export interface Lecturer {
  id: number;
  name: string;
  url: string;
}

export interface TimeTable {
  courseId: number | undefined;
  dayOfWeek: number;
  period: number;
  room: string | undefined;
}

export interface CourseDetail {
  abstract: string;
  goal: string;
  experience: boolean;
  keyword: string[];
  competencies: string[];
  flow: string;
  schedule: string;
  outOfClass: string;
  textbook: string;
  referenceBook: string;
  assessment: string;
  relatedCourses: string;
  prerequisite: string;
  note: string;
}

export interface CourseListItem {
  id: number;
  university: string;
  code: string;
  title: string;
  lecturer: string;
  timetable: string;
  semester: string;
  department: string;
  credit: number;
}

export const getSampleItems = (numSamples: number): CourseListItem[] => {
  const items: CourseListItem[] = [];
  for (let i = 0; i < numSamples; i++) {
    items.push({
      id: i,
      university: `大学${i}`,
      code: `000${i}`,
      title: `コース${i}`,
      lecturer: `講師${i}`,
      timetable: `月${i}, 木${i}`,
      semester: '2024前期',
      department: `学科${i}`,
      credit: 2,
    });
  }
  return items;
};
