CREATE TABLE targets (
  guid UUID NOT NULL,
  name VARCHAR NOT NULL,
  url VARCHAR NOT NULL,
  active BOOLEAN NOT NULL DEFAULT 't',
  interval INTEGER NOT NULL DEFAULT 10,
  last_crawl timestamp,
  creation_time timestamp DEFAULT now(),
    PRIMARY KEY (guid)
)