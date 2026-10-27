# SQL Unit 1 Practice

This folder contains my introductory PostgreSQL practice. The exercises
use a small student and course database to demonstrate relational
modeling and essential SQL operations.

## Database Structure

The database contains three related tables:

- `students` stores student details.
- `courses` stores course details.
- `enrollments` connects students and courses and stores marks.

A student can join multiple courses, and a course can contain multiple
students. The `enrollments` table represents this many-to-many
relationship.

## Concepts Practised

- Creating tables and inserting records
- Primary keys, foreign keys, and check constraints
- Selecting columns and filtering rows
- Sorting, limiting, and removing duplicate results
- `BETWEEN`, `IN`, `LIKE`, `IS NULL`, and `CASE`
- Aggregate functions such as `COUNT`, `SUM`, `AVG`, `MIN`, and `MAX`
- Grouping data with `GROUP BY` and filtering groups with `HAVING`
- Inner joins and left joins
- Finding rows without related records
- Transactions using `BEGIN` and `ROLLBACK`
- Adding, updating, and deleting data
- Scalar, list, and correlated subqueries
- `UNION`, `UNION ALL`, `INTERSECT`, and `EXCEPT`
- Relational-model terms and table relationships

## Files

- `01_create_tables.sql` creates the students table and sample records.
- `02_select_where.sql` demonstrates basic selection and filtering.
- `03_filtering_sorting.sql` demonstrates sorting and combined conditions.
- `04_advanced_filtering.sql` covers advanced filters and `CASE`.
- `05_aggregates_grouping.sql` covers aggregates, grouping, and `HAVING`.
- `06_relationships_joins.sql` creates related tables and demonstrates joins.
- `07_data_modification.sql` demonstrates inserts, updates, deletes, and rollback.
- `08_subqueries.sql` demonstrates three forms of subqueries.
- `09_set_operations.sql` demonstrates SQL set operations.
- `10_relational_model.md` contains notes about relational database concepts.

## Running the Exercises

A PostgreSQL server must be running, and the `p5db` database must exist.

Run the SQL files in numerical order:

```bash
cd ~/aureate/srisathvik-sql/unit-1-sql

psql -d p5db -f 01_create_tables.sql
psql -d p5db -f 02_select_where.sql
psql -d p5db -f 03_filtering_sorting.sql
psql -d p5db -f 04_advanced_filtering.sql
psql -d p5db -f 05_aggregates_grouping.sql
psql -d p5db -f 06_relationships_joins.sql
psql -d p5db -f 07_data_modification.sql
psql -d p5db -f 08_subqueries.sql
psql -d p5db -f 09_set_operations.sql
```

`10_relational_model.md` contains documentation and should not be executed
with `psql`.

## Validation

The exercises were run using PostgreSQL 17. The following behavior was
verified:

- Foreign keys reject enrollments for missing students or courses.
- The marks constraint rejects values outside 0 to 100.
- Inner joins return only matching records.
- Left joins retain students without enrollments.
- Transaction rollback restores the original data.
- Subqueries and set operations return the expected results.
