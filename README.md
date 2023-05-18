
# Getting Started

The server expects a PostgreSQL database running on
localhost. The database should be called "task-server"
and the username and password should both be "taskadmin".

See https://www.postgresql.org/docs/current/tutorial-install.html

```shell
$ createdb --owner taskadmin task-server
```

The server will create any necessary tables at runtime.

```shell
$ cargo run --release
```

The server will listen for incoming http connections on port 3000.

# Rest API

## GET /
List tasks.

Accepts query params "page_size", "page", "task_type", "task_state"

"page_size" limits the number of tasks returned. Defaults to 10.

"page" is a 0-based index of pages. Defaults to 0.

"task_type" is one of ("Foo", "Bar", "Baz")

"task_state" is one of ("Scheduled", "Running", "Completed")

## POST /
Create a task.

Expects a JSON body of the form
```json
{ "task_type": "Foo|Bar|Baz", "execution_time": "2023-05-17T20:25:27Z" }
```

## GET /:id
Get a task by id.

## DELETE /:id
Delete a task by id.
