ALTER TABLE articles
DROP CONSTRAINT articles_fk;

ALTER TABLE articles
DROP CONSTRAINT articles_un;

ALTER TABLE articles
DROP CONSTRAINT articles_pk;

DROP TABLE articles;
