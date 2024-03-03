-- migrate:up

create table if not exists bean
(
    bean_id     int primary key generated always as identity,
    name        text      not null,
    description text,
    ts          timestamp not null,
    region      text,
    grade       text
);

create table if not exists roast_level
(
    roast_level_id int primary key generated always as identity,
    name           text not null,
    description    text
);

create table if not exists roast
(
    roast_id int primary key generated always as identity,
    bean_id  int references bean (bean_id) on delete cascade,
    level_id int references roast_level (roast_level_id),
    ts       timestamp not null
);

create table if not exists roast_step
(
    roast_step_id int primary key generated always as identity,
    roast_id      integer references roast (roast_id) on delete cascade,
    time          bigint NOT NULL,
    fan_speed     integer check (fan_speed between 1 and 9),
    temp_setting  integer check (temp_setting between 1 and 9),
    temperature   numeric,
    description   text
);

-- source: https://thecaptainscoffee.com/pages/roast-levels
insert into roast_level(name, description)
values ('Cinnamon', 'Beans roasted to the very beginning of first cracks.'),
       ('City Roast', 'Beans roasted to the middle of first cracks.'),
       ('City+ Roast', 'Beans roasted to the tail end of first cracks.'),
       ('Full-City Roast', 'Beans roasted past first cracks but before second cracks have begun.'),
       ('Full-City+ Roast', 'Beans roasted to the very start of second cracks.'),
       ('Vienna Roast', 'Beans roasted to the middle of second cracks.'),
       ('French Roast', 'Beans roasted to the tail end of second cracks.'),
       ('Italian/Spanish Roast', 'Beans roasted past second cracks.')
on conflict do nothing;

-- migrate:down

drop table if exists roast_step;
drop table if exists roast;
drop table if exists roast_level;
drop table if exists bean;
