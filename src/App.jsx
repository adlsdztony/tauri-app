import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

class Section {
  section;
  session;
}

class Course {
  code;
  title;
  sections;
  prereq;
  constructor(title, code) {
    this.title = title;
    this.code = code;
    this.sections = [];
    this.prereq = "";
  }
}

class CourseMap {
  courses;
  constructor() {
    this.courses = [];
  }
}

function CourseCard({ course, key, add }) {
  //sort sections by section number
  course.sections.sort((a, b) => {
    return a.section.localeCompare(b.section);
  });

  let sections = [];
  for (const value of course.sections) {
    sections.push(
      <div
        className="section"
        onClick={() => {
          add(course.code, value.section);
        }}
      >
        <div className="section-title">{value.section}</div>
      </div>
    );
  }

  return (
    <div className="card" key={key}>
      <div className="card-body">
        <h5 className="card-title">{course.code}</h5>
        <p className="card-text">{course.title}</p>
        <div className="sections">{sections}</div>
      </div>
    </div>
  );
}

function CourseList({ courseMap, add }) {
  const courses = courseMap.courses;

  // if courseMap is empty
  if (courses.length === 0) {
    return (
      <div className="card-list">
        <p className="card-text">No courses found</p>
      </div>
    );
  }

  // sort courses by code
  courses.sort((a, b) => {
    return a.code.localeCompare(b.code);
  });

  let listItems = [];
  for (const value of courses) {
    listItems.push(<CourseCard course={value} key={value.code} add={add} />);
  }

  return <div className="card-list">{listItems}</div>;
}

function App() {
  const [courses, setCourses] = useState(new CourseMap());
  const [myCourses, setMyCourses] = useState(new CourseMap()); // [A-Z]{4}[0-9]{3}[A-Z]?
  const [pattern, setPattern] = useState(""); // [A-Z]{4}[0-9]{3}[A-Z]?
  const [conflict, setConflict] = useState(false);
  const [semester, setSemester] = useState("0"); // 0 = all, 1 = fall, 2 = winter

  async function search(conflict, semester, contains) {
    invoke("search", {
      conflict: conflict,
      semester: semester,
      contains: contains,
    }).then((result) => {
      setCourses(result);
    });
  }

  async function add(code, section) {
    invoke("add", { code: code, section: section }).then((result) => {
      setMyCourses(result);
      search(conflict, semester, pattern);
    });
  }

  async function getCourses() {
    invoke("courses").then((result) => {
      setMyCourses(result);
    });
  }

  async function clear() {
    invoke("clear").then((result) => {
      setMyCourses(result);
    });
  }

  return (
    <div className="main-body">
      <div className="course-info">
        <h2>
          <span>My Courses</span>
        </h2>

        <CourseList courseMap={myCourses} add={add} />
      </div>
      <div className="container">
        <div className="query">
          <input
            id="greet-input"
            onChange={(e) => {
              setPattern(e.currentTarget.value);
              search(conflict, semester, e.currentTarget.value);
            }}
            placeholder="Search a course"
          />

          {/* select semester */}
          <select
            id="semester"
            onChange={(e) => {
              setSemester(e.currentTarget.value);
              search(conflict, e.currentTarget.value, pattern);
            }}
            title="Select a semester"
          >
            <option value="0">All</option>
            <option value="1">Fall</option>
            <option value="2">Winter</option>
          </select>

          {/* button for conflict */}
          <button
            onClick={() => {
              setConflict(!conflict);
              search(!conflict, semester, pattern);
              // change color of button
              if (conflict) {
                document.getElementById("conflict").style.backgroundColor =
                  "#ffffff";
              } else {
                document.getElementById("conflict").style.backgroundColor =
                  "#cfcfcfb9";
              }
            }}
            id="conflict"
            title="Only show the courses without conflict"
          >
            {" "}
            Conflict{" "}
          </button>

          {/* button to clear courses */}
          <button
            onClick={() => {
              clear();
              search(conflict, semester, pattern);
            }}
            id="clear"
            title="Clear My Courses"
          >
            {" "}
            Clear{" "}
          </button>
        </div>
        <CourseList courseMap={courses} add={add} />
      </div>
    </div>
  );
}

export default App;
