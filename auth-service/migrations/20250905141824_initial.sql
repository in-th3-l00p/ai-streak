create table users (
    id serial primary key,
    username varchar(255) not null,
    email varchar(255) not null,
    password varchar(255) not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

create table notes (
    id serial primary key,
    title varchar(255) not null,
    content text not null,
    user_id int not null,
    is_public boolean not null default false,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
