-- Count all students.
SELECT COUNT(*) AS total_students
FROM students;

-- Count students whose CGPA is greater than 8.0.
SELECT COUNT(*) AS students_above_8
FROM students
WHERE cgpa > 8.0;

-- Calculate the total and average CGPA.
SELECT
    SUM(cgpa) AS total_cgpa,
    ROUND(AVG(cgpa), 2) AS average_cgpa
FROM students;

-- Find the lowest and highest CGPA.
SELECT
    MIN(cgpa) AS lowest_cgpa,
    MAX(cgpa) AS highest_cgpa
FROM students;

-- Count the students in each branch.
SELECT
    branch,
    COUNT(*) AS student_count
FROM students
GROUP BY branch
ORDER BY branch;

-- Calculate the average CGPA for each branch.
SELECT
    branch,
    ROUND(AVG(cgpa), 2) AS average_cgpa
FROM students
GROUP BY branch
ORDER BY branch;

-- Keep only branches whose average CGPA is at least 8.4.
SELECT
    branch,
    ROUND(AVG(cgpa), 2) AS average_cgpa
FROM students
GROUP BY branch
HAVING AVG(cgpa) >= 8.4
ORDER BY branch;
