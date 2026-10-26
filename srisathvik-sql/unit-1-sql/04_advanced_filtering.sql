-- Find students whose CGPA is between 8.0 and 9.0.
-- BETWEEN includes both boundary values.
SELECT name, cgpa
FROM students
WHERE cgpa BETWEEN 8.0 AND 9.0;

-- Find CSE students or students whose CGPA is above 9.0.
SELECT name, branch, cgpa
FROM students
WHERE branch = 'CSE'
   OR cgpa > 9.0;

-- Find students who are not from CSE.
SELECT name, branch
FROM students
WHERE NOT branch = 'CSE';

-- Display each branch only once.
SELECT DISTINCT branch
FROM students
ORDER BY branch;

-- Find names beginning with the letter P.
SELECT name
FROM students
WHERE name LIKE 'P%';

-- Find rows where the branch contains a value.
SELECT name, branch
FROM students
WHERE branch IS NOT NULL;

-- Display only the first two students after sorting by CGPA.
SELECT name, cgpa
FROM students
ORDER BY cgpa DESC
LIMIT 2;

-- Classify students based on their CGPA.
SELECT
    name,
    cgpa,
    CASE
        WHEN cgpa >= 9.0 THEN 'Excellent'
        WHEN cgpa >= 8.0 THEN 'Good'
        ELSE 'Needs improvement'
    END AS performance
FROM students
ORDER BY cgpa DESC;
