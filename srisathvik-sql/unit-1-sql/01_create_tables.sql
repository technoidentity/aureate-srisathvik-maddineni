-- Create the students table.
CREATE TABLE IF NOT EXISTS students (
    student_id INT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    branch VARCHAR(50),
    cgpa NUMERIC(3, 2)
);

-- Add sample student records.
-- ON CONFLICT prevents duplicate rows when this script is run again.
INSERT INTO students (student_id, name, branch, cgpa)
VALUES
    (101, 'Arjun', 'CSE', 8.50),
    (102, 'Ravi', 'DSAI', 7.80),
    (103, 'Priya', 'DSAI', 9.10),
    (104, 'Neha', 'CSE', 8.20)
ON CONFLICT (student_id) DO NOTHING;

-- Display all students.
SELECT *
FROM students;
