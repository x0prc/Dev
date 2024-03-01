CREATE TABLE student (
    student_id INT PRIMARY KEY, -- columns
    name VARCHAR(20) NOT NULL, -- cannot insert null for this
    major VARCHAR(20) UNIQUE, 
    major VARCHAR(20) DEFAULT undecided, -- no duplicates allowed, remains undecided for default
    PRIMARY KEY(student_id)

);

SELECT * FROM student;
INSERT INTO student(student_id, name) VALUES(1, 'Jack', 'Biology'); -- cannot insert duplicate entries
INSERT INTO student VALUES (2, 'Ed', 'Chemistry');
INSERT INTO student(name,major) VALUES('jack', 'baller');





DESCRIBE student; -- describes everyhing about the table

ALTER TABLE student ADD gpa DECIMAL(); -- add/alter values

DROP TABLE student; -- deletes the table
