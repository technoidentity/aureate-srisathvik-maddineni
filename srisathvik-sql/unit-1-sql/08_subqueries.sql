-- Find students whose CGPA is above the overall average.
-- The inner query returns one value, so this is a scalar subquery.
SELECT
    name,
    cgpa
FROM students
WHERE cgpa > (
    SELECT AVG(cgpa)
    FROM students
)
ORDER BY cgpa DESC;

-- Find students who scored at least 85 in any course.
-- The inner query returns several student IDs.
SELECT
    name,
    branch
FROM students
WHERE student_id IN (
    SELECT student_id
    FROM enrollments
    WHERE marks >= 85
)
ORDER BY name;

-- Find students who are not enrolled in any course.
-- The inner query refers to the current student from the outer query.
SELECT
    s.student_id,
    s.name
FROM students AS s
WHERE NOT EXISTS (
    SELECT 1
    FROM enrollments AS e
    WHERE e.student_id = s.student_id
);
