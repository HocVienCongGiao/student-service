CREATE TABLE IF NOT EXISTS public.course__enrolable
(
    id                 UUID PRIMARY KEY,
    type               VARCHAR NOT NULL
);

-- type: program
CREATE TABLE IF NOT EXISTS public.course__program
(
    id                 UUID PRIMARY KEY REFERENCES course__enrolable(id)
);

-- type: specialism
CREATE TABLE IF NOT EXISTS public.course__program_specialism
(
    id                 UUID PRIMARY KEY REFERENCES course__enrolable(id),
    program_id         UUID NOT NULL REFERENCES course__program(id)
);

-- type: course
CREATE TABLE IF NOT EXISTS public.course__course
(
    id                  UUID PRIMARY KEY REFERENCES course__enrolable(id)
);

-- course__enrolable_ properties
CREATE TABLE IF NOT EXISTS public.course__enrolable_code
(
    id                 UUID PRIMARY KEY REFERENCES course__enrolable(id),
    code               VARCHAR NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS IDX_course__course__enrolable_code ON course__enrolable_code (code);

CREATE TABLE IF NOT EXISTS public.course__enrolable_name
(
    id                 UUID PRIMARY KEY REFERENCES course__enrolable(id),
    name               VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS public.course__enrolable_ects
(
    id                 UUID PRIMARY KEY REFERENCES course__enrolable(id),
    ects               SMALLINT NOT NULL
);

-- N-N cardinality between course and program_specialism 
CREATE TABLE IF NOT EXISTS public.course__course_program_specialism
(
    id                  UUID PRIMARY KEY,
    course_id           UUID NOT NULL REFERENCES course__course(id),
    specialism_id       UUID NOT NULL REFERENCES course__program_specialism(id)
);

-- Program: Cử Nhân Thần Học
INSERT INTO public.course__enrolable (id, type)
VALUES ('d7513466-b2a9-4ae7-8f58-4599f414f14c', 'program');

INSERT INTO public.course__enrolable_name (id, name)
VALUES ('d7513466-b2a9-4ae7-8f58-4599f414f14c', 'Cử Nhân Thần Học');

INSERT INTO public.course__enrolable_code (id, code)
VALUES ('d7513466-b2a9-4ae7-8f58-4599f414f14c', 'STB');

INSERT INTO public.course__program (id)
VALUES ('d7513466-b2a9-4ae7-8f58-4599f414f14c');

--- Program: Cử Nhân Thần Học - general
INSERT INTO public.course__enrolable (id, type)
VALUES ('ed723833-e3ae-4899-a64b-e7bc7df59610', 'specialism');

INSERT INTO public.course__enrolable_name (id, name)
VALUES ('ed723833-e3ae-4899-a64b-e7bc7df59610', 'Cử Nhân Thần Học');

INSERT INTO public.course__enrolable_code (id, code)
VALUES ('ed723833-e3ae-4899-a64b-e7bc7df59610', 'STB-TH');

INSERT INTO public.course__program_specialism (id, program_id)
VALUES ('ed723833-e3ae-4899-a64b-e7bc7df59610', 'd7513466-b2a9-4ae7-8f58-4599f414f14c');

-- Program: Thạc Sĩ Thần Học
INSERT INTO public.course__enrolable (id, type)
VALUES ('be738e71-0023-40f6-a3e4-7e2a5bde0a75', 'program');

INSERT INTO public.course__enrolable_name (id, name)
VALUES ('be738e71-0023-40f6-a3e4-7e2a5bde0a75', 'Thạc Sĩ Thần Học');

INSERT INTO public.course__enrolable_code (id, code)
VALUES ('be738e71-0023-40f6-a3e4-7e2a5bde0a75', 'STL');

INSERT INTO public.course__program (id)
VALUES ('be738e71-0023-40f6-a3e4-7e2a5bde0a75');

-- Specialism: Thạc Sĩ Thần Học - Thánh Kinh
INSERT INTO public.course__enrolable (id, type)
VALUES ('90205738-a4d5-4c9f-8cab-9b7a6b2da4ed', 'specialism');

INSERT INTO public.course__enrolable_name (id, name)
VALUES ('90205738-a4d5-4c9f-8cab-9b7a6b2da4ed', 'Thạc Sĩ Thần Học Thánh Kinh');

INSERT INTO public.course__enrolable_code (id, code)
VALUES ('90205738-a4d5-4c9f-8cab-9b7a6b2da4ed', 'STL-TK');

INSERT INTO public.course__program_specialism (id, program_id)
VALUES ('90205738-a4d5-4c9f-8cab-9b7a6b2da4ed', 'be738e71-0023-40f6-a3e4-7e2a5bde0a75');

-- Specialism: Thạc Sĩ Thần Học - Tín Lý
INSERT INTO public.course__enrolable (id, type)
VALUES ('4eb07b8e-33dc-4e15-85b5-b6024613df20', 'specialism');

INSERT INTO public.course__enrolable_name (id, name)
VALUES ('4eb07b8e-33dc-4e15-85b5-b6024613df20', 'Thạc Sĩ Thần Học Tín Lý');

INSERT INTO public.course__enrolable_code (id, code)
VALUES ('4eb07b8e-33dc-4e15-85b5-b6024613df20', 'STL-TL');

INSERT INTO public.course__program_specialism (id, program_id)
VALUES ('4eb07b8e-33dc-4e15-85b5-b6024613df20', 'be738e71-0023-40f6-a3e4-7e2a5bde0a75');

-- View
CREATE VIEW course__enrolable_view AS
    SELECT course__enrolable.id, code, name, ects, type
    FROM course__enrolable
    LEFT JOIN course__enrolable_code ON course__enrolable.id = course__enrolable_code.id
    LEFT JOIN course__enrolable_name ON course__enrolable.id = course__enrolable_name.id
    LEFT JOIN course__enrolable_ects ON course__enrolable.id = course__enrolable_ects.id;
