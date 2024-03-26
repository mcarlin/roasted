--: Bean(description?, region?, grade?)

--! all_beans : Bean
select bean_id, name, description, ts, region, grade
from core.bean;

--! find_bean_by_id : Bean
select bean_id, name, description, ts, region, grade
from core.bean
where bean_id = :bean_id;

--! insert_bean (description?, region?, grade?) : Bean
insert into core.bean (bean_id, name, description, ts, region, grade)
values (:bean_id, :name, :description, :ts, :region, :grade)
returning
    bean_id,
    name,
    description,
    ts,
    region,
    grade;

--! update_bean (description?, region?, grade?) : Bean
update core.bean
set (name, description, ts, region, grade) = (:name, :description, :ts, :region, :grade)
where bean_id = :bean_id
returning
    bean_id,
    name,
    description,
    ts,
    region,
    grade;