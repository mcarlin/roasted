--! all_roasts
select r.roast_id       as roast_id,
       r.bean_id        as bean_id,
       r.ts             as roast_ts,
       r.roast_level_id as roast_level_id
from core.roast r;

--! find_roast_by_id
select r.roast_id       as roast_id,
       r.bean_id        as bean_id,
       r.ts             as roast_ts,
       r.roast_level_id as roast_level_id
from core.roast r
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
insert into core.roast (roast_id, bean_id, roast_level_id, ts)
values (:roast_id, :bean_id, :roast_level_id, :ts)
returning roast_id, bean_id, roast_level_id, ts;

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

--! update_roast_step
update core.roast_step
set roast_id     = :roast_id,
    position     = :position,
    description  = :description,
    time= :time,
    fan_speed    = :fan_speed,
    temp_setting = :temp_setting,
    temperature  = :temperature
where roast_id = :roast_id;

--! update_roast
update core.roast
set bean_id        = :bean_id,
    roast_level_id = :roast_level_id,
    ts             = :ts
where roast_id = :roast_id;