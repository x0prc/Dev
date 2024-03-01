DELIMITER $$ -- is done on terminal level.
CREATE 
    TRIGGER my_trigger BEFORE INSERT 
    ON employee
    FOR EACH ROW BEGIN
        INSERT INTO trigger_test VALUES('add new employee');
    END$$
DELIMITER ;

INSERT INTO employee    -- triggers enable to add or remove elements/values
VALUES(109, 'oscar', 'martinex');
SELECT * FROM trigger_test;

INSERT INTO trigger_test VALUES(NEW.first_name);
