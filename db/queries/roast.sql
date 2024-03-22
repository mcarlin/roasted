--! all_roasts
select r.roast_id        as roast_id,
       r.bean_id         as bean_id,
       r.ts              as roast_ts,
       rl.roast_level_id as roast_level_id,
       rl.name           as roast_level_name,
       rl.description    as roast_level_description
from core.roast r
         inner join core.roast_level rl using (roast_level_id);

--! find_roast_by_id
select r.roast_id        as roast_id,
       r.bean_id         as bean_id,
       r.ts              as roast_ts,
       rl.roast_level_id as roast_level_id,
       rl.name           as roast_level_name,
       rl.description    as roast_level_description
from core.roast r
         inner join core.roast_level rl using (roast_level_id)
where roast_id = :roast_id;

--! all_roast_steps
select roast_step_id,
       roast_id,
       position,
       description,
       time,
       fan_speed,
       temp_setting,
       temperature
from core.roast_step
where roast_id = :roast_id;

--! insert_roast
with r as (
    insert into core.roast (roast_id, bean_id, roast_level_id, ts)
        values (:roast_id, :bean_id, :roast_level_id, :ts)
        returning roast_id, bean_id, roast_level_id, ts)
select r.roast_id        as roast_id,
       r.bean_id         as bean_id,
       r.ts              as roast_ts,
       rl.roast_level_id as roast_level_id,
       rl.name           as roast_level_name,
       rl.description    as roast_level_description
from r
         inner join core.roast_level rl using (roast_level_id);

--! insert_roast_step
insert into core.roast_step(roast_step_id, roast_id, position, description, time, fan_speed, temp_setting, temperature)
values (:roast_step_id, :roast_id, :position, :description, :time, :fan_speed, :temp_setting, :temperature)
returning
    roast_step_id,
    roast_id,
    position,
    description,
    time,
    fan_speed,
    temp_setting,
    temperature;