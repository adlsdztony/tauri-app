import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

// create structure for Course
class Course {
  code;
  title;
  sections;
  prereq;
  constructor(title, code) {
    this.title = title;
    this.code = code;
    this.sections = new Map();
    this.prereq = "";
  }
}

// create structure for Courses
class CourseMap {
  courses;
  constructor() {
    this.courses = [];
  }
}

function CourseCard({ course }) {
  return (
    <div className="card">
      <div className="card-body">
        <h5 className="card-title">{course.code}</h5>
        <p className="card-text">{course.title}</p>
      </div>
    </div>
  );
}

function CourseList({ courseMap }) {
  console.log(courseMap);
  const courses = courseMap.courses;
  
  console.log(courses);

  let listItems = [];
  for (const value of courses) {
    listItems.push(<CourseCard course={value} key={value.code} />);
  }
  
  return <div className="courseList">{listItems}</div>;
}

function App() {
  const [name, setName] = useState("");
  const [courses, setCourses] = useState(new CourseMap());

  async function search(contains) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    invoke("search", { conflict: true, semester: 1, contains: contains }).then(
      (result) => {
        setCourses(result);
      }
    );
  }

  return (
    <div className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          search();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => { 
            setName(e.currentTarget.value)
            search(e.currentTarget.value);
          }}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <CourseList courseMap={courses} />
    </div>
  );
}

export default App;
