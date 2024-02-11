-- Add migration script here
create table todos (
    id text primary key,
    created_at timestamptz(3) not null,
    updated_at timestamptz(3) not null,
    description varchar(255) not null,
    done boolean not null default false,
    done_at timestamptz(3)
);