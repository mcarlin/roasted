-- Remove default schema
drop schema if exists public;

-- Create app specific schema... 'core' for beans, roasts, etc.
-- Assuming users, sessions, etc. fit in a different schema
create schema if not exists core;

-- Setup admin role
drop owned by roasted_admin_role;
drop role if exists roasted_admin_role;
create role roasted_admin_role nologin;
grant all privileges on database roasted to roasted_admin_role;
grant all privileges on schema core to roasted_admin_role;

-- Setup r/w role
drop owned by roasted_rw_role;
drop role if exists roasted_rw_role;
create role roasted_rw_role nologin;

-- Setup roasted admin user
drop owned by roasted_admin_user;
drop user if exists roasted_admin_user;
create user roasted_admin_user with encrypted password :'admin_pass';
grant roasted_admin_role to roasted_admin_user;
alter default privileges
    for user roasted_admin_user -- The user is the 'owner' will create all the db objects, and thus must be the granter
    in schema core
    grant select, insert, update, delete on tables to roasted_rw_role;

-- Setup roasted app user
drop owned by roasted_app_user;
drop user if exists roasted_app_user;
create user roasted_app_user with encrypted password :'app_pass';
grant connect on database roasted to roasted_app_user;
grant usage on schema core to roasted_app_user;
grant roasted_rw_role to roasted_app_user;
