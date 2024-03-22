-- migrate:up

create table if not exists core.bean
(
    bean_id     uuid primary key,
    name        text      not null,
    description text,
    ts          timestamptz not null,
    region      text,
    grade       text
);

create table if not exists core.roast_level
(
    roast_level_id uuid primary key,
    name           text not null ,
    description    text
);

create table if not exists core.roast
(
    roast_id       uuid primary key,
    bean_id        uuid references core.bean (bean_id) on delete cascade,
    roast_level_id uuid references core.roast_level (roast_level_id),
    ts             timestamptz not null
);

create table if not exists core.roast_step
(
    roast_step_id uuid primary key,
    roast_id      uuid references core.roast (roast_id) on delete cascade,
    position      integer not null,
    time          bigint  not null,
    fan_speed     integer check (fan_speed between 1 and 9),
    temp_setting  integer check (temp_setting between 1 and 9),
    temperature   numeric,
    description   text
);

-- source: https://thecaptainscoffee.com/pages/roast-levels
insert into core.roast_level(roast_level_id, name, description)
values (gen_random_uuid(), 'Cinnamon', 'Beans roasted to the very beginning of first cracks.'),
       (gen_random_uuid(), 'City Roast', 'Beans roasted to the middle of first cracks.'),
       (gen_random_uuid(), 'City+ Roast', 'Beans roasted to the tail end of first cracks.'),
       (gen_random_uuid(), 'Full-City Roast', 'Beans roasted past first cracks but before second cracks have begun.'),
       (gen_random_uuid(), 'Full-City+ Roast', 'Beans roasted to the very start of second cracks.'),
       (gen_random_uuid(), 'Vienna Roast', 'Beans roasted to the middle of second cracks.'),
       (gen_random_uuid(), 'French Roast', 'Beans roasted to the tail end of second cracks.'),
       (gen_random_uuid(), 'Italian/Spanish Roast', 'Beans roasted past second cracks.')
on conflict do nothing;

-- migrate:down

drop table if exists core.roast_step;
drop table if exists core.roast;
drop table if exists core.roast_level;
drop table if exists core.bean;
