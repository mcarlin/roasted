--: Bean(description?, region?, grade?)

--! all_beans : Bean
select bean_id, name, description, ts, region, grade
from core.bean;

--! find_bean_by_id : Bean
select bean_id, name, description, ts, region, grade
from core.bean
where bean_id = :bean_id;

--! insert_bean
insert into core.bean (name, description, ts, region, grade)
values (:name, :description, :ts, :region, :grade)
returning
    bean_id,
    name,
    description,
    ts,
    region,
    grade;