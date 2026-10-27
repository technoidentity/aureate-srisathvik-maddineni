-- UNION combines both results and removes duplicates.
SELECT name
FROM students
WHERE branch = 'CSE'

UNION

SELECT name
FROM students
WHERE cgpa >= 8.50

ORDER BY name;

-- UNION ALL combines both results and keeps duplicates.
SELECT name
FROM students
WHERE branch = 'CSE'

UNION ALL

SELECT name
FROM students
WHERE cgpa >= 8.50

ORDER BY name;

-- INTERSECT returns rows present in both results.
SELECT name
FROM students
WHERE branch = 'CSE'

INTERSECT

SELECT name
FROM students
WHERE cgpa >= 8.50

ORDER BY name;

-- EXCEPT returns rows from the first result
-- that do not appear in the second result.
SELECT name
FROM students
WHERE branch = 'CSE'

EXCEPT

SELECT name
FROM students
WHERE cgpa >= 8.50

ORDER BY name;
