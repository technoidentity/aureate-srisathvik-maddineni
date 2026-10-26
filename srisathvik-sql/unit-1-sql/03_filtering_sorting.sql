-- Sort students by CGPA from highest to lowest.
SELECT name, cgpa
FROM students
ORDER BY cgpa DESC;

-- Sort students by CGPA from lowest to highest.
SELECT name, cgpa
FROM students
ORDER BY cgpa ASC;

-- Find students from either CSE or DSAI with a CGPA of at least 8.5.
SELECT name, branch, cgpa
FROM students
WHERE branch IN ('CSE', 'DSAI')
  AND cgpa >= 8.5;
