CREATE TABLE entries (
  id SERIAL PRIMARY KEY,
  pract_date VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  notes TEXT NOT NULL,
  pract_time INT NOT NULL,
  focus_time INT NOT NULL
)