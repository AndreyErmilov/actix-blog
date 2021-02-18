CREATE TYPE article_status AS ENUM ('draft', 'approved', 'hidden');

CREATE TABLE users (
    id                      SERIAL PRIMARY KEY,
    user_name               VARCHAR NOT NULL,
    user_pic                VARCHAR NOT NULL
);

CREATE TABLE articles (
    id                      SERIAL PRIMARY KEY,
    user_id                 INTEGER NOT NULL,
    title                   VARCHAR NOT NULL,
    body                    VARCHAR NOT NULL,
    publication_time        TIMESTAMP WITH TIME ZONE NOT NULL,
    modification_time       TIMESTAMP WITH TIME ZONE NOT NULL,
    status                  article_status NOT NULL DEFAULT 'draft',
    FOREIGN KEY (user_id)   REFERENCES users (id)
);
