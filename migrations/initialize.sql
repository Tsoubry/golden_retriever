CREATE TABLE article (
    id serial PRIMARY KEY,
    article_id VARCHAR(36) UNIQUE NOT NULL,
    article_title VARCHAR(255) NOT NULL,
    platform VARCHAR(60) NOT NULL,
    section VARCHAR(60) NOT NULL,
    image_url TEXT,
    article_url TEXT,
    updated TIMESTAMP
);
