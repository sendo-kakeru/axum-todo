-- Add migration script here
CREATE TABLE task (
  id UUID PRIMARY KEY,
  title VARCHAR(40) NOT NULL,
  state TEXT NOT NULL CHECK (state IN ('Todo', 'Progress', 'Done')),
  created_at timestamp NOT NULL DEFAULT now(),
  updated_at timestamp NOT NULL DEFAULT now()
);