CREATE TABLE IF NOT EXISTS users (
  id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name VARCHAR(32) NOT NULL,
  email VARCHAR(64) NOT NULL UNIQUE CHECK (email LIKE '%@%'),      -- must have an @ symbol within.
  phone VARCHAR(10) NOT NULL UNIQUE CHECK (phone ~ '^[0-9]{10}$')  -- only allow digits and length of 10.
);
