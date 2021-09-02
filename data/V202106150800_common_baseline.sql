CREATE TABLE foo (id INT, bar VARCHAR(20));
CREATE TABLE IF NOT EXISTS public.example__author_initial
(
    id      SERIAL PRIMARY KEY,
    name    VARCHAR NOT NULL,
    country VARCHAR NOT NULL
);
INSERT INTO public.example__author_initial (id, name, country)
VALUES (2, 'Ngo Dinh Nhu', 'Viet Nam Cong Hoa');
ALTER TABLE public.example__author_initial ADD COLUMN address VARCHAR(255);
