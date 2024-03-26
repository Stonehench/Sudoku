create table users(
    user_id char(36) primary key,
    username varchar(20) unique,
    password varchar(20)
)

create table scores(
    score_id SERIAL primary key,
    user_id char(36) references users(user_id),
    value int,
    submittet timestamp default now()
)

create view userscores as select user_id, username, sum(value) as value from scores natural join users group by scores.user_id;