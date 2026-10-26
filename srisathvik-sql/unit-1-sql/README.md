# SQL Unit 1 Practice

This folder contains my introductory PostgreSQL practice.

## Database

The exercises use a `students` table containing:

- Student ID
- Name
- Branch
- CGPA

## Concepts Practiced

- Creating a table
- Inserting records
- Selecting all rows
- Selecting specific columns
- Filtering rows using `WHERE`
- Comparison operators such as `>`, `<`, and `>=`
- Combining conditions using `AND`
- Matching multiple values using `IN`
- Sorting results using `ORDER BY`
- Ascending and descending sorting

## Files

- `01_create_tables.sql` creates the table and inserts sample data.
- `02_select_where.sql` contains basic selection and filtering queries.
- `03_filtering_sorting.sql` contains sorting and multiple-condition queries.

## Running the Exercises

Run the files in order:

```bash
psql -d p5db -f 01_create_tables.sql
psql -d p5db -f 02_select_where.sql
psql -d p5db -f 03_filtering_sorting.sql
