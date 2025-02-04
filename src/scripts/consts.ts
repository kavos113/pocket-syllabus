export type Day = '月' | '火' | '水' | '木' | '金';
export type Period = '1' | '2' | '3' | '4' | '5';
export type DayQuery =
  | 'Monday'
  | 'Tuesday'
  | 'Wednesday'
  | 'Thursday'
  | 'Friday'
  | 'Saturday'
  | 'Sunday';
export type PeriodQuery =
  | 'First'
  | 'Second'
  | 'Third'
  | 'Fourth'
  | 'Fifth'
  | 'Sixth';
export type SemesterQuery = 'First' | 'Second' | 'Third' | 'Fourth';
export function toDayQuery(day: Day): DayQuery {
  switch (day) {
    case '月':
      return 'Monday';
    case '火':
      return 'Tuesday';
    case '水':
      return 'Wednesday';
    case '木':
      return 'Thursday';
    case '金':
      return 'Friday';
  }
}

export function toPeriodQuery(period: Period): PeriodQuery {
  switch (period) {
    case '1':
      return 'First';
    case '2':
      return 'Second';
    case '3':
      return 'Third';
    case '4':
      return 'Fourth';
    case '5':
      return 'Fifth';
  }
}

export const DAYS: Day[] = ['月', '火', '水', '木', '金'];
export const PERIODS: Period[] = ['1', '2', '3', '4', '5'];
export const GRADES = [
  '学士1年',
  '学士2年',
  '学士3年',
  '修士1年',
  '修士2年',
  '博士課程',
];
export const QUATERS = ['1Q', '2Q', '3Q', '4Q'];
export const UNIVERCITIES = { 大学を選択: ['東京工業大学', '一橋大学'] };
export const DEPARTMENTS = {
  開講元を選択: {
    東京工業大学: {
      学士課程: {
        理学院: ['数学系', '物理学系', '化学系', '地球惑星科学系'],
        工学院: [
          '機械系',
          'システム制御系',
          '電気電子系',
          '情報通信系',
          '経営工学系',
        ],
        物質理工学院: ['材料系', '応用化学系'],
        情報理工学院: ['数理・計算科学系', '情報工学系'],
        生命理工学院: ['生命理工学系'],
        '環境・社会理工学院': ['建築学系', '土木・環境工学系', '融合理工学系'],
        '工学院，物質理工学院，環境・社会理工学院共通科目': [
          '工学院，物質理工学院，環境・社会理工学院共通科目',
        ],
        教養科目群: [
          '文系教養科目',
          '英語科目',
          '第二外国語科目',
          '日本語・日本文化科目',
          '教職科目',
          'アントレプレナーシップ科目',
          '広域教養科目',
          '理工系教養科目',
        ],
      },
      大学院課程: {
        理学院: [
          '数学コース',
          '物理学コース',
          '化学コース',
          'エネルギーコース',
          'エネルギー・情報コース',
          '地球惑星科学コース',
          '地球生命コース',
        ],
        工学院: [
          '機械コース',
          'エネルギーコース',
          'エネルギー・情報コース',
          'エンジニアリングデザインコース',
          'ライフエンジニアリングコース',
          '原子核工学コース',
          'システム制御コース',
          '電気電子コース',
          '情報通信コース',
          '経営工学コース',
        ],
        物質理工学院: [
          '材料コース',
          '応用化学コース',
          'エネルギーコース',
          'エネルギー・情報コース',
          'ライフエンジニアリングコース',
          '原子核工学コース',
          '地球生命コース',
        ],
        情報理工学院: [
          '数理・計算科学コース',
          '情報工学コース',
          '知能情報コース',
          'エネルギー・情報コース',
          'ライフエンジニアリングコース',
        ],
        生命理工学院: [
          '生命理工学コース',
          'ライフエンジニアリングコース',
          '地球生命コース',
        ],
        '環境・社会理工学院': [
          '建築学コース',
          '土木工学コース',
          '融合理工学コース',
          'エンジニアリングデザインコース',
          '都市・環境学コース',
          '地球環境共創コース',
          'エネルギーコース',
          'エネルギー・情報コース',
          '原子核工学コース',
          '社会・人間科学コース',
          'イノベーション科学コース',
          '技術経営専門職学位課程',
        ],
        教養科目群: [
          '文系教養科目',
          '英語科目',
          '第二外国語科目',
          '日本語・日本文化科目',
          '教職科目',
          'アントレプレナーシップ科目',
          '広域教養科目',
          'キャリア科目',
        ],
      },
    },
  },
};
export const YEARS = {
  年度を選択: ['2024年度'],
};
