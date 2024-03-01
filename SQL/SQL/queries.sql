SELECT student.name, student.major
FROM student;
ORDER BY name DESC; -- descending/ascending order
ORDER BY major, student_id; -- order by categories

LIMIT 2; -- sets a limit of 2 rows.

WHERE major = 'Chemistry' OR name = 'Kate';

-- <, >, <=, >=, =, <> (not equal to), AND, OR

WHERE student_id < 3 AND name <> 'Jack';

WHERE name IN('Claire', 'Kanye') -- can select from these constraints.
