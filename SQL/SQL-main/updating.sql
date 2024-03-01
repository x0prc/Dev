SELECT * from student;

UPDATE student -- where is not compulsory.
SET major = 'Bio'
WHERE major = 'Biology'

UPDATE student  -- specifically
SET major = 'Comp Sci'
WHERE student_id = 4;

UPDATE student         -- adding OR logic
SET major = 'Biochem' 
WHERE major = 'Biooo' OR major = 'Chemistry'
