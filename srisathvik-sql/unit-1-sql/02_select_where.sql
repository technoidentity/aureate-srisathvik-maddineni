-- Display all DSAI students.
SELECT *
FROM students
WHERE branch = 'DSAI';

-- Display all CSE students.
SELECT *
FROM students
WHERE branch = 'CSE';

-- Find students whose CGPA is greater than 8.0.
SELECT name, cgpa
FROM students
WHERE cgpa > 8.0;

-- Find students whose CGPA is below 8.0.
SELECT name, cgpa
FROM students
WHERE cgpa < 8.0;

-- Display the names of CSE students.
SELECT name, branch
FROM students
WHERE branch = 'CSE';

-- Find DSAI students whose CGPA is greater than 8.0.
SELECT name, cgpa
FROM students
WHERE branch = 'DSAI'
  AND cgpa > 8.0;
