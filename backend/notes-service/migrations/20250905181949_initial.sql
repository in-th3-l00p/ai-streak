create table notes (
    id serial primary key,
    user_id int not null,
    title varchar(255) not null,
    content text not null,
    is_public boolean not null default false,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);