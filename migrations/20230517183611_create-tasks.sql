-- Add migration script here

CREATE EXTENSION "uuid-ossp";

CREATE TYPE task_state AS ENUM ('Scheduled', 'Running', 'Completed');

CREATE TYPE task_type as ENUM ('Foo', 'Bar', 'Baz');

CREATE TABLE IF NOT EXISTS tasks (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    execution_time timestamp with time zone NOT NULL,
    task_type task_type NOT NULL,
    task_state task_state DEFAULT 'Scheduled'
);

CREATE INDEX IF NOT EXISTS execution_time_and_state_index ON tasks (
    execution_time,
    task_state
);
CREATE INDEX IF NOT EXISTS execution_time_index ON tasks (execution_time);
CREATE INDEX IF NOT EXISTS task_type_index ON tasks (task_type);
CREATE INDEX IF NOT EXISTS task_state_index ON tasks (task_state);
