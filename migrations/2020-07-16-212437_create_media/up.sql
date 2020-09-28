CREATE TABLE media (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  path VARCHAR UNIQUE NOT NULL,
  processed BOOLEAN NOT NULL DEFAULT 'f'
)
