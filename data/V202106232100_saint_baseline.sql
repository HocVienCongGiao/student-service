CREATE TABLE IF NOT EXISTS public.saint__saint
(
    id                 UUID PRIMARY KEY
);
CREATE TABLE IF NOT EXISTS public.saint__saint_english_name
(
    id                 UUID PRIMARY KEY REFERENCES saint__saint(id),
    english_name       VARCHAR NOT NULL
);
CREATE UNIQUE INDEX IDX_saint_saint_english_name ON saint__saint_english_name (english_name);
CREATE TABLE IF NOT EXISTS public.saint__saint_french_name
(
    id                 UUID PRIMARY KEY REFERENCES saint__saint(id),
    french_name        VARCHAR NOT NULL
);
CREATE UNIQUE INDEX IDX_saint_saint_french_name ON saint__saint_french_name (french_name);
CREATE TABLE IF NOT EXISTS public.saint__saint_latin_name
(
    id                 UUID PRIMARY KEY REFERENCES saint__saint(id),
    latin_name         VARCHAR NOT NULL
);
CREATE UNIQUE INDEX IDX_saint_saint_latin_name ON saint__saint_latin_name (latin_name);
CREATE TABLE IF NOT EXISTS public.saint__saint_vietnamese_name
(
    id                 UUID PRIMARY KEY REFERENCES saint__saint(id),
    vietnamese_name    VARCHAR NOT NULL
);
CREATE UNIQUE INDEX IDX_saint_saint_vietnamese_name ON saint__saint_vietnamese_name (vietnamese_name);
CREATE TABLE IF NOT EXISTS public.saint__saint_display_name
(
    id                 UUID PRIMARY KEY REFERENCES saint__saint(id),
    display_name       VARCHAR NOT NULL
);
CREATE INDEX IDX_saint_saint_display_name ON saint__saint_display_name (display_name);
CREATE TABLE IF NOT EXISTS public.saint__saint_gender
(
    id                 UUID PRIMARY KEY REFERENCES saint__saint(id),
    is_male            BOOLEAN NOT NULL
);
CREATE TABLE IF NOT EXISTS public.saint__saint_feast_day
(
    id                 UUID PRIMARY KEY REFERENCES saint__saint(id),
    feast_day          SMALLINT NOT NULL,
    feast_month        SMALLINT NOT NULL
);

-- Thánh Phêrô Tông đồ 
INSERT INTO public.saint__saint (id)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608');

INSERT INTO public.saint__saint_english_name (id, english_name)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', 'Peter the Apostle');

INSERT INTO public.saint__saint_french_name (id, french_name)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', 'saint Pierre');

INSERT INTO public.saint__saint_latin_name (id, latin_name)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', 'Simon Petrus');

INSERT INTO public.saint__saint_vietnamese_name (id, vietnamese_name)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608','Thánh Phêrô Tông đồ');

INSERT INTO public.saint__saint_display_name (id, display_name)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', 'Phêrô');

INSERT INTO public.saint__saint_gender (id, is_male)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', true);

INSERT INTO public.saint__saint_feast_day (id, feast_day, feast_month)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', 29, 06);

CREATE VIEW saint__saint_view AS
    SELECT saint__saint.id, english_name, french_name, latin_name, vietnamese_name, display_name, is_male, feast_day, feast_month
    FROM
    saint__saint
    LEFT JOIN saint__saint_english_name ON saint__saint.id = saint__saint_english_name.id
    LEFT JOIN saint__saint_french_name ON saint__saint.id = saint__saint_french_name.id
    LEFT JOIN saint__saint_latin_name ON saint__saint.id = saint__saint_latin_name.id
    LEFT JOIN saint__saint_vietnamese_name ON saint__saint.id = saint__saint_vietnamese_name.id
    LEFT JOIN saint__saint_display_name ON saint__saint.id = saint__saint_display_name.id
    LEFT JOIN saint__saint_gender ON saint__saint.id = saint__saint_gender.id
    LEFT JOIN saint__saint_feast_day ON saint__saint.id = saint__saint_feast_day.id
