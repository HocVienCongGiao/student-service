CREATE TABLE IF NOT EXISTS public.student__student
(
    id                 UUID PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS public.student__student_polity
(
    id                 UUID PRIMARY KEY REFERENCES student__student(id),
    polity_id          UUID NOT NULL REFERENCES polity__polity(id)
);

CREATE TABLE IF NOT EXISTS public.student__student_title
(
    id                 UUID PRIMARY KEY REFERENCES student__student(id),
    title              VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS public.student__student_christian_names
(
    student_id         UUID NOT NULL REFERENCES student__student(id),
    saint_id           UUID NOT NULL REFERENCES saint__saint(id),
    ordering           SMALLINT NOT NULL,
    PRIMARY KEY(student_id, saint_id)
);

CREATE TABLE IF NOT EXISTS public.student__student_first_name
(
    id                 UUID PRIMARY KEY REFERENCES student__student(id),
    first_name         VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS public.student__student_middle_name
(
    id                 UUID PRIMARY KEY REFERENCES student__student(id),
    middle_name        VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS public.student__student_last_name
(
    id                 UUID PRIMARY KEY REFERENCES student__student(id),
    last_name          VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS public.student__student_date_of_birth
(
    id                 UUID PRIMARY KEY REFERENCES student__student(id),
    date_of_birth      DATE NOT NULL
);

CREATE TABLE IF NOT EXISTS public.student__student_place_of_birth
(
    id                 UUID PRIMARY KEY REFERENCES student__student(id),
    place_of_birth     VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS public.student__student_email
(
    id                 UUID PRIMARY KEY REFERENCES student__student(id),
    email              VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS public.student__student_phone
(
    id                 UUID PRIMARY KEY REFERENCES student__student(id),
    phone              VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS public.student__student_undergraduate_school_name
(
    id                 UUID PRIMARY KEY REFERENCES student__student(id),
    school_name        VARCHAR NOT NULL
);

-- Student: Nguyễn Hữu Chiến
INSERT INTO public.student__student (id)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283');

INSERT INTO public.student__student_polity (id, polity_id)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283', '4d084b56-54e1-4bd2-878e-c52675497c2b');

INSERT INTO public.student__student_christian_names (student_id, saint_id, ordering)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283', '40e6215d-b5c6-4896-987c-f30f3678f608', 1);

INSERT INTO public.student__student_title (id, title)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283', 'PRIEST');

INSERT INTO public.student__student_first_name (id, first_name)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283', 'Nguyễn');

INSERT INTO public.student__student_middle_name (id, middle_name)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283', 'Hữu');

INSERT INTO public.student__student_last_name (id, last_name)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283', 'Chiến');

INSERT INTO public.student__student_date_of_birth (id, date_of_birth)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283', '1983-05-16');

INSERT INTO public.student__student_place_of_birth (id, place_of_birth)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283', 'Trà Vinh');

INSERT INTO public.student__student_email (id, email)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283', 'binh@sunrise.vn');

INSERT INTO public.student__student_phone (id, phone)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283', '+84 1228019700');

INSERT INTO public.student__student_undergraduate_school_name (id, school_name)
VALUES ('53f549b9-99bf-4e12-88e3-c2f868953283', 'Đại Chủng Viện Thánh Quý - Cần Thơ');

-- View
CREATE VIEW student__student_christian_name_view AS
    SELECT student.id as student_id, string_agg(saint__saint_display_name.display_name, ' ' ORDER BY student__student_christian_names.ordering) as christian_name       
    FROM student__student student
    LEFT JOIN student__student_christian_names ON student.id = student__student_christian_names.student_id
    LEFT JOIN saint__saint_display_name ON student__student_christian_names.saint_id = saint__saint_display_name.id
    GROUP BY student.id;

CREATE VIEW student__student_view AS
    SELECT student.*, 
    student__student_title.title,
    student__student_christian_name_view.christian_name,

    student__student_first_name.first_name,
    student__student_middle_name.middle_name,
    student__student_last_name.last_name,
    student__student_date_of_birth.date_of_birth,
    student__student_place_of_birth.place_of_birth,
    student__student_email.email,
    student__student_phone.phone,
    student__student_undergraduate_school_name.school_name undergraduate_school_name,

    student__student_polity.polity_id,
    polity.name polity_name,
    polity.location_address polity_location_address,
    polity.location_name polity_location_name,
    polity.location_email polity_location_email
    FROM student__student student
    LEFT JOIN student__student_title ON student.id = student__student_title.id
    LEFT JOIN student__student_first_name ON student.id = student__student_first_name.id
    LEFT JOIN student__student_middle_name ON student.id = student__student_middle_name.id
    LEFT JOIN student__student_last_name ON student.id = student__student_last_name.id
    LEFT JOIN student__student_date_of_birth ON student.id = student__student_date_of_birth.id
    LEFT JOIN student__student_place_of_birth ON student.id = student__student_place_of_birth.id
    LEFT JOIN student__student_email ON student.id = student__student_email.id
    LEFT JOIN student__student_phone ON student.id = student__student_phone.id
    LEFT JOIN student__student_undergraduate_school_name ON student.id = student__student_undergraduate_school_name.id

    LEFT JOIN student__student_christian_name_view ON student.id = student__student_christian_name_view.student_id

    LEFT JOIN student__student_polity ON student.id = student__student_polity.id
    LEFT JOIN polity__polity_view polity ON student__student_polity.polity_id = polity.id