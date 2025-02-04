<script setup lang="ts">
import { Course } from '../../scripts/course.ts';

const props = defineProps<{
  content: Course;
}>();

const splitItems = (item: string) => {
  return item.split('<br>');
};
</script>

<template>
  <div>
    <div class="wrapper">
      <div class="title">
        <p class="jaTitle">{{ props.content.title }}</p>
        <p class="enTitle">{{ props.content.englishTitle }}</p>
      </div>
      <div class="data">
        <dl class="dataItem">
          <dt>開講元</dt>
          <dd>{{ props.content.department }}</dd>
        </dl>
        <dl class="dataItem">
          <dt>担当教員</dt>
          <dd>
            <a
              v-for="lecturer in props.content.lecturer"
              :key="lecturer.id"
              :href="lecturer.url"
              class="lecturerText"
            >
              {{ lecturer.name }}
            </a>
          </dd>
        </dl>
        <dl class="dataItem">
          <dt>授業形態</dt>
          <dd>{{ props.content.lectureType }}</dd>
        </dl>
      </div>
      <dl class="dataItem">
        <dt>曜日・時限(講義室)</dt>
        <dd>
          <span
            v-for="timetable in props.content.timeTable"
            :key="`${timetable.dayOfWeek}${timetable.period}`"
          >
            {{ timetable.dayOfWeek }}{{ timetable.period }}限({{
              timetable.room
            }})
          </span>
        </dd>
      </dl>
      <dl class="dataItem">
        <dt>開講時期</dt>
        <dd>
          {{ props.content.year }}年
          <span
            v-for="semester in props.content.semester"
            :key="semester"
          >
            {{ semester }}Q
          </span>
        </dd>
      </dl>
      <div class="dataItem">
        <dl class="dataItemChild">
          <dt>科目コード</dt>
          <dd>{{ props.content.code }}</dd>
        </dl>
        <dl class="dataItemChild">
          <dt>単位数</dt>
          <dd>{{ props.content.credit }}</dd>
        </dl>
      </div>
      <div class="dataItem">
        <dl class="dataItemChild">
          <dt>シラバス更新日</dt>
          <dd>{{ props.content.sylbs_update }}</dd>
        </dl>
        <dl class="dataItemChild">
          <dt>言語</dt>
          <dd>{{ props.content.language }}</dd>
        </dl>
      </div>
      <div class="details">
        <div class="detailItem">
          <h3>講義の概要とねらい</h3>
          <p v-for="abst in splitItems(props.content.courseDetail.abst)">
            {{ abst }}
          </p>
        </div>
        <div class="detailItem">
          <h3>授業計画・課題</h3>
          <table class="plans">
            <thead>
              <tr>
                <th>回</th>
                <th>授業計画</th>
                <th>課題</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="plan in props.content.courseDetail.schedule">
                <td class="count">第{{ plan.count }}回</td>
                <td>{{ plan.plan }}</td>
                <td>{{ plan.assignment }}</td>
              </tr>
            </tbody>
          </table>
        </div>
        <div class="detailItem">
          <h3>到達目標</h3>
          <p v-for="goal in splitItems(props.content.courseDetail.goal)">
            {{ goal }}
          </p>
        </div>
        <div
          class="detailItem"
          v-if="props.content.courseDetail.experience"
        >
          <h3>実務経験のある教員による授業</h3>
          <p>あり</p>
        </div>
        <div class="detailItem">
          <h3>キーワード</h3>
          <p>
            {{ props.content.courseDetail.keyword.join(', ') }}
          </p>
        </div>
        <div class="detailItem">
          <h3>教科書</h3>
          <ul class="reference">
            <li
              v-for="textbook in splitItems(
                props.content.courseDetail.textbook,
              )"
            >
              {{ textbook }}
            </li>
          </ul>
        </div>
        <div class="detailItem">
          <h3>参考書・講義資料等</h3>
          <ul class="reference">
            <li
              v-for="reference in splitItems(
                props.content.courseDetail.referenceBook,
              )"
            >
              {{ reference }}
            </li>
          </ul>
        </div>
        <div class="detailItem">
          <h3>学生が身につける力(ディグリー・ポリシー)</h3>
          <table class="skill">
            <tbody>
              <tr>
                <td class="skill_checked">専門力</td>
                <td class="skill_checked">教養力</td>
                <td class="skill_checked">コミュニケーション力</td>
                <td class="skill_checked">展開力(探究力又は設定力)</td>
                <td class="skill_checked">展開力(実践力又は解決力)</td>
              </tr>
            </tbody>
          </table>
        </div>
        <div class="detailItem">
          <h3>授業の進め方</h3>
          <p v-for="flow in splitItems(props.content.courseDetail.flow)">
            {{ flow }}
          </p>
        </div>

        <div class="detailItem">
          <h3>授業時間外学修（予習・復習等）</h3>
          <p
            v-for="outOfClass in splitItems(
              props.content.courseDetail.outOfClass,
            )"
          >
            {{ outOfClass }}
          </p>
        </div>
        <div class="detailItem">
          <h3>成績評価の基準及び方法</h3>
          <p
            v-for="assessment in splitItems(
              props.content.courseDetail.assessment,
            )"
          >
            {{ assessment }}
          </p>
        </div>
        <div class="detailItem">
          <h3>関連する科目</h3>
          <ul class="related">
            <li v-for="related in props.content.courseDetail.relatedCourse">
              {{ related }}
            </li>
          </ul>
        </div>
        <div class="detailItem">
          <h3>履修の条件</h3>
          <p
            v-for="prerequisite in splitItems(
              props.content.courseDetail.prerequisite,
            )"
          >
            {{ prerequisite }}
          </p>
        </div>
        <div class="detailItem">
          <h3>その他</h3>
          <p v-for="note in splitItems(props.content.courseDetail.note)">
            {{ note }}
          </p>
        </div>
        <div class="detailItem">
          <h3>連絡先</h3>
          <p v-for="contact in splitItems(props.content.courseDetail.contact)">
            {{ contact }}
          </p>
        </div>
        <div class="detailItem">
          <h3>オフィスアワー</h3>
          <p
            v-for="officeHour in splitItems(
              props.content.courseDetail.officeHour,
            )"
          >
            {{ officeHour }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.wrapper {
  padding: 1rem 2rem;
  font-size: 0.8rem;
}

.title {
  margin: 0 0 1rem;
  border-bottom: 4px solid var(--stmain);
  padding: 1rem 0 0.5rem;
  position: relative;
}

.title:before {
  content: '';
  border-bottom: 2px solid var(--stsub);
  position: absolute;
  top: calc(100% + 6px);
  left: 0;
  right: 0;
}

.jaTitle {
  font-size: 2rem;
  padding-top: 1rem;
  margin: 0;
}

.enTitle {
  margin: 0;
}

.data {
  display: flex;
  flex-direction: column;
  margin-bottom: 1rem;
}

.dataItem {
  display: flex;
  flex-direction: row;
  margin: 0 0 0.2rem;
  width: 100%;
}

.dataItem dt {
  width: 140px;
  font-weight: bold;
}

.dataItemChild {
  display: flex;
  flex-direction: row;
  width: 50%;
  margin: 0;
}

.lecturerText {
  margin-right: 0.5rem;
}

.detailItem {
  font-size: 0.8rem;
  margin-bottom: 2rem;
  margin-top: 2rem;
}

.detailItem h3 {
  font-size: 1.3rem;
  font-weight: normal;
  border-bottom: 2px solid var(--stsub);
  margin: 1rem 0 0.5rem;
}

.detailItem p {
  margin: 0;
}

.plans {
  width: 100%;
  border-collapse: collapse;
}

.plans th {
  border: 1px solid var(--ablack);
  padding: 0.2rem 0.5rem;
  background-color: var(--background);
}

.plans td {
  border: 1px solid var(--ablack);
  padding: 0.2rem 0.5rem;
}

.count {
  text-align: center;
}

.reference {
  list-style: none;
  padding: 0;
}

.related {
  list-style: none;
  padding: 0;
}
</style>
