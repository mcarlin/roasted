SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: core; Type: SCHEMA; Schema: -; Owner: -
--

CREATE SCHEMA core;


--
-- Name: public; Type: SCHEMA; Schema: -; Owner: -
--

-- *not* creating schema, since initdb creates it


--
-- Name: SCHEMA public; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON SCHEMA public IS '';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: bean; Type: TABLE; Schema: core; Owner: -
--

CREATE TABLE core.bean (
    bean_id uuid NOT NULL,
    name text NOT NULL,
    description text,
    ts timestamp with time zone NOT NULL,
    region text,
    grade text
);


--
-- Name: roast; Type: TABLE; Schema: core; Owner: -
--

CREATE TABLE core.roast (
    roast_id uuid NOT NULL,
    bean_id uuid,
    roast_level_id uuid,
    ts timestamp with time zone NOT NULL
);


--
-- Name: roast_level; Type: TABLE; Schema: core; Owner: -
--

CREATE TABLE core.roast_level (
    roast_level_id uuid NOT NULL,
    name text NOT NULL,
    description text
);


--
-- Name: roast_step; Type: TABLE; Schema: core; Owner: -
--

CREATE TABLE core.roast_step (
    roast_step_id uuid NOT NULL,
    roast_id uuid,
    "position" integer NOT NULL,
    "time" bigint NOT NULL,
    fan_speed integer,
    temp_setting integer,
    temperature numeric,
    description text,
    CONSTRAINT roast_step_fan_speed_check CHECK (((fan_speed >= 1) AND (fan_speed <= 9))),
    CONSTRAINT roast_step_temp_setting_check CHECK (((temp_setting >= 1) AND (temp_setting <= 9)))
);


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schema_migrations (
    version character varying(128) NOT NULL
);


--
-- Name: bean bean_pkey; Type: CONSTRAINT; Schema: core; Owner: -
--

ALTER TABLE ONLY core.bean
    ADD CONSTRAINT bean_pkey PRIMARY KEY (bean_id);


--
-- Name: roast_level roast_level_pkey; Type: CONSTRAINT; Schema: core; Owner: -
--

ALTER TABLE ONLY core.roast_level
    ADD CONSTRAINT roast_level_pkey PRIMARY KEY (roast_level_id);


--
-- Name: roast roast_pkey; Type: CONSTRAINT; Schema: core; Owner: -
--

ALTER TABLE ONLY core.roast
    ADD CONSTRAINT roast_pkey PRIMARY KEY (roast_id);


--
-- Name: roast_step roast_step_pkey; Type: CONSTRAINT; Schema: core; Owner: -
--

ALTER TABLE ONLY core.roast_step
    ADD CONSTRAINT roast_step_pkey PRIMARY KEY (roast_step_id);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: roast roast_bean_id_fkey; Type: FK CONSTRAINT; Schema: core; Owner: -
--

ALTER TABLE ONLY core.roast
    ADD CONSTRAINT roast_bean_id_fkey FOREIGN KEY (bean_id) REFERENCES core.bean(bean_id) ON DELETE CASCADE;


--
-- Name: roast roast_roast_level_id_fkey; Type: FK CONSTRAINT; Schema: core; Owner: -
--

ALTER TABLE ONLY core.roast
    ADD CONSTRAINT roast_roast_level_id_fkey FOREIGN KEY (roast_level_id) REFERENCES core.roast_level(roast_level_id);


--
-- Name: roast_step roast_step_roast_id_fkey; Type: FK CONSTRAINT; Schema: core; Owner: -
--

ALTER TABLE ONLY core.roast_step
    ADD CONSTRAINT roast_step_roast_id_fkey FOREIGN KEY (roast_id) REFERENCES core.roast(roast_id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--


--
-- Dbmate schema migrations
--

INSERT INTO public.schema_migrations (version) VALUES
    ('20240303063729');
