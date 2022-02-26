CREATE TABLE articles (
	guid varchar NOT NULL,
	targert_guid uuid NOT NULL,
	title varchar NOT NULL,
	summary varchar NULL,
	body varchar NOT NULL,
	"date" timestamp NOT NULL,
	author varchar NULL,
	link varchar NOT NULL,
	CONSTRAINT articles_pk PRIMARY KEY (guid),
	CONSTRAINT articles_un UNIQUE (link),
	CONSTRAINT articles_fk FOREIGN KEY (targert_guid) REFERENCES public.targets(guid)
);
