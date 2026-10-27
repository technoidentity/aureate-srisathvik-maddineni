# Relational Model Notes

A relational database stores information in tables and connects related
tables using keys.

## Main terms

A relation is a table. In this project, `students`, `courses`, and
`enrollments` are relations.

A tuple is one complete row. The row containing Arjun's information is
one tuple in the `students` relation.

An attribute is a column, such as `name`, `branch`, or `cgpa`.

A domain defines the kind of value that an attribute can store.
For example, `student_id` uses `INTEGER` and `name` uses `VARCHAR(100)`.

A schema is the structure of a table, including its columns, data types,
keys, and constraints. The current rows stored in that schema are called
the database instance.

## Keys and constraints

A primary key uniquely identifies every row. `student_id`, `course_id`,
and `enrollment_id` are primary keys in their respective tables.

A foreign key connects one table to another. The foreign keys in
`enrollments` ensure that an enrollment can only use an existing student
and an existing course.

`NOT NULL` prevents required values from being omitted.

The marks check constraint allows values only from 0 to 100.

## Relationships

One student can have many enrollment rows, and one course can also have
many enrollment rows. The `enrollments` table connects them, creating a
many-to-many relationship between students and courses.

## Relational operations used

Selection chooses rows and is written using `WHERE`.

Projection chooses columns and is written using the column list after
`SELECT`.

A join combines related rows from multiple tables.

`UNION`, `INTERSECT`, and `EXCEPT` perform set operations on compatible
query results.

## Design observation

The current `cgpa` column uses `NUMERIC(3,2)`, which can store values up
to 9.99. A system that allows a CGPA of 10.00 should instead use
`NUMERIC(4,2)` with a check constraint limiting values from 0 to 10.
