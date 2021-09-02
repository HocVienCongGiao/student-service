-- Program Specialism Instance
CREATE TABLE IF NOT EXISTS public.enrolment__specialism_instance
(
    id                 UUID PRIMARY KEY,
    specialism_id      UUID NOT NULL REFERENCES course__program_specialism(id),
    code               VARCHAR NOT NULL,
    UNIQUE (specialism_id, code)
);

CREATE TABLE IF NOT EXISTS public.enrolment__specialism_instance_progresses
(
    id                      UUID PRIMARY KEY,
    specialism_instance_id  UUID NOT NULL REFERENCES enrolment__specialism_instance(id),
    level                   SMALLINT NOT NULL,
    UNIQUE (specialism_instance_id, level)
);

CREATE TABLE IF NOT EXISTS public.enrolment__specialism_instance_progress_school_year
(
    id                  UUID PRIMARY KEY REFERENCES enrolment__specialism_instance_progresses(id),
    school_year         SMALLINT NOT NULL
);

-- Course Instance
CREATE TABLE IF NOT EXISTS public.enrolment__course_instance
(
    id                              UUID PRIMARY KEY,
    course_id                       UUID NOT NULL REFERENCES course__course(id),
    specialism_instance_progress_id UUID NOT NULL REFERENCES enrolment__specialism_instance_progresses(id),
    UNIQUE (course_id, specialism_instance_progress_id)
);

CREATE TABLE IF NOT EXISTS public.enrolment__course_instance_school_year
(
    id                 UUID PRIMARY KEY REFERENCES enrolment__course_instance(id),
    school_year        SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS public.enrolment__course_instance_semester
(
    id                 UUID PRIMARY KEY REFERENCES enrolment__course_instance(id),
    semester           SMALLINT NOT NULL
);

-- Student Enrolment
CREATE TABLE IF NOT EXISTS public.enrolment__students_specialisms
(
    id                      UUID PRIMARY KEY,
    student_id              UUID NOT NULL REFERENCES student__student(id),
    specialism_instance_progress_id  UUID NOT NULL REFERENCES enrolment__specialism_instance_progresses(id),
    UNIQUE (student_id, specialism_instance_progress_id)
);

CREATE TABLE IF NOT EXISTS public.enrolment__students_courses
(
    id                      UUID PRIMARY KEY,
    student_id              UUID NOT NULL REFERENCES student__student(id),
    course_instance_id      UUID NOT NULL REFERENCES enrolment__course_instance(id),
    UNIQUE (student_id, course_instance_id)
);

-- Program: STL - Specialism: Tín Lý 
INSERT INTO public.enrolment__specialism_instance (id, specialism_id, code)
VALUES ('118f832d-2f4a-4ab4-af3e-49bcbf14028f', '4eb07b8e-33dc-4e15-85b5-b6024613df20', 'STL-TL-K1');

-- Program: STL - Specialism: Tín Lý - Level: 0
INSERT INTO public.enrolment__specialism_instance_progresses (id, specialism_instance_id, level)
VALUES ('c93d5c74-04d7-4607-b6bf-4f121065ae9e', '118f832d-2f4a-4ab4-af3e-49bcbf14028f', 0);

-- Program: STL - Specialism: Tín Lý - Level: 0 - SchoolYear: 2016
INSERT INTO public.enrolment__specialism_instance_progress_school_year (id, school_year)
VALUES ('c93d5c74-04d7-4607-b6bf-4f121065ae9e', 2016);

-- Program: STL - Specialism: Tín Lý - Level: 0 - SchoolYear: 2016 - Student: Nguyễn Hữu Chiến
INSERT INTO public.enrolment__students_specialisms (id, student_id, specialism_instance_progress_id)
VALUES ('7fdcd5d6-e952-4725-a64f-77d2803db352', '53f549b9-99bf-4e12-88e3-c2f868953283', 'c93d5c74-04d7-4607-b6bf-4f121065ae9e');

-- View
CREATE VIEW enrolment__student_specialism_enrolment_view AS
    SELECT ss.id student_specialism_instance_id, ss.student_id, ss.specialism_instance_progress_id progress_id, progress.level,
    course__enrolable_name.name  specialism_name,
    student.title as student_title,
    student.christian_name, student.first_name, student.middle_name, student.last_name,
    student.date_of_birth, student.place_of_birth, student.undergraduate_school_name, student.email, student.phone,
    student.polity_name, student.polity_location_name, student.polity_location_address, student.polity_location_email
    FROM enrolment__students_specialisms ss
    LEFT JOIN student__student_view student ON ss.student_id = student.id

    LEFT JOIN enrolment__specialism_instance_progresses progress ON ss.specialism_instance_progress_id = progress.id
    LEFT JOIN enrolment__specialism_instance specialism_instance ON progress.specialism_instance_id = specialism_instance.id
    LEFT JOIN course__enrolable specialism ON  specialism_instance.specialism_id = specialism.id
    LEFT JOIN course__enrolable_name ON specialism.id = course__enrolable_name.id;
