BEGIN;

CREATE TABLE users (
  id SERIAL,
  name VARCHAR(255)
);

INSERT INTO users (name) VALUES ('John', 'Sean');

COMMIT;
