CREATE TABLE IF NOT EXISTS users(
  id SERIAL PRIMARY KEY,
  username VARCHAR(100) NOT NULL,
  email VARCHAR(255) NOT NULL
);

CREATE UNIQUE INDEX uk_users_email ON users(lower(email));
CREATE UNIQUE INDEX uk_users_username ON users(lower(username));
