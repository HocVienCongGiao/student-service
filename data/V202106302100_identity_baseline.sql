CREATE TABLE IF NOT EXISTS public.identity__user
(
    id UUID PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS public.identity__user_username
(
    id UUID PRIMARY KEY REFERENCES identity__user(id),
    username VARCHAR NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS public.identity__user_email
(
    id UUID PRIMARY KEY REFERENCES identity__user(id),
    email VARCHAR NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS public.identity__user_phone
(
    id UUID PRIMARY KEY REFERENCES identity__user(id),
    phone VARCHAR NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS public.identity__user_enabled
(
    id UUID PRIMARY KEY REFERENCES identity__user(id),
    enabled bool NOT NULL
);

INSERT INTO public.identity__user (id)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608');

INSERT INTO public.identity__user_username (id, username)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', 'dev-operator');

INSERT INTO public.identity__user_email (id, email)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', 'projecthocvienconggiao@gmail.com');

INSERT INTO public.identity__user_phone (id, phone)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', '+84 909683979');

INSERT INTO public.identity__user_enabled (id, enabled)
VALUES ('40e6215d-b5c6-4896-987c-f30f3678f608', true);

CREATE VIEW identity__user_view AS
SELECT identity__user.id, username, email, phone, enabled
FROM identity__user
         LEFT JOIN identity__user_username
                   ON identity__user.id = identity__user_username.id
         LEFT JOIN identity__user_email
                   ON identity__user.id = identity__user_email.id
         LEFT JOIN identity__user_phone
                   ON identity__user.id = identity__user_phone.id
         LEFT JOIN identity__user_enabled
                   ON identity__user.id = identity__user_enabled.id
