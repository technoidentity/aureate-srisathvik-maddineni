-- Start a transaction so the practice changes can be cancelled.
BEGIN;

-- Add a new student.
INSERT INTO students (student_id, name, branch, cgpa)
VALUES (105, 'Kiran', 'ECE', 8.10);

SELECT *
FROM students
WHERE student_id = 105;

-- Change the student's CGPA.
-- WHERE prevents every student from being updated.
UPDATE students
SET cgpa = 8.30
WHERE student_id = 105
RETURNING *;

-- Remove only the practice student.
-- WHERE prevents every student from being deleted.
DELETE FROM students
WHERE student_id = 105
RETURNING *;

-- Cancel every change made after BEGIN.
ROLLBACK;

-- Confirm that the original data remains unchanged.
SELECT COUNT(*) AS total_students
FROM students;

SELECT *
FROM students
WHERE student_id = 105;
