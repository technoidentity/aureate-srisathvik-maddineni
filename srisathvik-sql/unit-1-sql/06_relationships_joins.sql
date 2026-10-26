-- Store course information separately.
CREATE TABLE IF NOT EXISTS courses (
    course_id INT PRIMARY KEY,
    course_name VARCHAR(100) NOT NULL,
    instructor VARCHAR(100) NOT NULL
);

INSERT INTO courses (course_id, course_name, instructor)
VALUES
    (201, 'Database Systems', 'Anita'),
    (202, 'Rust Programming', 'Rahul'),
    (203, 'Linux Fundamentals', 'Meera')
ON CONFLICT (course_id) DO NOTHING;

-- Connect students and courses using foreign keys.
CREATE TABLE IF NOT EXISTS enrollments (
    enrollment_id INT PRIMARY KEY,
    student_id INT NOT NULL REFERENCES students(student_id),
    course_id INT NOT NULL REFERENCES courses(course_id),
    marks INT CHECK (marks BETWEEN 0 AND 100)
);

INSERT INTO enrollments (enrollment_id, student_id, course_id, marks)
VALUES
    (301, 101, 201, 88),
    (302, 102, 202, 76),
    (303, 103, 201, 92),
    (304, 101, 203, 85)
ON CONFLICT (enrollment_id) DO NOTHING;

-- Return only students who have matching enrollments and courses.
SELECT
    s.name,
    c.course_name,
    e.marks
FROM enrollments AS e
INNER JOIN students AS s
    ON e.student_id = s.student_id
INNER JOIN courses AS c
    ON e.course_id = c.course_id
ORDER BY s.name, c.course_name;

-- Keep every student, including students without an enrollment.
SELECT
    s.name,
    c.course_name,
    e.marks
FROM students AS s
LEFT JOIN enrollments AS e
    ON s.student_id = e.student_id
LEFT JOIN courses AS c
    ON e.course_id = c.course_id
ORDER BY s.name, c.course_name;

-- Find students who have not enrolled in any course.
SELECT
    s.student_id,
    s.name
FROM students AS s
LEFT JOIN enrollments AS e
    ON s.student_id = e.student_id
WHERE e.enrollment_id IS NULL;
