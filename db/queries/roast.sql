--! all_roasts : Roast()
select r.roast_id       as roast_id,
       r.bean_id        as bean_id,
       r.ts             as ts,
       r.roast_level_id as roast_level_id
from core.roast r;

--! find_roast_by_id : Roast()
select r.roast_id       as roast_id,
       r.bean_id        as bean_id,
       r.ts             as ts,
       r.roast_level_id as roast_level_id
from core.roast r
where roast_id = :roast_id;

--! insert_roast : Roast()
insert into core.roast (roast_id, bean_id, roast_level_id, ts)
values (:roast_id, :bean_id, :roast_level_id, :ts)
returning roast_id, bean_id, roast_level_id, ts;

--! update_roast : Roast()
update core.roast
set bean_id        = :bean_id,
    roast_level_id = :roast_level_id,
    ts             = :ts
where roast_id = :roast_id
returning roast_id, bean_id, roast_level_id, ts;

--! delete_roast
delete from core.roast
where roast_id = :roast_id;

--: RoastLevel (description?)

--! all_roast_levels: RoastLevel
select roast_level_id,
       name,
       description
from core.roast_level;

--: RoastStep (description?, fan_speed?, temp_setting?, temperature?)

--! all_roast_steps : RoastStep
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


--! find_roast_step_by_id: RoastStep
select roast_step_id,
       roast_id,
       position,
       description,
       time,
       fan_speed,
       temp_setting,
       temperature
from core.roast_step
where roast_step_id = :roast_step_id;

--! insert_roast_step (description?, fan_speed?, temp_setting?, temperature?) : RoastStep
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

--! update_roast_step (description?, fan_speed?, temp_setting?, temperature?) : RoastStep
update core.roast_step
set roast_id     = :roast_id,
    position     = :position,
    description  = :description,
    time = :time,
    fan_speed    = :fan_speed,
    temp_setting = :temp_setting,
    temperature  = :temperature
where roast_step_id = :roast_step_id
returning
    roast_step_id,
    roast_id,
    position,
    description,
    time,
    fan_speed,
    temp_setting,
    temperature;

--! delete_roast_step
delete from core.roast
where roast_id = :roast_id;
