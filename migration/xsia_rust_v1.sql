--
-- PostgreSQL database dump
--

-- Dumped from database version 17.5
-- Dumped by pg_dump version 17.5

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: academic_campaign_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_campaign_reference;


ALTER SCHEMA academic_campaign_reference OWNER TO bendo01;

--
-- Name: academic_campaign_transaction; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_campaign_transaction;


ALTER SCHEMA academic_campaign_transaction OWNER TO bendo01;

--
-- Name: academic_candidate_master; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_candidate_master;


ALTER SCHEMA academic_candidate_master OWNER TO bendo01;

--
-- Name: academic_candidate_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_candidate_reference;


ALTER SCHEMA academic_candidate_reference OWNER TO bendo01;

--
-- Name: academic_candidate_transaction; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_candidate_transaction;


ALTER SCHEMA academic_candidate_transaction OWNER TO bendo01;

--
-- Name: academic_course_master; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_course_master;


ALTER SCHEMA academic_course_master OWNER TO bendo01;

--
-- Name: academic_course_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_course_reference;


ALTER SCHEMA academic_course_reference OWNER TO bendo01;

--
-- Name: academic_general_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_general_reference;


ALTER SCHEMA academic_general_reference OWNER TO bendo01;

--
-- Name: academic_lecturer_master; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_lecturer_master;


ALTER SCHEMA academic_lecturer_master OWNER TO bendo01;

--
-- Name: academic_lecturer_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_lecturer_reference;


ALTER SCHEMA academic_lecturer_reference OWNER TO bendo01;

--
-- Name: academic_lecturer_transaction; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_lecturer_transaction;


ALTER SCHEMA academic_lecturer_transaction OWNER TO bendo01;

--
-- Name: academic_prior_learning_recognition_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_prior_learning_recognition_reference;


ALTER SCHEMA academic_prior_learning_recognition_reference OWNER TO bendo01;

--
-- Name: academic_prior_learning_recognition_transaction; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_prior_learning_recognition_transaction;


ALTER SCHEMA academic_prior_learning_recognition_transaction OWNER TO bendo01;

--
-- Name: academic_student_adviser; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_student_adviser;


ALTER SCHEMA academic_student_adviser OWNER TO bendo01;

--
-- Name: academic_student_campaign; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_student_campaign;


ALTER SCHEMA academic_student_campaign OWNER TO bendo01;

--
-- Name: academic_student_final_assignment_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_student_final_assignment_reference;


ALTER SCHEMA academic_student_final_assignment_reference OWNER TO bendo01;

--
-- Name: academic_student_final_assignment_transaction; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_student_final_assignment_transaction;


ALTER SCHEMA academic_student_final_assignment_transaction OWNER TO bendo01;

--
-- Name: academic_student_master; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_student_master;


ALTER SCHEMA academic_student_master OWNER TO bendo01;

--
-- Name: academic_student_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_student_reference;


ALTER SCHEMA academic_student_reference OWNER TO bendo01;

--
-- Name: academic_survey_master; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_survey_master;


ALTER SCHEMA academic_survey_master OWNER TO bendo01;

--
-- Name: academic_survey_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_survey_reference;


ALTER SCHEMA academic_survey_reference OWNER TO bendo01;

--
-- Name: academic_survey_transaction; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA academic_survey_transaction;


ALTER SCHEMA academic_survey_transaction OWNER TO bendo01;

--
-- Name: ai; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA ai;


ALTER SCHEMA ai OWNER TO bendo01;

--
-- Name: auth; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA auth;


ALTER SCHEMA auth OWNER TO bendo01;

--
-- Name: building_master; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA building_master;


ALTER SCHEMA building_master OWNER TO bendo01;

--
-- Name: building_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA building_reference;


ALTER SCHEMA building_reference OWNER TO bendo01;

--
-- Name: contact_master; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA contact_master;


ALTER SCHEMA contact_master OWNER TO bendo01;

--
-- Name: contact_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA contact_reference;


ALTER SCHEMA contact_reference OWNER TO bendo01;

--
-- Name: document_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA document_reference;


ALTER SCHEMA document_reference OWNER TO bendo01;

--
-- Name: document_transaction; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA document_transaction;


ALTER SCHEMA document_transaction OWNER TO bendo01;

--
-- Name: feeder_akumulasi; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA feeder_akumulasi;


ALTER SCHEMA feeder_akumulasi OWNER TO bendo01;

--
-- Name: feeder_akun; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA feeder_akun;


ALTER SCHEMA feeder_akun OWNER TO bendo01;

--
-- Name: feeder_master; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA feeder_master;


ALTER SCHEMA feeder_master OWNER TO bendo01;

--
-- Name: feeder_referensi; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA feeder_referensi;


ALTER SCHEMA feeder_referensi OWNER TO bendo01;

--
-- Name: feeder_rekapitulasi; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA feeder_rekapitulasi;


ALTER SCHEMA feeder_rekapitulasi OWNER TO bendo01;

--
-- Name: institution_master; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA institution_master;


ALTER SCHEMA institution_master OWNER TO bendo01;

--
-- Name: institution_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA institution_reference;


ALTER SCHEMA institution_reference OWNER TO bendo01;

--
-- Name: literate; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA literate;


ALTER SCHEMA literate OWNER TO bendo01;

--
-- Name: location; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA location;


ALTER SCHEMA location OWNER TO bendo01;

--
-- Name: payment_midtrans; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA payment_midtrans;


ALTER SCHEMA payment_midtrans OWNER TO bendo01;

--
-- Name: person_master; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA person_master;


ALTER SCHEMA person_master OWNER TO bendo01;

--
-- Name: person_reference; Type: SCHEMA; Schema: -; Owner: bendo01
--

CREATE SCHEMA person_reference;


ALTER SCHEMA person_reference OWNER TO bendo01;

--
-- Name: pg_uuidv7; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pg_uuidv7 WITH SCHEMA public;


--
-- Name: EXTENSION pg_uuidv7; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pg_uuidv7 IS 'pg_uuidv7: create UUIDv7 values in postgres';


--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: attend_types; Type: TABLE; Schema: academic_campaign_reference; Owner: bendo01
--

CREATE TABLE academic_campaign_reference.attend_types (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_reference.attend_types OWNER TO bendo01;

--
-- Name: calendar_categories; Type: TABLE; Schema: academic_campaign_reference; Owner: bendo01
--

CREATE TABLE academic_campaign_reference.calendar_categories (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_reference.calendar_categories OWNER TO bendo01;

--
-- Name: encounter_categories; Type: TABLE; Schema: academic_campaign_reference; Owner: bendo01
--

CREATE TABLE academic_campaign_reference.encounter_categories (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_reference.encounter_categories OWNER TO bendo01;

--
-- Name: encounter_types; Type: TABLE; Schema: academic_campaign_reference; Owner: bendo01
--

CREATE TABLE academic_campaign_reference.encounter_types (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_reference.encounter_types OWNER TO bendo01;

--
-- Name: evaluation_types; Type: TABLE; Schema: academic_campaign_reference; Owner: bendo01
--

CREATE TABLE academic_campaign_reference.evaluation_types (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    feeder_id uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_reference.evaluation_types OWNER TO bendo01;

--
-- Name: implementations; Type: TABLE; Schema: academic_campaign_reference; Owner: bendo01
--

CREATE TABLE academic_campaign_reference.implementations (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_reference.implementations OWNER TO bendo01;

--
-- Name: scopes; Type: TABLE; Schema: academic_campaign_reference; Owner: bendo01
--

CREATE TABLE academic_campaign_reference.scopes (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_reference.scopes OWNER TO bendo01;

--
-- Name: substances; Type: TABLE; Schema: academic_campaign_reference; Owner: bendo01
--

CREATE TABLE academic_campaign_reference.substances (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_reference.substances OWNER TO bendo01;

--
-- Name: activities; Type: TABLE; Schema: academic_campaign_transaction; Owner: bendo01
--

CREATE TABLE academic_campaign_transaction.activities (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(255) NOT NULL,
    week_quantity integer DEFAULT 0,
    student_target integer DEFAULT 0 NOT NULL,
    candidate_number integer DEFAULT 0 NOT NULL,
    candidate_pass integer DEFAULT 0 NOT NULL,
    became_student integer DEFAULT 0 NOT NULL,
    transfer_student integer DEFAULT 0 NOT NULL,
    total_class_member integer DEFAULT 0,
    start_date date,
    end_date date,
    start_transaction date,
    end_transaction date,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    academic_year_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    is_active boolean DEFAULT false,
    feeder_id uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_transaction.activities OWNER TO bendo01;

--
-- Name: calendar_details; Type: TABLE; Schema: academic_campaign_transaction; Owner: bendo01
--

CREATE TABLE academic_campaign_transaction.calendar_details (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    calendar_category_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    calendar_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    start_date date NOT NULL,
    end_date date NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_transaction.calendar_details OWNER TO bendo01;

--
-- Name: calendars; Type: TABLE; Schema: academic_campaign_transaction; Owner: bendo01
--

CREATE TABLE academic_campaign_transaction.calendars (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    academic_year_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    institution_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_transaction.calendars OWNER TO bendo01;

--
-- Name: class_codes; Type: TABLE; Schema: academic_campaign_transaction; Owner: bendo01
--

CREATE TABLE academic_campaign_transaction.class_codes (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    activity_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    start_effective_date date,
    end_effective_date date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    capacity integer DEFAULT 0
);


ALTER TABLE academic_campaign_transaction.class_codes OWNER TO bendo01;

--
-- Name: grades; Type: TABLE; Schema: academic_campaign_transaction; Owner: bendo01
--

CREATE TABLE academic_campaign_transaction.grades (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    grade double precision DEFAULT '0'::double precision NOT NULL,
    minimum double precision DEFAULT '0'::double precision NOT NULL,
    maximum double precision DEFAULT '0'::double precision NOT NULL,
    start_date date,
    end_date date,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_transaction.grades OWNER TO bendo01;

--
-- Name: schedules; Type: TABLE; Schema: academic_campaign_transaction; Owner: bendo01
--

CREATE TABLE academic_campaign_transaction.schedules (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name text,
    start_hour time(0) without time zone NOT NULL,
    end_hour time(0) without time zone NOT NULL,
    weekday_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    room_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    teach_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_transaction.schedules OWNER TO bendo01;

--
-- Name: teach_decrees; Type: TABLE; Schema: academic_campaign_transaction; Owner: bendo01
--

CREATE TABLE academic_campaign_transaction.teach_decrees (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    decree_number character varying(255) NOT NULL,
    decree_date date NOT NULL,
    activity_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    staff_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_transaction.teach_decrees OWNER TO bendo01;

--
-- Name: teach_lecturers; Type: TABLE; Schema: academic_campaign_transaction; Owner: bendo01
--

CREATE TABLE academic_campaign_transaction.teach_lecturers (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255),
    planning integer DEFAULT 0 NOT NULL,
    realization integer DEFAULT 0 NOT NULL,
    credit numeric(3,1) DEFAULT 0,
    is_lecturer_home_base boolean DEFAULT false NOT NULL,
    lecturer_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    teach_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    evaluation_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_transaction.teach_lecturers OWNER TO bendo01;

--
-- Name: teaches; Type: TABLE; Schema: academic_campaign_transaction; Owner: bendo01
--

CREATE TABLE academic_campaign_transaction.teaches (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name text,
    class_code_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    course_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    activity_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    description text,
    start_date date,
    end_date date,
    practice_start_date date,
    practice_end_date date,
    curriculum_detail_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    teach_decree_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    is_lecturer_credit_sum_problem boolean DEFAULT false,
    is_lock boolean DEFAULT false,
    encounter_category_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    scope_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    max_member integer DEFAULT 0,
    feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_campaign_transaction.teaches OWNER TO bendo01;

--
-- Name: candidates; Type: TABLE; Schema: academic_candidate_master; Owner: bendo01
--

CREATE TABLE academic_candidate_master.candidates (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    thread integer DEFAULT 0,
    code character varying(255),
    name character varying(255) NOT NULL,
    student_national_number character varying(255),
    school_name character varying(255),
    school_regency_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    state_smart_card_number character varying(255),
    individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    academic_year_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    student_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    user_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    registration_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    institution_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    guidence_name character varying(255),
    guidence_phone_number character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_candidate_master.candidates OWNER TO bendo01;

--
-- Name: exam_classes; Type: TABLE; Schema: academic_candidate_master; Owner: bendo01
--

CREATE TABLE academic_candidate_master.exam_classes (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    phase_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    start_date timestamp(0) without time zone NOT NULL,
    end_date timestamp(0) without time zone,
    start_time time(0) without time zone,
    end_time time(0) without time zone,
    capacity integer DEFAULT 0 NOT NULL,
    lms_category integer DEFAULT 0,
    is_online boolean DEFAULT false,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_candidate_master.exam_classes OWNER TO bendo01;

--
-- Name: document_types; Type: TABLE; Schema: academic_candidate_reference; Owner: bendo01
--

CREATE TABLE academic_candidate_reference.document_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_candidate_reference.document_types OWNER TO bendo01;

--
-- Name: phases; Type: TABLE; Schema: academic_candidate_reference; Owner: bendo01
--

CREATE TABLE academic_candidate_reference.phases (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_candidate_reference.phases OWNER TO bendo01;

--
-- Name: registration_categories; Type: TABLE; Schema: academic_candidate_reference; Owner: bendo01
--

CREATE TABLE academic_candidate_reference.registration_categories (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp(6) without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_candidate_reference.registration_categories OWNER TO bendo01;

--
-- Name: registration_types; Type: TABLE; Schema: academic_candidate_reference; Owner: bendo01
--

CREATE TABLE academic_candidate_reference.registration_types (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    code integer DEFAULT 0,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp(6) without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    student_registration_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    registration_category_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    institution_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_candidate_reference.registration_types OWNER TO bendo01;

--
-- Name: candidate_unit_choices; Type: TABLE; Schema: academic_candidate_transaction; Owner: bendo01
--

CREATE TABLE academic_candidate_transaction.candidate_unit_choices (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    candidate_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    student_registration_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    registration_category_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    phase_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    priority integer DEFAULT 0 NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_candidate_transaction.candidate_unit_choices OWNER TO bendo01;

--
-- Name: documents; Type: TABLE; Schema: academic_candidate_transaction; Owner: bendo01
--

CREATE TABLE academic_candidate_transaction.documents (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    candidate_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    document_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    filename character varying(255),
    dir character varying(255),
    type character varying(255),
    size integer,
    is_verified boolean DEFAULT false,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_candidate_transaction.documents OWNER TO bendo01;

--
-- Name: exams; Type: TABLE; Schema: academic_candidate_transaction; Owner: bendo01
--

CREATE TABLE academic_candidate_transaction.exams (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    candidate_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    exam_class_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    score double precision DEFAULT 0 NOT NULL,
    is_present boolean DEFAULT false,
    is_pass boolean DEFAULT false NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_candidate_transaction.exams OWNER TO bendo01;

--
-- Name: concentrations; Type: TABLE; Schema: academic_course_master; Owner: bendo01
--

CREATE TABLE academic_course_master.concentrations (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    name character varying(255) NOT NULL,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_course_master.concentrations OWNER TO bendo01;

--
-- Name: course_evaluation_plannings; Type: TABLE; Schema: academic_course_master; Owner: bendo01
--

CREATE TABLE academic_course_master.course_evaluation_plannings (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    percentage real DEFAULT '0'::real,
    decription_indonesian text NOT NULL,
    decription_english text,
    course_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    course_evaluation_base_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_course_master.course_evaluation_plannings OWNER TO bendo01;

--
-- Name: course_learn_plannings; Type: TABLE; Schema: academic_course_master; Owner: bendo01
--

CREATE TABLE academic_course_master.course_learn_plannings (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    name character varying(255) NOT NULL,
    decription_indonesian text NOT NULL,
    decription_english text,
    course_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_course_master.course_learn_plannings OWNER TO bendo01;

--
-- Name: courses; Type: TABLE; Schema: academic_course_master; Owner: bendo01
--

CREATE TABLE academic_course_master.courses (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    implementation_method text,
    total_credit double precision DEFAULT 0 NOT NULL,
    lecture_credit double precision DEFAULT 0 NOT NULL,
    practice_credit double precision DEFAULT 0 NOT NULL,
    field_practice_credit double precision DEFAULT 0 NOT NULL,
    simulation_credit double precision DEFAULT 0 NOT NULL,
    has_unit boolean DEFAULT false NOT NULL,
    has_syllabus boolean DEFAULT false NOT NULL,
    has_material boolean DEFAULT false NOT NULL,
    has_practice boolean DEFAULT false NOT NULL,
    has_dictation boolean DEFAULT false NOT NULL,
    group_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    variety_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    competence_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    feeder_course_group_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    feeder_course_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    feeder_course_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    start_date date,
    end_date date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_course_master.courses OWNER TO bendo01;

--
-- Name: curriculum_details; Type: TABLE; Schema: academic_course_master; Owner: bendo01
--

CREATE TABLE academic_course_master.curriculum_details (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer,
    curriculum_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    semester_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    course_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    credit double precision DEFAULT '0'::double precision,
    name character varying,
    concentration_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    is_convertable_to_mbkm boolean DEFAULT false,
    feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    is_convertable_to_prior_learning_recognition boolean DEFAULT false
);


ALTER TABLE academic_course_master.curriculum_details OWNER TO bendo01;

--
-- Name: curriculums; Type: TABLE; Schema: academic_course_master; Owner: bendo01
--

CREATE TABLE academic_course_master.curriculums (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    academic_year_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    curriculum_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    total_credit double precision DEFAULT 0,
    mandatory_course_credit double precision DEFAULT 0,
    optional_course_credit double precision DEFAULT 0,
    feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    start_date date,
    end_date date,
    is_active boolean DEFAULT false NOT NULL
);


ALTER TABLE academic_course_master.curriculums OWNER TO bendo01;

--
-- Name: competences; Type: TABLE; Schema: academic_course_reference; Owner: bendo01
--

CREATE TABLE academic_course_reference.competences (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    start_effective_date date,
    end_effective_date date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_course_reference.competences OWNER TO bendo01;

--
-- Name: course_evaluation_bases; Type: TABLE; Schema: academic_course_reference; Owner: bendo01
--

CREATE TABLE academic_course_reference.course_evaluation_bases (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    evaluation_base character varying(255) NOT NULL,
    component_evaluation character varying(255) NOT NULL,
    start_effective_date date,
    end_effective_date date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_course_reference.course_evaluation_bases OWNER TO bendo01;

--
-- Name: curriculum_types; Type: TABLE; Schema: academic_course_reference; Owner: bendo01
--

CREATE TABLE academic_course_reference.curriculum_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    start_effective_date date,
    end_effective_date date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_course_reference.curriculum_types OWNER TO bendo01;

--
-- Name: groups; Type: TABLE; Schema: academic_course_reference; Owner: bendo01
--

CREATE TABLE academic_course_reference.groups (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    abbreviation character varying(255),
    start_effective_date date,
    end_effective_date date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_course_reference.groups OWNER TO bendo01;

--
-- Name: semesters; Type: TABLE; Schema: academic_course_reference; Owner: bendo01
--

CREATE TABLE academic_course_reference.semesters (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    is_odd boolean DEFAULT false NOT NULL,
    start_effective_date date,
    end_effective_date date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_course_reference.semesters OWNER TO bendo01;

--
-- Name: varieties; Type: TABLE; Schema: academic_course_reference; Owner: bendo01
--

CREATE TABLE academic_course_reference.varieties (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    curriculum_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    start_effective_date date,
    end_effective_date date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_course_reference.varieties OWNER TO bendo01;

--
-- Name: academic_year_categories; Type: TABLE; Schema: academic_general_reference; Owner: bendo01
--

CREATE TABLE academic_general_reference.academic_year_categories (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_general_reference.academic_year_categories OWNER TO bendo01;

--
-- Name: academic_years; Type: TABLE; Schema: academic_general_reference; Owner: bendo01
--

CREATE TABLE academic_general_reference.academic_years (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    year integer NOT NULL,
    name character varying(255) NOT NULL,
    feeder_name character varying(255) NOT NULL,
    academic_year_category_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    is_active boolean DEFAULT true,
    start_date date,
    end_date date
);


ALTER TABLE academic_general_reference.academic_years OWNER TO bendo01;

--
-- Name: lecturers; Type: TABLE; Schema: academic_lecturer_master; Owner: bendo01
--

CREATE TABLE academic_lecturer_master.lecturers (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code character varying(255) NOT NULL,
    name character varying(255),
    individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    institution_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    alternative_code character varying(255),
    accessor_number character varying(255),
    identification_number character varying(255),
    status_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    contract_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    rank_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    start_date date,
    end_date date,
    front_title character varying(255),
    last_title character varying(255),
    feeder_id uuid,
    group_id uuid,
    nuptk character varying,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_lecturer_master.lecturers OWNER TO bendo01;

--
-- Name: contracts; Type: TABLE; Schema: academic_lecturer_reference; Owner: bendo01
--

CREATE TABLE academic_lecturer_reference.contracts (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_lecturer_reference.contracts OWNER TO bendo01;

--
-- Name: groups; Type: TABLE; Schema: academic_lecturer_reference; Owner: bendo01
--

CREATE TABLE academic_lecturer_reference.groups (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_lecturer_reference.groups OWNER TO bendo01;

--
-- Name: ranks; Type: TABLE; Schema: academic_lecturer_reference; Owner: bendo01
--

CREATE TABLE academic_lecturer_reference.ranks (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_lecturer_reference.ranks OWNER TO bendo01;

--
-- Name: statuses; Type: TABLE; Schema: academic_lecturer_reference; Owner: bendo01
--

CREATE TABLE academic_lecturer_reference.statuses (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_lecturer_reference.statuses OWNER TO bendo01;

--
-- Name: academic_groups; Type: TABLE; Schema: academic_lecturer_transaction; Owner: bendo01
--

CREATE TABLE academic_lecturer_transaction.academic_groups (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    decree_number character varying(255),
    decree_date date,
    lecturer_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    group_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    start_date date,
    end_date date
);


ALTER TABLE academic_lecturer_transaction.academic_groups OWNER TO bendo01;

--
-- Name: academic_ranks; Type: TABLE; Schema: academic_lecturer_transaction; Owner: bendo01
--

CREATE TABLE academic_lecturer_transaction.academic_ranks (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    decree_number character varying(255),
    decree_date date,
    lecturer_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    rank_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    start_date date,
    end_date date
);


ALTER TABLE academic_lecturer_transaction.academic_ranks OWNER TO bendo01;

--
-- Name: homebases; Type: TABLE; Schema: academic_lecturer_transaction; Owner: bendo01
--

CREATE TABLE academic_lecturer_transaction.homebases (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    lecturer_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    institution_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    status_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    contract_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_lecturer_transaction.homebases OWNER TO bendo01;

--
-- Name: evaluator_types; Type: TABLE; Schema: academic_prior_learning_recognition_reference; Owner: bendo01
--

CREATE TABLE academic_prior_learning_recognition_reference.evaluator_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    description text
);


ALTER TABLE academic_prior_learning_recognition_reference.evaluator_types OWNER TO bendo01;

--
-- Name: evidence_categories; Type: TABLE; Schema: academic_prior_learning_recognition_reference; Owner: bendo01
--

CREATE TABLE academic_prior_learning_recognition_reference.evidence_categories (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_prior_learning_recognition_reference.evidence_categories OWNER TO bendo01;

--
-- Name: evidence_types; Type: TABLE; Schema: academic_prior_learning_recognition_reference; Owner: bendo01
--

CREATE TABLE academic_prior_learning_recognition_reference.evidence_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    description text
);


ALTER TABLE academic_prior_learning_recognition_reference.evidence_types OWNER TO bendo01;

--
-- Name: professionalisms; Type: TABLE; Schema: academic_prior_learning_recognition_reference; Owner: bendo01
--

CREATE TABLE academic_prior_learning_recognition_reference.professionalisms (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    description text
);


ALTER TABLE academic_prior_learning_recognition_reference.professionalisms OWNER TO bendo01;

--
-- Name: decrees; Type: TABLE; Schema: academic_prior_learning_recognition_transaction; Owner: bendo01
--

CREATE TABLE academic_prior_learning_recognition_transaction.decrees (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    decree_number character varying(255) NOT NULL,
    decree_date date NOT NULL,
    evaluation_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_prior_learning_recognition_transaction.decrees OWNER TO bendo01;

--
-- Name: evaluation_details; Type: TABLE; Schema: academic_prior_learning_recognition_transaction; Owner: bendo01
--

CREATE TABLE academic_prior_learning_recognition_transaction.evaluation_details (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    evaluation_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    archive_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    evidence_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_prior_learning_recognition_transaction.evaluation_details OWNER TO bendo01;

--
-- Name: evaluations; Type: TABLE; Schema: academic_prior_learning_recognition_transaction; Owner: bendo01
--

CREATE TABLE academic_prior_learning_recognition_transaction.evaluations (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    recognition_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    course_evaluation_planning_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    professionalism_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    evidence_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    evaluator_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_prior_learning_recognition_transaction.evaluations OWNER TO bendo01;

--
-- Name: evaluators; Type: TABLE; Schema: academic_prior_learning_recognition_transaction; Owner: bendo01
--

CREATE TABLE academic_prior_learning_recognition_transaction.evaluators (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    evaluator_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    recognition_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_prior_learning_recognition_transaction.evaluators OWNER TO bendo01;

--
-- Name: recognitions; Type: TABLE; Schema: academic_prior_learning_recognition_transaction; Owner: bendo01
--

CREATE TABLE academic_prior_learning_recognition_transaction.recognitions (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    candidate_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    curriculum_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL
);


ALTER TABLE academic_prior_learning_recognition_transaction.recognitions OWNER TO bendo01;

--
-- Name: counsellors; Type: TABLE; Schema: academic_student_adviser; Owner: bendo01
--

CREATE TABLE academic_student_adviser.counsellors (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    decree_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    student_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    lecturer_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_adviser.counsellors OWNER TO bendo01;

--
-- Name: decrees; Type: TABLE; Schema: academic_student_adviser; Owner: bendo01
--

CREATE TABLE academic_student_adviser.decrees (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    decree_date date NOT NULL,
    decree_number character varying(255) NOT NULL,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    staff_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_adviser.decrees OWNER TO bendo01;

--
-- Name: convertions; Type: TABLE; Schema: academic_student_campaign; Owner: bendo01
--

CREATE TABLE academic_student_campaign.convertions (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    student_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    course_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    grade_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    transfer_code character varying(255) NOT NULL,
    transfer_name character varying(255) NOT NULL,
    transfer_credit double precision DEFAULT 0 NOT NULL,
    transfer_grade character varying(255) NOT NULL,
    is_lock timestamp(0) without time zone,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_campaign.convertions OWNER TO bendo01;

--
-- Name: detail_activities; Type: TABLE; Schema: academic_student_campaign; Owner: bendo01
--

CREATE TABLE academic_student_campaign.detail_activities (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    mark double precision DEFAULT '0'::double precision,
    credit double precision,
    grade_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    course_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    activity_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    teach_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    is_lock boolean DEFAULT false,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    name character varying,
    feeder_grade_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    curiculum_detail_sequence integer DEFAULT 0
);


ALTER TABLE academic_student_campaign.detail_activities OWNER TO bendo01;

--
-- Name: detail_activity_evaluation_components; Type: TABLE; Schema: academic_student_campaign; Owner: bendo01
--

CREATE TABLE academic_student_campaign.detail_activity_evaluation_components (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name text,
    detail_activity_id uuid NOT NULL,
    course_evaluation_planning_id uuid NOT NULL,
    mark real DEFAULT '0'::real,
    percentage real DEFAULT '0'::real,
    total real DEFAULT '0'::real,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_campaign.detail_activity_evaluation_components OWNER TO bendo01;

--
-- Name: student_activities; Type: TABLE; Schema: academic_student_campaign; Owner: bendo01
--

CREATE TABLE academic_student_campaign.student_activities (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255),
    cumulative_index double precision DEFAULT '0'::double precision NOT NULL,
    grand_cumulative_index double precision DEFAULT '0'::double precision NOT NULL,
    total_credit double precision DEFAULT 0,
    grand_total_credit double precision DEFAULT 0,
    student_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    unit_activity_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    status_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    resign_status_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    is_lock boolean DEFAULT false,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    finance_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    finance_fee double precision DEFAULT '0'::double precision
);


ALTER TABLE academic_student_campaign.student_activities OWNER TO bendo01;

--
-- Name: adviser_categories; Type: TABLE; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_reference.adviser_categories (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_reference.adviser_categories OWNER TO bendo01;

--
-- Name: approval_types; Type: TABLE; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_reference.approval_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_reference.approval_types OWNER TO bendo01;

--
-- Name: categories; Type: TABLE; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_reference.categories (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_reference.categories OWNER TO bendo01;

--
-- Name: requirements; Type: TABLE; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_reference.requirements (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    stage_id uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_reference.requirements OWNER TO bendo01;

--
-- Name: stages; Type: TABLE; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_reference.stages (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_reference.stages OWNER TO bendo01;

--
-- Name: varieties; Type: TABLE; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_reference.varieties (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_reference.varieties OWNER TO bendo01;

--
-- Name: advisers; Type: TABLE; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_transaction.advisers (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    thread integer NOT NULL,
    lecturer_id uuid NOT NULL,
    detail_activity_id uuid NOT NULL,
    submission_id uuid,
    adviser_category_id uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_transaction.advisers OWNER TO bendo01;

--
-- Name: evaluation_details; Type: TABLE; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_transaction.evaluation_details (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    evaluation_summary_id uuid NOT NULL,
    adviser_id uuid NOT NULL,
    mark real DEFAULT '0'::real,
    grade_id uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_transaction.evaluation_details OWNER TO bendo01;

--
-- Name: evaluation_summaries; Type: TABLE; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_transaction.evaluation_summaries (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    submission_id uuid,
    detail_activity_id uuid NOT NULL,
    stage_id uuid NOT NULL,
    mark real DEFAULT '0'::real,
    grade_id uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_transaction.evaluation_summaries OWNER TO bendo01;

--
-- Name: final_assignment_decrees; Type: TABLE; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_transaction.final_assignment_decrees (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    decree_number character varying(255) NOT NULL,
    decree_date date NOT NULL,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    activity_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    staff_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_transaction.final_assignment_decrees OWNER TO bendo01;

--
-- Name: prerequisites; Type: TABLE; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_transaction.prerequisites (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    thread integer NOT NULL,
    requirement_id uuid NOT NULL,
    submission_id uuid NOT NULL,
    approval_type_id uuid NOT NULL,
    stage_id uuid NOT NULL,
    filename character varying(255),
    dir character varying(255),
    type character varying(255),
    filesize integer,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_transaction.prerequisites OWNER TO bendo01;

--
-- Name: schedules; Type: TABLE; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_transaction.schedules (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    ecree_number character varying(255),
    schedule_date date,
    schedule_time time(0) without time zone,
    submission_id uuid,
    detail_activity_id uuid NOT NULL,
    stage_id uuid NOT NULL,
    room_id uuid,
    zoom_meeting text,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_transaction.schedules OWNER TO bendo01;

--
-- Name: submissions; Type: TABLE; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

CREATE TABLE academic_student_final_assignment_transaction.submissions (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    title text,
    student_id uuid NOT NULL,
    approval_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    category_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    stage_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    final_assignment_decree_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    detail_activity_id uuid NOT NULL,
    is_taken timestamp(0) without time zone,
    is_lock timestamp(0) without time zone,
    filename character varying(255),
    dir character varying(255),
    type character varying(255),
    filesize integer,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_final_assignment_transaction.submissions OWNER TO bendo01;

--
-- Name: images; Type: TABLE; Schema: academic_student_master; Owner: bendo01
--

CREATE TABLE academic_student_master.images (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    student_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    filename character varying(255) NOT NULL,
    dir character varying(255) NOT NULL,
    mimetype character varying(255),
    size bigint,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_master.images OWNER TO bendo01;

--
-- Name: students; Type: TABLE; Schema: academic_student_master; Owner: bendo01
--

CREATE TABLE academic_student_master.students (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    selection_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    registered date NOT NULL,
    individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    status_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    academic_year_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    registration_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    nisn character varying(255),
    resign_status_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    concentration_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    curriculum_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    class_code_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    transfer_code character varying(255),
    transfer_unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    feeder_registration_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    finance_fee double precision DEFAULT 0,
    finance_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_master.students OWNER TO bendo01;

--
-- Name: finances; Type: TABLE; Schema: academic_student_reference; Owner: bendo01
--

CREATE TABLE academic_student_reference.finances (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_reference.finances OWNER TO bendo01;

--
-- Name: registrations; Type: TABLE; Schema: academic_student_reference; Owner: bendo01
--

CREATE TABLE academic_student_reference.registrations (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_reference.registrations OWNER TO bendo01;

--
-- Name: resign_statuses; Type: TABLE; Schema: academic_student_reference; Owner: bendo01
--

CREATE TABLE academic_student_reference.resign_statuses (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_reference.resign_statuses OWNER TO bendo01;

--
-- Name: selection_types; Type: TABLE; Schema: academic_student_reference; Owner: bendo01
--

CREATE TABLE academic_student_reference.selection_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_reference.selection_types OWNER TO bendo01;

--
-- Name: statuses; Type: TABLE; Schema: academic_student_reference; Owner: bendo01
--

CREATE TABLE academic_student_reference.statuses (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_student_reference.statuses OWNER TO bendo01;

--
-- Name: answers; Type: TABLE; Schema: academic_survey_master; Owner: bendo01
--

CREATE TABLE academic_survey_master.answers (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    question_id uuid NOT NULL,
    point double precision DEFAULT '0'::double precision NOT NULL,
    suggestion text,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_survey_master.answers OWNER TO bendo01;

--
-- Name: bundle_question; Type: TABLE; Schema: academic_survey_master; Owner: bendo01
--

CREATE TABLE academic_survey_master.bundle_question (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    bundle_id uuid NOT NULL,
    question_id uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_survey_master.bundle_question OWNER TO bendo01;

--
-- Name: bundles; Type: TABLE; Schema: academic_survey_master; Owner: bendo01
--

CREATE TABLE academic_survey_master.bundles (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    institution_id uuid NOT NULL,
    bundle_category_id uuid NOT NULL,
    unit_id uuid,
    suggestion text,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_survey_master.bundles OWNER TO bendo01;

--
-- Name: questions; Type: TABLE; Schema: academic_survey_master; Owner: bendo01
--

CREATE TABLE academic_survey_master.questions (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255),
    name text NOT NULL,
    institution_id uuid NOT NULL,
    question_variety_id uuid,
    suggestion text,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_survey_master.questions OWNER TO bendo01;

--
-- Name: bundle_categories; Type: TABLE; Schema: academic_survey_reference; Owner: bendo01
--

CREATE TABLE academic_survey_reference.bundle_categories (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_survey_reference.bundle_categories OWNER TO bendo01;

--
-- Name: question_varieties; Type: TABLE; Schema: academic_survey_reference; Owner: bendo01
--

CREATE TABLE academic_survey_reference.question_varieties (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_survey_reference.question_varieties OWNER TO bendo01;

--
-- Name: conducts; Type: TABLE; Schema: academic_survey_transaction; Owner: bendo01
--

CREATE TABLE academic_survey_transaction.conducts (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    alphabet_code character varying(255),
    name text NOT NULL,
    bundle_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    conductable_type character varying(255) NOT NULL,
    conductable_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    is_finish boolean DEFAULT false NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_survey_transaction.conducts OWNER TO bendo01;

--
-- Name: responds; Type: TABLE; Schema: academic_survey_transaction; Owner: bendo01
--

CREATE TABLE academic_survey_transaction.responds (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name text,
    conduct_id uuid NOT NULL,
    bundle_id uuid NOT NULL,
    question_id uuid NOT NULL,
    answer_id uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE academic_survey_transaction.responds OWNER TO bendo01;

--
-- Name: permission_position_type; Type: TABLE; Schema: auth; Owner: bendo01
--

CREATE TABLE auth.permission_position_type (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    permission_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    position_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE auth.permission_position_type OWNER TO bendo01;

--
-- Name: permission_user; Type: TABLE; Schema: auth; Owner: bendo01
--

CREATE TABLE auth.permission_user (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    user_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    permission_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE auth.permission_user OWNER TO bendo01;

--
-- Name: permissions; Type: TABLE; Schema: auth; Owner: bendo01
--

CREATE TABLE auth.permissions (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    uri text,
    is_open boolean DEFAULT false,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE auth.permissions OWNER TO bendo01;

--
-- Name: roles; Type: TABLE; Schema: auth; Owner: bendo01
--

CREATE TABLE auth.roles (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name character varying(255) NOT NULL,
    user_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    position_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp(6) without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    roleable_id uuid,
    roleable_type character varying(255)
);


ALTER TABLE auth.roles OWNER TO bendo01;

--
-- Name: user_position_type; Type: TABLE; Schema: auth; Owner: bendo01
--

CREATE TABLE auth.user_position_type (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    user_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    position_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE auth.user_position_type OWNER TO bendo01;

--
-- Name: users; Type: TABLE; Schema: auth; Owner: bendo01
--

CREATE TABLE auth.users (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    email character varying(255) NOT NULL,
    email_verified_at timestamp(0) without time zone,
    password character varying(255) NOT NULL,
    remember_token character varying(100),
    individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    is_active boolean DEFAULT true,
    current_role_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    pid uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    api_key character varying,
    reset_token character varying,
    email_verification_token character varying,
    reset_sent_at timestamp(6) without time zone,
    email_verification_sent_at timestamp(6) without time zone,
    magic_link_token character varying,
    magic_link_expiration timestamp(6) without time zone,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE auth.users OWNER TO bendo01;

--
-- Name: verifications; Type: TABLE; Schema: auth; Owner: bendo01
--

CREATE TABLE auth.verifications (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    email character varying(255),
    token character varying(255) NOT NULL,
    is_password_recovery boolean DEFAULT false NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE auth.verifications OWNER TO bendo01;

--
-- Name: buildings; Type: TABLE; Schema: building_master; Owner: bendo01
--

CREATE TABLE building_master.buildings (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    long real DEFAULT 0,
    wide real DEFAULT 0,
    high real DEFAULT 0,
    variety_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    category_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    total_floor integer DEFAULT 1,
    residence_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    condition_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE building_master.buildings OWNER TO bendo01;

--
-- Name: rooms; Type: TABLE; Schema: building_master; Owner: bendo01
--

CREATE TABLE building_master.rooms (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    alphabet_code character varying(255),
    name character varying(255) NOT NULL,
    long real DEFAULT 0,
    wide real DEFAULT 0,
    high real DEFAULT 0,
    room_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    building_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    condition_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE building_master.rooms OWNER TO bendo01;

--
-- Name: categories; Type: TABLE; Schema: building_reference; Owner: bendo01
--

CREATE TABLE building_reference.categories (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE building_reference.categories OWNER TO bendo01;

--
-- Name: conditions; Type: TABLE; Schema: building_reference; Owner: bendo01
--

CREATE TABLE building_reference.conditions (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE building_reference.conditions OWNER TO bendo01;

--
-- Name: room_types; Type: TABLE; Schema: building_reference; Owner: bendo01
--

CREATE TABLE building_reference.room_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE building_reference.room_types OWNER TO bendo01;

--
-- Name: varieties; Type: TABLE; Schema: building_reference; Owner: bendo01
--

CREATE TABLE building_reference.varieties (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE building_reference.varieties OWNER TO bendo01;

--
-- Name: electronic_mails; Type: TABLE; Schema: contact_master; Owner: bendo01
--

CREATE TABLE contact_master.electronic_mails (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    email_address character varying(255) NOT NULL,
    electronic_mail_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    electronic_mailable_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    electronic_mailable_type character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE contact_master.electronic_mails OWNER TO bendo01;

--
-- Name: phones; Type: TABLE; Schema: contact_master; Owner: bendo01
--

CREATE TABLE contact_master.phones (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    phone_number character varying(255) NOT NULL,
    phone_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    phoneable_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    phoneable_type character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE contact_master.phones OWNER TO bendo01;

--
-- Name: residences; Type: TABLE; Schema: contact_master; Owner: bendo01
--

CREATE TABLE contact_master.residences (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    street character varying(255) NOT NULL,
    citizens_association integer DEFAULT 0 NOT NULL,
    neighborhood_association integer DEFAULT 0 NOT NULL,
    province_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    regency_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    sub_district_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    village_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    residence_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    residenceable_type character varying(255),
    residenceable_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    latitude double precision,
    longitude double precision,
    zoom integer,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE contact_master.residences OWNER TO bendo01;

--
-- Name: websites; Type: TABLE; Schema: contact_master; Owner: bendo01
--

CREATE TABLE contact_master.websites (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    website_url character varying(255) NOT NULL,
    website_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    websiteable_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    websiteable_type character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE contact_master.websites OWNER TO bendo01;

--
-- Name: electronic_mail_types; Type: TABLE; Schema: contact_reference; Owner: bendo01
--

CREATE TABLE contact_reference.electronic_mail_types (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE contact_reference.electronic_mail_types OWNER TO bendo01;

--
-- Name: phone_types; Type: TABLE; Schema: contact_reference; Owner: bendo01
--

CREATE TABLE contact_reference.phone_types (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE contact_reference.phone_types OWNER TO bendo01;

--
-- Name: residence_types; Type: TABLE; Schema: contact_reference; Owner: bendo01
--

CREATE TABLE contact_reference.residence_types (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE contact_reference.residence_types OWNER TO bendo01;

--
-- Name: website_types; Type: TABLE; Schema: contact_reference; Owner: bendo01
--

CREATE TABLE contact_reference.website_types (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE contact_reference.website_types OWNER TO bendo01;

--
-- Name: archive_types; Type: TABLE; Schema: document_reference; Owner: bendo01
--

CREATE TABLE document_reference.archive_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE document_reference.archive_types OWNER TO bendo01;

--
-- Name: archives; Type: TABLE; Schema: document_transaction; Owner: bendo01
--

CREATE TABLE document_transaction.archives (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    dir character varying(255) NOT NULL,
    mimetype character varying(255) NOT NULL,
    size integer DEFAULT 0,
    archiveable_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    archiveable_type character varying(255) NOT NULL,
    archive_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE document_transaction.archives OWNER TO bendo01;

--
-- Name: estimasi; Type: TABLE; Schema: feeder_akumulasi; Owner: bendo01
--

CREATE TABLE feeder_akumulasi.estimasi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    institution_id uuid NOT NULL,
    total_data_per_request integer DEFAULT 0,
    last_offset integer DEFAULT 0,
    total_data integer DEFAULT 0,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_akumulasi.estimasi OWNER TO bendo01;

--
-- Name: jumlah_data; Type: TABLE; Schema: feeder_akumulasi; Owner: bendo01
--

CREATE TABLE feeder_akumulasi.jumlah_data (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    total_app integer DEFAULT 0,
    total_feeder integer DEFAULT 0,
    institution_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_akumulasi.jumlah_data OWNER TO bendo01;

--
-- Name: kredential; Type: TABLE; Schema: feeder_akun; Owner: bendo01
--

CREATE TABLE feeder_akun.kredential (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    username character varying(255) NOT NULL,
    password character varying(255) NOT NULL,
    institution_id uuid NOT NULL,
    service_url text NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_akun.kredential OWNER TO bendo01;

--
-- Name: aktifitas_kuliah_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.aktifitas_kuliah_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_registrasi_mahasiswa uuid,
    id_mahasiswa uuid,
    id_semester character varying(255),
    nama_semester character varying(255),
    nim character varying(255),
    nama_mahasiswa character varying(255),
    angkatan character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    id_status_mahasiswa character varying(255),
    nama_status_mahasiswa character varying(255),
    ips character varying(255),
    ipk character varying(255),
    sks_semester character varying(255),
    sks_total character varying(255),
    biaya_kuliah_smt character varying(255),
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.aktifitas_kuliah_mahasiswa OWNER TO bendo01;

--
-- Name: aktifitas_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.aktifitas_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    asal_data character varying(255),
    nm_asaldata character varying(255),
    id_aktivitas uuid,
    jenis_anggota character varying(255),
    nama_jenis_anggota character varying(255),
    id_jenis_aktivitas uuid,
    nama_jenis_aktivitas character varying(255),
    id_prodi uuid,
    nama_prodi character varying(255),
    id_semester uuid,
    nama_semester character varying(255),
    judul character varying(255),
    keterangan character varying(255),
    lokasi character varying(255),
    sk_tugas character varying(255),
    tanggal_sk_tugas date,
    untuk_kampus_merdeka integer DEFAULT 0,
    tanggal_mulai date,
    tanggal_selesai date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.aktifitas_mahasiswa OWNER TO bendo01;

--
-- Name: aktifitas_mengajar_dosen; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.aktifitas_mengajar_dosen (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_registrasi_dosen uuid,
    id_dosen uuid,
    nama_dosen character varying(255),
    id_periode character varying(255),
    nama_periode character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    id_matkul uuid,
    nama_mata_kuliah character varying(255),
    id_kelas uuid,
    nama_kelas_kuliah character varying(255),
    rencana_minggu_pertemuan character varying(255),
    realisasi_minggu_pertemuan character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.aktifitas_mengajar_dosen OWNER TO bendo01;

--
-- Name: anggota_aktifitas_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.anggota_aktifitas_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_aktivitas uuid,
    judul character varying(255),
    id_anggota uuid,
    id_registrasi_mahasiswa uuid,
    nim character varying(255),
    nama_mahasiswa character varying(255),
    jenis_peran character varying(255),
    nama_jenis_peran character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.anggota_aktifitas_mahasiswa OWNER TO bendo01;

--
-- Name: bidang_minat_perguruan_tinggi; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.bidang_minat_perguruan_tinggi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_bidang_minat uuid,
    nm_bidang_minat character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    smt_dimulai integer,
    sk_bidang_minat integer,
    tamat_sk_bidang_minat integer,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.bidang_minat_perguruan_tinggi OWNER TO bendo01;

--
-- Name: bimbing_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.bimbing_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_aktivitas uuid,
    judul character varying(255),
    id_bimbing_mahasiswa uuid,
    id_kategori_kegiatan uuid,
    nama_kategori_kegiatan character varying(255),
    id_dosen uuid,
    nidn character varying(255),
    nama_dosen character varying(255),
    pembimbing_ke integer,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.bimbing_mahasiswa OWNER TO bendo01;

--
-- Name: biodata_dosen; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.biodata_dosen (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_dosen character varying(255),
    nama_dosen character varying(255),
    tempat_lahir character varying(255),
    tanggal_lahir date,
    jenis_kelamin character varying(255),
    id_agama character varying(255),
    nama_agama character varying(255),
    id_status_aktif character varying(255),
    nama_status_aktif character varying(255),
    nidn character varying(255),
    nama_ibu_kandung character varying(255),
    nik character varying(255),
    nip character varying(255),
    npwp character varying(255),
    id_jenis_sdm character varying(255),
    nama_jenis_sdm character varying(255),
    no_sk_cpns character varying(255),
    tanggal_sk_cpns date,
    no_sk_pengangkatan character varying(255),
    mulai_sk_pengangkatan character varying(255),
    id_lembaga_pengangkatan character varying(255),
    nama_lembaga_pengangkatan character varying(255),
    id_pangkat_golongan character varying(255),
    nama_pangkat_golongan character varying(255),
    id_sumber_gaji character varying(255),
    nama_sumber_gaji character varying(255),
    jalan character varying(255),
    dusun character varying(255),
    rt character varying(255),
    rw character varying(255),
    ds_kel character varying(255),
    kode_pos character varying(255),
    id_wilayah character varying(255),
    nama_wilayah character varying(255),
    telepon character varying(255),
    handphone character varying(255),
    email character varying(255),
    status_pernikahan character varying(255),
    nama_suami_istri character varying(255),
    nip_suami_istri character varying(255),
    tanggal_mulai_pns date,
    id_pekerjaan_suami_istri character varying(255),
    nama_pekerjaan_suami_istri character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.biodata_dosen OWNER TO bendo01;

--
-- Name: biodata_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.biodata_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    nama_mahasiswa character varying(255),
    jenis_kelamin character varying(255),
    tempat_lahir character varying(255),
    tanggal_lahir date,
    id_mahasiswa uuid,
    id_agama character varying(255),
    nama_agama character varying(255),
    nik character varying(255),
    nisn character varying(255),
    npwp character varying(255),
    id_negara character varying(255),
    kewarganegaraan character varying(255),
    jalan character varying(255),
    dusun character varying(255),
    rt character varying(255),
    rw character varying(255),
    kelurahan character varying(255),
    kode_pos character varying(255),
    id_wilayah character varying(255),
    nama_wilayah character varying(255),
    id_jenis_tinggal character varying(255),
    nama_jenis_tinggal character varying(255),
    id_alat_transportasi character varying(255),
    nama_alat_transportasi character varying(255),
    telepon character varying(255),
    handphone character varying(255),
    email character varying(255),
    penerima_kps character varying(255),
    nomor_kps character varying(255),
    nik_ayah character varying(255),
    nama_ayah character varying(255),
    tanggal_lahir_ayah date,
    id_pendidikan_ayah character varying(255),
    nama_pendidikan_ayah character varying(255),
    id_pekerjaan_ayah character varying(255),
    nama_pekerjaan_ayah character varying(255),
    id_penghasilan_ayah character varying(255),
    nama_penghasilan_ayah character varying(255),
    nik_ibu character varying(255),
    nama_ibu_kandung character varying(255),
    tanggal_lahir_ibu character varying(255),
    id_pendidikan_ibu character varying(255),
    nama_pendidikan_ibu character varying(255),
    id_pekerjaan_ibu character varying(255),
    nama_pekerjaan_ibu character varying(255),
    id_penghasilan_ibu character varying(255),
    nama_penghasilan_ibu character varying(255),
    nama_wali character varying(255),
    tanggal_lahir_wali character varying(255),
    id_pendidikan_wali character varying(255),
    nama_pendidikan_wali character varying(255),
    id_pekerjaan_wali character varying(255),
    nama_pekerjaan_wali character varying(255),
    id_penghasilan_wali character varying(255),
    nama_penghasilan_wali character varying(255),
    id_kebutuhan_khusus_mahasiswa character varying(255),
    nama_kebutuhan_khusus_mahasiswa character varying(255),
    id_kebutuhan_khusus_ayah character varying(255),
    nama_kebutuhan_khusus_ayah character varying(255),
    id_kebutuhan_khusus_ibu character varying(255),
    nama_kebutuhan_khusus_ibu character varying(255),
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.biodata_mahasiswa OWNER TO bendo01;

--
-- Name: dosen; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.dosen (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_dosen uuid,
    nama_dosen character varying(255),
    nidn character varying(255),
    nip character varying(255),
    jenis_kelamin character varying(255),
    id_agama character varying(255),
    nama_agama character varying(255),
    tanggal_lahir date,
    id_status_aktif character varying(255),
    nama_status_aktif character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.dosen OWNER TO bendo01;

--
-- Name: dosen_pembimbing; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.dosen_pembimbing (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_registrasi_mahasiswa uuid,
    nama_mahasiswa character varying(255),
    nim character varying(255),
    id_dosen uuid,
    nidn character varying(255),
    nama_dosen character varying(255),
    pembimbing_ke integer,
    jenis_aktivitas character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.dosen_pembimbing OWNER TO bendo01;

--
-- Name: dosen_pengajar_kelas_kuliah; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.dosen_pengajar_kelas_kuliah (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_aktivitas_mengajar uuid,
    id_registrasi_dosen uuid,
    id_dosen uuid,
    nidn character varying(255),
    nama_dosen character varying(255),
    id_kelas_kuliah uuid,
    nama_kelas_kuliah character varying(255),
    id_substansi uuid,
    sks_substansi_total character varying(255),
    rencana_minggu_pertemuan character varying(255),
    realisasi_minggu_pertemuan character varying(255),
    id_jenis_evaluasi character varying(255),
    nama_jenis_evaluasi character varying(255),
    id_prodi uuid,
    id_semester character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.dosen_pengajar_kelas_kuliah OWNER TO bendo01;

--
-- Name: fakultas; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.fakultas (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_fakultas uuid,
    nama_fakultas character varying(255),
    status character varying(255),
    id_jenjang_pendidikan uuid,
    nama_jenjang_pendidikan character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.fakultas OWNER TO bendo01;

--
-- Name: hitung_transkrip_angkatan_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.hitung_transkrip_angkatan_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    angkatan integer,
    id_prodi uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.hitung_transkrip_angkatan_mahasiswa OWNER TO bendo01;

--
-- Name: kartu_rencana_studi_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.kartu_rencana_studi_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_registrasi_mahasiswa uuid,
    id_periode character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    id_matkul uuid,
    kode_mata_kuliah character varying(255),
    nama_mata_kuliah character varying(255),
    id_kelas uuid,
    nama_kelas_kuliah character varying(255),
    sks_mata_kuliah character varying(255),
    nim character varying(255),
    nama_mahasiswa character varying(255),
    angkatan character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.kartu_rencana_studi_mahasiswa OWNER TO bendo01;

--
-- Name: kelas_kuliah; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.kelas_kuliah (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_kelas_kuliah uuid,
    id_prodi uuid,
    nama_program_studi character varying(255),
    id_semester character varying(255),
    nama_semester character varying(255),
    id_matkul uuid,
    kode_mata_kuliah character varying(255),
    nama_mata_kuliah character varying(255),
    nama_kelas_kuliah character varying(255),
    sks character varying(255),
    id_dosen text,
    nama_dosen text,
    jumlah_mahasiswa character varying(255),
    apa_untuk_pditt character varying(255),
    bahasan text,
    tanggal_mulai_efektif date,
    tanggal_akhir_efektif date,
    kapasitas character varying(255),
    tanggal_tutup_daftar date,
    prodi_penyelenggara character varying(255),
    perguruan_tinggi_penyelenggara character varying(255),
    jumlah_mahasiswa_dapat_nilai character varying(255),
    sks_tm character varying(255),
    sks_prak character varying(255),
    sks_prak_lap character varying(255),
    sks_sim character varying(255),
    a_selenggara_pditt character varying(255),
    a_pengguna_pditt character varying(255),
    kuota_pditt character varying(255),
    tgl_mulai_koas date,
    tgl_selesai_koas date,
    id_mou character varying(255),
    id_kls_pditt character varying(255),
    tgl_create date,
    lingkup_kelas character varying(255),
    mode_kuliah character varying(255),
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.kelas_kuliah OWNER TO bendo01;

--
-- Name: komponen_evaluasi_kelas; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.komponen_evaluasi_kelas (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_komponen_evaluasi uuid,
    id_kelas_kuliah uuid,
    id_jenis_evaluasi integer,
    nama character varying(255),
    nama_inggris character varying(255),
    nomor_urut integer DEFAULT 0,
    bobot_evaluasi character varying(255),
    last_update date,
    tgl_create date,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    deleted_at timestamp without time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.komponen_evaluasi_kelas OWNER TO bendo01;

--
-- Name: konsistensi_data; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.konsistensi_data (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    total integer NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.konsistensi_data OWNER TO bendo01;

--
-- Name: konversi_kampus_merdeka; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.konversi_kampus_merdeka (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_semester uuid,
    nama_semester character varying(255),
    id_konversi_aktivitas uuid,
    id_matkul uuid,
    nama_mata_kuliah character varying(255),
    id_aktivitas uuid,
    judul character varying(255),
    id_anggota uuid,
    nama_mahasiswa character varying(255),
    nim integer,
    sks_mata_kuliah integer,
    nilai_angka integer,
    nilai_indeks integer,
    nilai_huruf integer,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.konversi_kampus_merdeka OWNER TO bendo01;

--
-- Name: kurikulum; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.kurikulum (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_kurikulum uuid,
    jml_sem_normal character varying(255),
    id_jenj_didik character varying(255),
    nama_kurikulum character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    id_semester character varying(255),
    semester_mulai_berlaku character varying(255),
    jumlah_sks_lulus character varying(255),
    jumlah_sks_wajib character varying(255),
    jumlah_sks_pilihan character varying(255),
    jumlah_sks_mata_kuliah_wajib character varying(255),
    jumlah_sks_mata_kuliah_pilihan character varying(255),
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.kurikulum OWNER TO bendo01;

--
-- Name: mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    nama_mahasiswa character varying(255),
    jenis_kelamin character varying(255),
    tanggal_lahir date,
    id_perguruan_tinggi uuid,
    nipd character varying(255),
    ipk character varying(255),
    total_sks character varying(255),
    id_sms uuid,
    id_mahasiswa uuid,
    id_agama character varying(255),
    nama_agama character varying(255),
    id_prodi character varying(255),
    nama_program_studi character varying(255),
    id_status_mahasiswa character varying(255),
    nama_status_mahasiswa character varying(255),
    nim character varying(255),
    id_periode character varying(255),
    nama_periode_masuk character varying(255),
    id_registrasi_mahasiswa uuid,
    id_periode_keluar character varying(255),
    tanggal_keluar date,
    last_update date,
    tgl_create date,
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.mahasiswa OWNER TO bendo01;

--
-- Name: mahasiswa_bimbingan_dosen; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.mahasiswa_bimbingan_dosen (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_aktivitas uuid,
    judul character varying(255),
    id_bimbing_mahasiswa uuid,
    id_kategori_kegiatan uuid,
    nama_kategori_kegiatan character varying(255),
    id_dosen uuid,
    nidn integer,
    nama_dosen character varying(255),
    pembimbing_ke integer,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.mahasiswa_bimbingan_dosen OWNER TO bendo01;

--
-- Name: mahasiswa_lulus_dropout; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.mahasiswa_lulus_dropout (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_registrasi_mahasiswa uuid,
    id_mahasiswa uuid,
    nim character varying(255),
    nama_mahasiswa character varying(255),
    id_jenis_keluar uuid,
    nama_jenis_keluar character varying(255),
    tanggal_keluar date,
    keterangan character varying(255),
    nomor_sk_yudisium character varying(255),
    tanggal_sk_yudisium date,
    ipk integer,
    nomor_ijazah integer,
    jalur_skripsi character varying(255),
    judul_skripsi character varying(255),
    bulan_awal_bimbingan date,
    bulan_akhir_bimbingan date,
    id_dosen uuid,
    nidn character varying(255),
    nama_dosen character varying(255),
    pembimbing_ke integer,
    id_periode_keluar uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.mahasiswa_lulus_dropout OWNER TO bendo01;

--
-- Name: matakuliah; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.matakuliah (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_matkul uuid,
    kode_mata_kuliah character varying(255),
    nama_mata_kuliah character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    id_jenis_mata_kuliah character varying(255),
    id_kelompok_mata_kuliah character varying(255),
    sks_mata_kuliah character varying(255),
    sks_tatap_muka character varying(255),
    sks_praktek character varying(255),
    sks_praktek_lapangan character varying(255),
    sks_simulasi character varying(255),
    metode_kuliah text,
    ada_sap character varying(255),
    ada_silabus character varying(255),
    ada_bahan_ajar character varying(255),
    ada_acara_praktek character varying(255),
    ada_diktat character varying(255),
    tanggal_mulai_efektif timestamp with time zone,
    tanggal_selesai_efektif timestamp with time zone,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.matakuliah OWNER TO bendo01;

--
-- Name: matakuliah_kurikulum; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.matakuliah_kurikulum (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    tgl_create date,
    id_kurikulum uuid,
    nama_kurikulum character varying(255),
    id_matkul uuid,
    kode_mata_kuliah character varying(255),
    nama_mata_kuliah character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    semester character varying(255),
    id_semester character varying(255),
    semester_mulai_berlaku character varying(255),
    sks_mata_kuliah character varying(255),
    sks_tatap_muka character varying(255),
    sks_praktek character varying(255),
    sks_praktek_lapangan character varying(255),
    sks_simulasi character varying(255),
    apakah_wajib character varying(255),
    status_sync character varying(255),
    sync_at timestamp(0) without time zone,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.matakuliah_kurikulum OWNER TO bendo01;

--
-- Name: nilai_perkuliahan_kelas; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.nilai_perkuliahan_kelas (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_prodi uuid,
    nama_program_studi character varying(255),
    id_semester character varying(255),
    nama_semester character varying(255),
    id_matkul uuid,
    kode_mata_kuliah character varying(255),
    nama_mata_kuliah character varying(255),
    sks_mata_kuliah character varying(255),
    id_kelas_kuliah uuid,
    nama_kelas_kuliah character varying(255),
    id_registrasi_mahasiswa uuid,
    id_mahasiswa uuid,
    nim character varying(255),
    nama_mahasiswa character varying(255),
    jurusan character varying(255),
    angkatan character varying(255),
    nilai_angka character varying(255),
    nilai_indeks character varying(255),
    nilai_huruf character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.nilai_perkuliahan_kelas OWNER TO bendo01;

--
-- Name: nilai_transfer_pendidikan_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.nilai_transfer_pendidikan_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_transfer uuid,
    id_registrasi_mahasiswa uuid,
    nim character varying(255),
    nama_mahasiswa character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    id_periode_masuk character varying(255),
    kode_mata_kuliah_asal character varying(255),
    nama_mata_kuliah_asal character varying(255),
    sks_mata_kuliah_asal character varying(255),
    nilai_huruf_asal character varying(255),
    id_matkul uuid,
    kode_matkul_diakui character varying(255),
    nama_mata_kuliah_diakui character varying(255),
    sks_mata_kuliah_diakui character varying(255),
    nilai_huruf_diakui character varying(255),
    nilai_angka_diakui character varying(255),
    id_perguruan_tinggi uuid,
    id_aktivitas character varying(255),
    judul text,
    id_jenis_aktivitas character varying(255),
    nama_jenis_aktivitas character varying(255),
    id_semester character varying(255),
    nama_semester character varying(255),
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.nilai_transfer_pendidikan_mahasiswa OWNER TO bendo01;

--
-- Name: penugasan_dosen; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.penugasan_dosen (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_registrasi_dosen uuid,
    jk character varying(255),
    id_dosen uuid,
    nama_dosen character varying(255),
    nidn character varying(255),
    id_tahun_ajaran character varying(255),
    nama_tahun_ajaran character varying(255),
    id_perguruan_tinggi uuid,
    nama_perguruan_tinggi character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    nomor_surat_tugas character varying(255),
    tanggal_surat_tugas date,
    mulai_surat_tugas date,
    tgl_create date,
    tgl_ptk_keluar timestamp(0) without time zone,
    id_stat_pegawai character varying(255),
    id_jns_keluar character varying(255),
    id_ikatan_kerja character varying(255),
    a_sp_homebase character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.penugasan_dosen OWNER TO bendo01;

--
-- Name: perguruan_tinggi; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.perguruan_tinggi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_perguruan_tinggi uuid,
    kode_perguruan_tinggi character varying(255),
    nama_perguruan_tinggi character varying(255),
    nama_singkat character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.perguruan_tinggi OWNER TO bendo01;

--
-- Name: periode_aktif; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.periode_aktif (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_prodi uuid,
    kode_prodi character varying(255),
    nama_program_studi character varying(255),
    status_prodi character varying(255),
    jenjang_pendidikan character varying(255),
    periode_pelaporan character varying(255),
    tipe_periode character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.periode_aktif OWNER TO bendo01;

--
-- Name: periode_perkuliahan; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.periode_perkuliahan (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_prodi uuid,
    nama_program_studi character varying(255),
    id_semester character varying(255),
    nama_semester character varying(255),
    jumlah_target_mahasiswa_baru character varying(255),
    tanggal_awal_perkuliahan date,
    tanggal_akhir_perkuliahan date,
    calon_ikut_seleksi character varying(255),
    calon_lulus_seleksi character varying(255),
    daftar_sbg_mhs character varying(255),
    pst_undur_diri character varying(255),
    jml_mgu_kul character varying(255),
    metode_kul text,
    metode_kul_eks text,
    tgl_create date,
    last_update date,
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.periode_perkuliahan OWNER TO bendo01;

--
-- Name: perkuliahan_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.perkuliahan_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_registrasi_mahasiswa uuid,
    nim character varying(255),
    nama_mahasiswa character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    angkatan character varying(255),
    id_periode_masuk character varying(255),
    id_semester character varying(255),
    nama_semester character varying(255),
    id_status_mahasiswa character varying(255),
    nama_status_mahasiswa character varying(255),
    ips character varying(255),
    ipk character varying(255),
    sks_semester character varying(255),
    sks_total character varying(255),
    biaya_kuliah_smt character varying(255),
    id_pembiayaan character varying(255),
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.perkuliahan_mahasiswa OWNER TO bendo01;

--
-- Name: peserta_kelas_kuliah; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.peserta_kelas_kuliah (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_kelas_kuliah uuid,
    nama_kelas_kuliah character varying(255),
    id_registrasi_mahasiswa uuid,
    id_mahasiswa uuid,
    nim character varying(255),
    nama_mahasiswa character varying(255),
    id_matkul uuid,
    kode_mata_kuliah character varying(255),
    nama_mata_kuliah character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    angkatan character varying(255),
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.peserta_kelas_kuliah OWNER TO bendo01;

--
-- Name: prestasi_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.prestasi_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_prestasi uuid,
    id_mahasiswa uuid,
    nama_mahasiswa character varying(255),
    id_jenis_prestasi uuid,
    nama_jenis_prestasi character varying(255),
    id_tingkat_prestasi uuid,
    nama_tingkat_prestasi character varying(255),
    nama_prestasi character varying(255),
    tahun_prestasi integer,
    penyelenggara character varying(255),
    peringkat integer,
    id_aktivitas uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.prestasi_mahasiswa OWNER TO bendo01;

--
-- Name: profil_perguruan_tinggi; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.profil_perguruan_tinggi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_perguruan_tinggi uuid,
    kode_perguruan_tinggi character varying(255),
    nama_perguruan_tinggi character varying(255),
    telepon character varying(255),
    faximile character varying(255),
    email character varying(255),
    website character varying(255),
    jalan character varying(255),
    dusun character varying(255),
    rt_rw integer,
    kelurahan character varying(255),
    kode_pos character varying(255),
    id_wilayah character varying(255),
    nama_wilayah character varying(255),
    lintang_bujur character varying(255),
    bank character varying(255),
    unit_cabang character varying(255),
    nomor_rekening character varying(255),
    mbs character varying(255),
    luas_tanah_milik character varying(255),
    luas_tanah_bukan_milik character varying(255),
    sk_pendirian character varying(255),
    tanggal_sk_pendirian character varying(255),
    id_status_milik character varying(255),
    nama_status_milik character varying(255),
    status_perguruan_tinggi character varying(255),
    sk_izin_operasional character varying(255),
    tanggal_izin_operasional date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.profil_perguruan_tinggi OWNER TO bendo01;

--
-- Name: program_studi; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.program_studi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_perguruan_tinggi uuid,
    kode_perguruan_tinggi character varying(255),
    nama_perguruan_tinggi character varying(255),
    id_prodi uuid,
    kode_program_studi character varying(255),
    nama_program_studi character varying(255),
    status character varying(255),
    id_jenjang_pendidikan character varying(255),
    nama_jenjang_pendidikan character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.program_studi OWNER TO bendo01;

--
-- Name: rencana_evaluasi; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.rencana_evaluasi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jenis_evaluasi character varying(255),
    id_rencana_evaluasi uuid,
    jenis_evaluasi character varying(255),
    id_matkul uuid,
    nama_mata_kuliah character varying(255),
    kode_mata_kuliah character varying(255),
    sks_mata_kuliah character varying(255),
    id_prodi uuid,
    nama_program_studi character varying(255),
    nama_evaluasi character varying(255),
    deskripsi_indonesia text,
    deskrips_inggris text,
    nomor_urut character varying(255),
    bobot_evaluasi character varying(255),
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.rencana_evaluasi OWNER TO bendo01;

--
-- Name: riwayat_fungsional_dosen; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.riwayat_fungsional_dosen (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_dosen uuid,
    nidn character varying(255),
    nama_dosen character varying(255),
    id_jabatan_fungsional uuid,
    nama_jabatan_fungsional character varying(255),
    sk_jabatan_fungsional character varying(255),
    mulai_sk_jabatan date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.riwayat_fungsional_dosen OWNER TO bendo01;

--
-- Name: riwayat_nilai_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.riwayat_nilai_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_registrasi_mahasiswa uuid,
    id_prodi uuid,
    nama_program_studi character varying(255),
    id_periode character varying(255),
    id_matkul uuid,
    nama_mata_kuliah character varying(255),
    id_kelas uuid,
    nama_kelas_kuliah character varying(255),
    sks_mata_kuliah character varying(255),
    nilai_angka character varying(255),
    nilai_huruf character varying(255),
    nilai_indeks character varying(255),
    nim character varying(255),
    nama_mahasiswa character varying(255),
    angkatan character varying(255),
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.riwayat_nilai_mahasiswa OWNER TO bendo01;

--
-- Name: riwayat_pangkat_dosen; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.riwayat_pangkat_dosen (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_dosen uuid,
    nidn character varying(255),
    nama_dosen character varying(255),
    id_pangkat_golongan uuid,
    nama_pangkat_golongan character varying(255),
    sk_pangkat character varying(255),
    tanggal_sk_pangkat date,
    mulai_sk_pangkat date,
    masa_kerja_dalam_tahun character varying(255),
    masa_kerja_dalam_bulan character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.riwayat_pangkat_dosen OWNER TO bendo01;

--
-- Name: riwayat_pendidikan_dosen; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.riwayat_pendidikan_dosen (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_dosen uuid,
    nidn character varying(255),
    nama_dosen character varying(255),
    id_bidang_studi character varying(255),
    nama_bidang_studi character varying(255),
    id_jenjang_pendidikan character varying(255),
    nama_jenjang_pendidikan character varying(255),
    id_gelar_akademik character varying(255),
    nama_gelar_akademik character varying(255),
    id_perguruan_tinggi uuid,
    nama_perguruan_tinggi character varying(255),
    fakultas character varying(255),
    tahun_lulus character varying(255),
    sks_lulus character varying(255),
    ipk character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.riwayat_pendidikan_dosen OWNER TO bendo01;

--
-- Name: riwayat_penelitian_dosen; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.riwayat_penelitian_dosen (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_dosen uuid,
    nidn character varying(255),
    nama_dosen character varying(255),
    id_penelitian uuid,
    judul_penelitian text,
    id_kelompok_bidang uuid,
    kode_kelompok_bidang character varying(255),
    nama_kelompok_bidang character varying(255),
    id_lembaga_iptek uuid,
    nama_lembaga_iptek character varying(255),
    tahun_kegiatan character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.riwayat_penelitian_dosen OWNER TO bendo01;

--
-- Name: riwayat_sertifikasi_dosen; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.riwayat_sertifikasi_dosen (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_dosen uuid,
    nidn character varying(255),
    nama_dosen character varying(255),
    nomor_peserta character varying(255),
    id_bidang_studi character varying(255),
    nama_bidang_studi character varying(255),
    id_jenis_sertifikasi character varying(255),
    nama_jenis_sertifikasi character varying(255),
    tahun_sertifikasi character varying(255),
    sk_sertifikasi character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.riwayat_sertifikasi_dosen OWNER TO bendo01;

--
-- Name: skala_nilai_program_studi; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.skala_nilai_program_studi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    tgl_create date,
    id_bobot_nilai uuid,
    id_prodi uuid,
    nama_program_studi character varying(255),
    nilai_huruf character varying(255),
    nilai_indeks character varying(255),
    bobot_minimum character varying(255),
    bobot_maksimum character varying(255),
    tanggal_mulai_efektif date,
    tanggal_akhir_efektif date,
    status_sync character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.skala_nilai_program_studi OWNER TO bendo01;

--
-- Name: substansi_matakuliah; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.substansi_matakuliah (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_substansi uuid,
    id_prodi uuid,
    nama_program_studi character varying(255),
    nama_substansi character varying(255),
    sks_mata_kuliah integer,
    sks_tatap_muka integer,
    sks_praktek integer,
    sks_praktek_lapangan integer,
    sks_simulasi integer,
    id_jenis_substansi uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.substansi_matakuliah OWNER TO bendo01;

--
-- Name: transkrip_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.transkrip_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_registrasi_mahasiswa uuid,
    id_matkul uuid,
    id_kelas_kuliah uuid,
    id_nilai_transfer character varying(255),
    id_konversi_aktivitas character varying(255),
    smt_diambil character varying(255),
    kode_mata_kuliah character varying(255),
    nama_mata_kuliah character varying(255),
    sks_mata_kuliah character varying(255),
    nilai_angka character varying(255),
    nilai_huruf character varying(255),
    nilai_indeks character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.transkrip_mahasiswa OWNER TO bendo01;

--
-- Name: uji_mahasiswa; Type: TABLE; Schema: feeder_master; Owner: bendo01
--

CREATE TABLE feeder_master.uji_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_aktivitas uuid,
    judul character varying(255),
    id_uji uuid,
    id_kategori_kegiatan uuid,
    nama_kategori_kegiatan character varying(255),
    id_dosen uuid,
    nidn character varying(255),
    nama_dosen character varying(255),
    penguji_ke integer,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_master.uji_mahasiswa OWNER TO bendo01;

--
-- Name: agama; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.agama (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_agama integer DEFAULT 0,
    nama_agama character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.agama OWNER TO bendo01;

--
-- Name: alat_transportasi; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.alat_transportasi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_alat_transportasi character varying(255),
    nama_alat_transportasi character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.alat_transportasi OWNER TO bendo01;

--
-- Name: bentuk_pendidikan; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.bentuk_pendidikan (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_bentuk_pendidikan character varying(255),
    nama_bentuk_pendidikan character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.bentuk_pendidikan OWNER TO bendo01;

--
-- Name: ikatan_kerja_sumber_daya_manusia; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.ikatan_kerja_sumber_daya_manusia (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_ikatan_kerja character varying(255),
    nama_ikatan_kerja character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.ikatan_kerja_sumber_daya_manusia OWNER TO bendo01;

--
-- Name: jabatan_fungsional; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jabatan_fungsional (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jabatan_fungsional character varying(255),
    nama_jabatan_fungsional character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jabatan_fungsional OWNER TO bendo01;

--
-- Name: jalur_masuk; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jalur_masuk (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jalur_masuk character varying(255),
    nama_jalur_masuk character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jalur_masuk OWNER TO bendo01;

--
-- Name: jenis_aktifitas_mahasiswa; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jenis_aktifitas_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jenis_aktivitas_mahasiswa character varying(255),
    nama_jenis_aktivitas_mahasiswa character varying(255),
    untuk_kampus_merdeka character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jenis_aktifitas_mahasiswa OWNER TO bendo01;

--
-- Name: jenis_evaluasi; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jenis_evaluasi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jenis_evaluasi character varying(255),
    nama_jenis_evaluasi character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jenis_evaluasi OWNER TO bendo01;

--
-- Name: jenis_keluar; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jenis_keluar (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jenis_keluar character varying(255),
    jenis_keluar character varying(255),
    apa_mahasiswa character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jenis_keluar OWNER TO bendo01;

--
-- Name: jenis_pendaftaran; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jenis_pendaftaran (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jenis_daftar character varying(255),
    nama_jenis_daftar character varying(255),
    untuk_daftar_sekolah character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jenis_pendaftaran OWNER TO bendo01;

--
-- Name: jenis_prestasi; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jenis_prestasi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jenis_prestasi character varying(255),
    nama_jenis_prestasi character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jenis_prestasi OWNER TO bendo01;

--
-- Name: jenis_satuan_manajemen_sumberdaya; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jenis_satuan_manajemen_sumberdaya (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jenis_sms character varying(255),
    nama_jenis_sms character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jenis_satuan_manajemen_sumberdaya OWNER TO bendo01;

--
-- Name: jenis_sertifikasi; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jenis_sertifikasi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jenis_sertifikasi character varying(255),
    nama_jenis_sertifikasi character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jenis_sertifikasi OWNER TO bendo01;

--
-- Name: jenis_substansi; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jenis_substansi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jenis_substansi character varying(255),
    nama_jenis_substansi character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jenis_substansi OWNER TO bendo01;

--
-- Name: jenis_tinggal; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jenis_tinggal (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jenis_tinggal character varying(255),
    nama_jenis_tinggal character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jenis_tinggal OWNER TO bendo01;

--
-- Name: jenjang_pendidikan; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.jenjang_pendidikan (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_jenjang_didik character varying(255),
    nama_jenjang_didik character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.jenjang_pendidikan OWNER TO bendo01;

--
-- Name: kategori_kegiatan; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.kategori_kegiatan (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_kategori_kegiatan character varying(255),
    nama_kategori_kegiatan text,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.kategori_kegiatan OWNER TO bendo01;

--
-- Name: kebutuhan_khusus; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.kebutuhan_khusus (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_kebutuhan_khusus character varying(255),
    nama_kebutuhan_khusus character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.kebutuhan_khusus OWNER TO bendo01;

--
-- Name: lembaga_pengangkat; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.lembaga_pengangkat (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_lembaga_angkat character varying(255),
    nama_lembaga_angkat character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.lembaga_pengangkat OWNER TO bendo01;

--
-- Name: level_wilayah; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.level_wilayah (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_level_wilayah character varying(255),
    nama_level_wilayah character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.level_wilayah OWNER TO bendo01;

--
-- Name: negara; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.negara (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_negara character varying(255),
    nama_negara character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.negara OWNER TO bendo01;

--
-- Name: pangkat_golongan; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.pangkat_golongan (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_pangkat_golongan character varying(255),
    kode_golongan character varying(255),
    nama_pangkat character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.pangkat_golongan OWNER TO bendo01;

--
-- Name: pekerjaan; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.pekerjaan (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_pekerjaan character varying(255),
    nama_pekerjaan character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.pekerjaan OWNER TO bendo01;

--
-- Name: pembiayaan; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.pembiayaan (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_pembiayaan character varying(255),
    nama_pembiayaan character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.pembiayaan OWNER TO bendo01;

--
-- Name: penghasilan; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.penghasilan (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_penghasilan character varying(255),
    nama_penghasilan character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.penghasilan OWNER TO bendo01;

--
-- Name: periode_lampau; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.periode_lampau (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_program_studi character varying(255),
    program_studi character varying(255),
    id_semester character varying(255),
    semester character varying(255),
    tanggal_mulai_perkuliahan date,
    tanggal_selesai_perkuliahan date,
    tipe_periode character varying(255),
    sync_at timestamp(0) without time zone,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.periode_lampau OWNER TO bendo01;

--
-- Name: semester; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.semester (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_semester character varying(255),
    id_tahun_ajaran character varying(255),
    nama_semester character varying(255),
    semester character varying(255),
    a_periode_aktif character varying(255),
    tanggal_mulai date,
    tanggal_selesai date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.semester OWNER TO bendo01;

--
-- Name: status_keaktifan_pegawai; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.status_keaktifan_pegawai (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_status_aktif character varying(255),
    nama_status_aktif character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.status_keaktifan_pegawai OWNER TO bendo01;

--
-- Name: status_kepegawaian; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.status_kepegawaian (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_status_pegawai character varying(255),
    nama_status_pegawai character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.status_kepegawaian OWNER TO bendo01;

--
-- Name: status_mahasiswa; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.status_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_status_mahasiswa character varying(255),
    nama_status_mahasiswa character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.status_mahasiswa OWNER TO bendo01;

--
-- Name: tahun_ajaran; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.tahun_ajaran (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_tahun_ajaran character varying(255),
    nama_tahun_ajaran character varying(255),
    a_periode_aktif character varying(255),
    tanggal_mulai date,
    tanggal_selesai date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.tahun_ajaran OWNER TO bendo01;

--
-- Name: tingkat_prestasi; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.tingkat_prestasi (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_tingkat_prestasi character varying(255),
    nama_tingkat_prestasi character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.tingkat_prestasi OWNER TO bendo01;

--
-- Name: wilayah; Type: TABLE; Schema: feeder_referensi; Owner: bendo01
--

CREATE TABLE feeder_referensi.wilayah (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_level_wilayah character varying(255),
    id_wilayah character varying(255),
    id_negara character varying(255),
    nama_wilayah character varying(255),
    id_induk_wilayah character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_referensi.wilayah OWNER TO bendo01;

--
-- Name: indeks_prestasi_sementara_mahasiswa; Type: TABLE; Schema: feeder_rekapitulasi; Owner: bendo01
--

CREATE TABLE feeder_rekapitulasi.indeks_prestasi_sementara_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_prodi uuid,
    aktif character varying(255),
    cuti character varying(255),
    non_aktif character varying(255),
    sedang_double_degree boolean,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_rekapitulasi.indeks_prestasi_sementara_mahasiswa OWNER TO bendo01;

--
-- Name: jumlah_dosen; Type: TABLE; Schema: feeder_rekapitulasi; Owner: bendo01
--

CREATE TABLE feeder_rekapitulasi.jumlah_dosen (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_prodi uuid,
    id_periode uuid,
    nama_periode character varying(255),
    nama_prodi character varying(255),
    jumlah_dosen_homebase integer,
    is_homebase boolean,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_rekapitulasi.jumlah_dosen OWNER TO bendo01;

--
-- Name: jumlah_mahasiswa; Type: TABLE; Schema: feeder_rekapitulasi; Owner: bendo01
--

CREATE TABLE feeder_rekapitulasi.jumlah_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_prodi uuid,
    aktif character varying(255),
    cuti character varying(255),
    non_aktif character varying(255),
    sedang_double_degree boolean,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_rekapitulasi.jumlah_mahasiswa OWNER TO bendo01;

--
-- Name: kartu_hasil_studi_mahasiswa; Type: TABLE; Schema: feeder_rekapitulasi; Owner: bendo01
--

CREATE TABLE feeder_rekapitulasi.kartu_hasil_studi_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_registrasi_mahasiswa uuid,
    nim integer,
    nama_mahasiswa character varying(255),
    id_periode uuid,
    nama_periode character varying(255),
    id_matkul uuid,
    nama_mata_kuliah character varying(255),
    sks_mata_kuliah integer,
    nilai_angka integer,
    nilai_huruf character varying(255),
    nilai_indeks integer,
    sks_x_indeks integer,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_rekapitulasi.kartu_hasil_studi_mahasiswa OWNER TO bendo01;

--
-- Name: kartu_rencana_studi_mahasiswa; Type: TABLE; Schema: feeder_rekapitulasi; Owner: bendo01
--

CREATE TABLE feeder_rekapitulasi.kartu_rencana_studi_mahasiswa (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_registrasi_mahasiswa uuid,
    nim integer,
    nama_mahasiswa character varying(255),
    id_matkul uuid,
    id_semester uuid,
    kode_mata_kuliah character varying(255),
    nama_mata_kuliah character varying(255),
    sks_mata_kuliah integer,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_rekapitulasi.kartu_rencana_studi_mahasiswa OWNER TO bendo01;

--
-- Name: laporan; Type: TABLE; Schema: feeder_rekapitulasi; Owner: bendo01
--

CREATE TABLE feeder_rekapitulasi.laporan (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    id_prodi uuid,
    nama_program_studi character varying(255),
    id_semester uuid,
    nama_semester character varying(255),
    jumlah_target_mahasiswa_baru integer,
    tanggal_awal_perkuliahan date,
    tanggal_akhir_perkuliahan date,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE feeder_rekapitulasi.laporan OWNER TO bendo01;

--
-- Name: employees; Type: TABLE; Schema: institution_master; Owner: bendo01
--

CREATE TABLE institution_master.employees (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    institution_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    decree_number character varying(255),
    decree_date date,
    is_active boolean DEFAULT false NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE institution_master.employees OWNER TO bendo01;

--
-- Name: institutions; Type: TABLE; Schema: institution_master; Owner: bendo01
--

CREATE TABLE institution_master.institutions (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code character varying(255),
    name character varying(255),
    alphabet_code character varying(255),
    is_active boolean DEFAULT false NOT NULL,
    variety_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    category_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    country_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    parent_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    academic_year_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE institution_master.institutions OWNER TO bendo01;

--
-- Name: staffes; Type: TABLE; Schema: institution_master; Owner: bendo01
--

CREATE TABLE institution_master.staffes (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code character varying(255),
    name character varying(255),
    decree_number character varying(255),
    decree_date date,
    start_date date,
    end_date date,
    employee_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    position_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE institution_master.staffes OWNER TO bendo01;

--
-- Name: units; Type: TABLE; Schema: institution_master; Owner: bendo01
--

CREATE TABLE institution_master.units (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code character varying(255),
    name character varying(255),
    is_active boolean DEFAULT false NOT NULL,
    unit_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    institution_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    parent_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    education_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    lft bigint DEFAULT 0,
    rght bigint DEFAULT 0,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE institution_master.units OWNER TO bendo01;

--
-- Name: categories; Type: TABLE; Schema: institution_reference; Owner: bendo01
--

CREATE TABLE institution_reference.categories (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    sync_at timestamp without time zone
);


ALTER TABLE institution_reference.categories OWNER TO bendo01;

--
-- Name: position_types; Type: TABLE; Schema: institution_reference; Owner: bendo01
--

CREATE TABLE institution_reference.position_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    sync_at timestamp without time zone
);


ALTER TABLE institution_reference.position_types OWNER TO bendo01;

--
-- Name: unit_types; Type: TABLE; Schema: institution_reference; Owner: bendo01
--

CREATE TABLE institution_reference.unit_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    sync_at timestamp without time zone
);


ALTER TABLE institution_reference.unit_types OWNER TO bendo01;

--
-- Name: varieties; Type: TABLE; Schema: institution_reference; Owner: bendo01
--

CREATE TABLE institution_reference.varieties (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    sync_at timestamp without time zone
);


ALTER TABLE institution_reference.varieties OWNER TO bendo01;

--
-- Name: categories; Type: TABLE; Schema: literate; Owner: bendo01
--

CREATE TABLE literate.categories (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE literate.categories OWNER TO bendo01;

--
-- Name: educations; Type: TABLE; Schema: literate; Owner: bendo01
--

CREATE TABLE literate.educations (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    abbreviation character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    level_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    group_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    category_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    variety_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE literate.educations OWNER TO bendo01;

--
-- Name: groups; Type: TABLE; Schema: literate; Owner: bendo01
--

CREATE TABLE literate.groups (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE literate.groups OWNER TO bendo01;

--
-- Name: levels; Type: TABLE; Schema: literate; Owner: bendo01
--

CREATE TABLE literate.levels (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE literate.levels OWNER TO bendo01;

--
-- Name: varieties; Type: TABLE; Schema: literate; Owner: bendo01
--

CREATE TABLE literate.varieties (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE literate.varieties OWNER TO bendo01;

--
-- Name: continents; Type: TABLE; Schema: location; Owner: bendo01
--

CREATE TABLE location.continents (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0,
    alphabet_code character varying(255) DEFAULT NULL::character varying NOT NULL,
    name character varying(255) DEFAULT NULL::character varying NOT NULL,
    slug character varying(255) DEFAULT NULL::character varying,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE location.continents OWNER TO bendo01;

--
-- Name: countries; Type: TABLE; Schema: location; Owner: bendo01
--

CREATE TABLE location.countries (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    alpha2_code character varying(255) NOT NULL,
    alpha3_code character varying(255) NOT NULL,
    iso3166_2_code character varying(255) NOT NULL,
    dikti_code character varying(255) DEFAULT NULL::character varying,
    continent_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    region_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    slug character varying(255),
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE location.countries OWNER TO bendo01;

--
-- Name: provinces; Type: TABLE; Schema: location; Owner: bendo01
--

CREATE TABLE location.provinces (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code character varying(255) DEFAULT NULL::character varying,
    name character varying(255),
    dikti_code character varying(255) DEFAULT NULL::character varying,
    epsbed_code character varying(255) DEFAULT NULL::character varying,
    slug character varying(255) DEFAULT NULL::character varying,
    description text,
    alt_slug character varying(255) DEFAULT NULL::character varying,
    state_ministry_code character varying(255) DEFAULT NULL::character varying,
    state_ministry_full_code character varying(255) DEFAULT NULL::character varying,
    state_post_department_code character varying(255) DEFAULT NULL::character varying,
    state_ministry_name character varying(255) DEFAULT NULL::character varying,
    dikti_name character varying(255) DEFAULT NULL::character varying,
    validation_code character varying(255) DEFAULT NULL::character varying,
    latitude double precision DEFAULT 0,
    longitude double precision DEFAULT 0,
    zoom integer DEFAULT 0,
    country_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE location.provinces OWNER TO bendo01;

--
-- Name: regencies; Type: TABLE; Schema: location; Owner: bendo01
--

CREATE TABLE location.regencies (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code character varying(255) DEFAULT NULL::character varying,
    name character varying(255),
    dikti_code character varying(255) DEFAULT NULL::character varying,
    epsbed_code character varying(255) DEFAULT NULL::character varying,
    province_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    regency_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    slug character varying(255) DEFAULT NULL::character varying,
    description text,
    alt_slug character varying(255) DEFAULT NULL::character varying,
    state_ministry_code character varying(255) DEFAULT NULL::character varying,
    state_ministry_full_code character varying(255) DEFAULT NULL::character varying,
    state_post_department_code character varying(255) DEFAULT NULL::character varying,
    state_ministry_name character varying(255) DEFAULT NULL::character varying,
    dikti_name character varying(255) DEFAULT NULL::character varying,
    validation_code character varying(255) DEFAULT NULL::character varying,
    latitude double precision DEFAULT 0,
    longitude double precision DEFAULT 0,
    zoom integer DEFAULT 0,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE location.regencies OWNER TO bendo01;

--
-- Name: regency_types; Type: TABLE; Schema: location; Owner: bendo01
--

CREATE TABLE location.regency_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE location.regency_types OWNER TO bendo01;

--
-- Name: regions; Type: TABLE; Schema: location; Owner: bendo01
--

CREATE TABLE location.regions (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) DEFAULT NULL::character varying,
    alt_slug character varying(255) DEFAULT NULL::character varying,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE location.regions OWNER TO bendo01;

--
-- Name: sub_districts; Type: TABLE; Schema: location; Owner: bendo01
--

CREATE TABLE location.sub_districts (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    dikti_code character varying(255) DEFAULT NULL::character varying,
    regency_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    slug character varying(255) DEFAULT NULL::character varying,
    alt_slug character varying(255) DEFAULT NULL::character varying,
    state_ministry_code character varying(255) DEFAULT NULL::character varying,
    state_ministry_full_code character varying(255) DEFAULT NULL::character varying,
    state_post_department_code character varying(255) DEFAULT NULL::character varying,
    state_ministry_name character varying(255) DEFAULT NULL::character varying,
    dikti_name character varying(255) DEFAULT NULL::character varying,
    validation_code character varying(255) DEFAULT NULL::character varying,
    agriculture_department_name character varying(255) DEFAULT NULL::character varying,
    latitude double precision DEFAULT 0,
    longitude double precision DEFAULT 0,
    zoom integer DEFAULT 0,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE location.sub_districts OWNER TO bendo01;

--
-- Name: villages; Type: TABLE; Schema: location; Owner: bendo01
--

CREATE TABLE location.villages (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code character varying(255) DEFAULT NULL::character varying NOT NULL,
    name character varying(255) DEFAULT NULL::character varying NOT NULL,
    sub_district_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    slug character varying(255) DEFAULT NULL::character varying,
    alt_slug character varying(255) DEFAULT NULL::character varying,
    state_ministry_code character varying(255) DEFAULT NULL::character varying,
    state_post_department_code character varying(255) DEFAULT NULL::character varying,
    state_ministry_name character varying(255) DEFAULT NULL::character varying,
    dikti_name character varying(255) DEFAULT NULL::character varying,
    dikti_code character varying(255),
    latitude double precision DEFAULT 0,
    longitude double precision DEFAULT 0,
    zoom integer DEFAULT 0,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE location.villages OWNER TO bendo01;

--
-- Name: accounts; Type: TABLE; Schema: payment_midtrans; Owner: bendo01
--

CREATE TABLE payment_midtrans.accounts (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    institution_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    merchant_identification character varying(255) NOT NULL,
    client_key character varying(255) NOT NULL,
    server_key character varying(255) NOT NULL,
    sandbox_url character varying(255) NOT NULL,
    production_url character varying(255) NOT NULL,
    is_production boolean DEFAULT false NOT NULL,
    is_sanitized boolean DEFAULT true NOT NULL,
    is_3ds boolean DEFAULT true NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE payment_midtrans.accounts OWNER TO bendo01;

--
-- Name: billing_addresses; Type: TABLE; Schema: payment_midtrans; Owner: bendo01
--

CREATE TABLE payment_midtrans.billing_addresses (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    first_name character varying(255) NOT NULL,
    last_name character varying(255),
    email character varying(255) NOT NULL,
    phone character varying(255) NOT NULL,
    customer_detail_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE payment_midtrans.billing_addresses OWNER TO bendo01;

--
-- Name: customer_details; Type: TABLE; Schema: payment_midtrans; Owner: bendo01
--

CREATE TABLE payment_midtrans.customer_details (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    first_name character varying(255) NOT NULL,
    last_name character varying(255),
    email character varying(255),
    phone character varying(255),
    transaction_detail_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE payment_midtrans.customer_details OWNER TO bendo01;

--
-- Name: item_details; Type: TABLE; Schema: payment_midtrans; Owner: bendo01
--

CREATE TABLE payment_midtrans.item_details (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    name character varying(255) NOT NULL,
    quantity integer NOT NULL,
    transaction_detail_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    price double precision DEFAULT 0
);


ALTER TABLE payment_midtrans.item_details OWNER TO bendo01;

--
-- Name: shipping_addresses; Type: TABLE; Schema: payment_midtrans; Owner: bendo01
--

CREATE TABLE payment_midtrans.shipping_addresses (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    first_name character varying(255) NOT NULL,
    last_name character varying(255),
    email character varying(255) NOT NULL,
    phone character varying(255) NOT NULL,
    address character varying(255) NOT NULL,
    city character varying(255) NOT NULL,
    postal_code character varying(255) NOT NULL,
    country_code character varying(255) NOT NULL,
    customer_detail_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE payment_midtrans.shipping_addresses OWNER TO bendo01;

--
-- Name: transaction_details; Type: TABLE; Schema: payment_midtrans; Owner: bendo01
--

CREATE TABLE payment_midtrans.transaction_details (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    order_id character varying(255) NOT NULL,
    account_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    transaction_detailable_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    transaction_detailable_type character varying(255) NOT NULL,
    is_paid boolean DEFAULT false,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    gross_amount double precision DEFAULT 0
);


ALTER TABLE payment_midtrans.transaction_details OWNER TO bendo01;

--
-- Name: biodatas; Type: TABLE; Schema: person_master; Owner: bendo01
--

CREATE TABLE person_master.biodatas (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    height double precision DEFAULT '0'::double precision,
    weight double precision DEFAULT '0'::double precision,
    is_positive_blood_rhesus boolean DEFAULT false NOT NULL,
    blood_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    hair_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    hair_color_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    eye_color_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    bust double precision DEFAULT '0'::double precision NOT NULL,
    waist double precision DEFAULT '0'::double precision NOT NULL,
    hip double precision DEFAULT '0'::double precision NOT NULL,
    arm_circumference double precision DEFAULT '0'::double precision NOT NULL,
    menarche_age integer DEFAULT 0 NOT NULL,
    menopause_age integer DEFAULT 0 NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    sync_at timestamp without time zone
);


ALTER TABLE person_master.biodatas OWNER TO bendo01;

--
-- Name: family_card_members; Type: TABLE; Schema: person_master; Owner: bendo01
--

CREATE TABLE person_master.family_card_members (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    family_card_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    relative_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    relative_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    sync_at timestamp without time zone
);


ALTER TABLE person_master.family_card_members OWNER TO bendo01;

--
-- Name: family_cards; Type: TABLE; Schema: person_master; Owner: bendo01
--

CREATE TABLE person_master.family_cards (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code character varying(255),
    individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    sync_at timestamp without time zone
);


ALTER TABLE person_master.family_cards OWNER TO bendo01;

--
-- Name: images; Type: TABLE; Schema: person_master; Owner: bendo01
--

CREATE TABLE person_master.images (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    filename character varying(255) NOT NULL,
    dir character varying(255) NOT NULL,
    mimetype character varying(255),
    size bigint,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_master.images OWNER TO bendo01;

--
-- Name: individuals; Type: TABLE; Schema: person_master; Owner: bendo01
--

CREATE TABLE person_master.individuals (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    front_title character varying(255),
    last_title character varying(255),
    birth_date date NOT NULL,
    birth_place character varying(255) NOT NULL,
    gender_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    religion_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    occupation_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    education_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    income_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    identification_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    marital_status_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    profession_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid NOT NULL,
    age_classification_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    is_special_need boolean DEFAULT false NOT NULL,
    is_social_protection_card_recipient boolean DEFAULT false NOT NULL,
    is_deceased boolean DEFAULT false,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    sync_at timestamp without time zone
);


ALTER TABLE person_master.individuals OWNER TO bendo01;

--
-- Name: age_classifications; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.age_classifications (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    minimum integer DEFAULT 0 NOT NULL,
    maximum integer DEFAULT 0,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.age_classifications OWNER TO bendo01;

--
-- Name: blood_types; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.blood_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.blood_types OWNER TO bendo01;

--
-- Name: eye_colors; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.eye_colors (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.eye_colors OWNER TO bendo01;

--
-- Name: genders; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.genders (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) without time zone,
    sync_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.genders OWNER TO bendo01;

--
-- Name: hair_colors; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.hair_colors (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    deleted_at timestamp(0) with time zone,
    sync_at timestamp without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.hair_colors OWNER TO bendo01;

--
-- Name: hair_types; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.hair_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.hair_types OWNER TO bendo01;

--
-- Name: identification_types; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.identification_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.identification_types OWNER TO bendo01;

--
-- Name: incomes; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.incomes (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    minimum double precision DEFAULT 0,
    maximum double precision DEFAULT 0,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.incomes OWNER TO bendo01;

--
-- Name: marital_statuses; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.marital_statuses (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.marital_statuses OWNER TO bendo01;

--
-- Name: occupations; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.occupations (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.occupations OWNER TO bendo01;

--
-- Name: professions; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.professions (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.professions OWNER TO bendo01;

--
-- Name: relative_types; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.relative_types (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.relative_types OWNER TO bendo01;

--
-- Name: religions; Type: TABLE; Schema: person_reference; Owner: bendo01
--

CREATE TABLE person_reference.religions (
    id uuid DEFAULT public.uuid_generate_v7() NOT NULL,
    code integer DEFAULT 0 NOT NULL,
    alphabet_code character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone DEFAULT now(),
    updated_at timestamp(0) without time zone DEFAULT now(),
    sync_at timestamp without time zone,
    deleted_at timestamp(0) without time zone,
    created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
    updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid
);


ALTER TABLE person_reference.religions OWNER TO bendo01;

--
-- Name: audits; Type: TABLE; Schema: public; Owner: bendo01
--

CREATE TABLE public.audits (
    id bigint NOT NULL,
    user_type character varying(255),
    user_id uuid,
    event character varying(255) NOT NULL,
    auditable_type character varying(255) NOT NULL,
    auditable_id uuid NOT NULL,
    old_values text,
    new_values text,
    url text,
    ip_address inet,
    user_agent character varying(1023),
    tags character varying(255),
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.audits OWNER TO bendo01;

--
-- Name: audits_id_seq; Type: SEQUENCE; Schema: public; Owner: bendo01
--

CREATE SEQUENCE public.audits_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.audits_id_seq OWNER TO bendo01;

--
-- Name: audits_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bendo01
--

ALTER SEQUENCE public.audits_id_seq OWNED BY public.audits.id;


--
-- Name: cache; Type: TABLE; Schema: public; Owner: bendo01
--

CREATE TABLE public.cache (
    key character varying(255) NOT NULL,
    value text NOT NULL,
    expiration integer NOT NULL
);


ALTER TABLE public.cache OWNER TO bendo01;

--
-- Name: cache_locks; Type: TABLE; Schema: public; Owner: bendo01
--

CREATE TABLE public.cache_locks (
    key character varying(255) NOT NULL,
    owner character varying(255) NOT NULL,
    expiration integer NOT NULL
);


ALTER TABLE public.cache_locks OWNER TO bendo01;

--
-- Name: failed_jobs; Type: TABLE; Schema: public; Owner: bendo01
--

CREATE TABLE public.failed_jobs (
    id bigint NOT NULL,
    uuid character varying(255) NOT NULL,
    connection text NOT NULL,
    queue text NOT NULL,
    payload text NOT NULL,
    exception text NOT NULL,
    failed_at timestamp(0) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.failed_jobs OWNER TO bendo01;

--
-- Name: failed_jobs_id_seq; Type: SEQUENCE; Schema: public; Owner: bendo01
--

CREATE SEQUENCE public.failed_jobs_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.failed_jobs_id_seq OWNER TO bendo01;

--
-- Name: failed_jobs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bendo01
--

ALTER SEQUENCE public.failed_jobs_id_seq OWNED BY public.failed_jobs.id;


--
-- Name: job_batches; Type: TABLE; Schema: public; Owner: bendo01
--

CREATE TABLE public.job_batches (
    id character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    total_jobs integer NOT NULL,
    pending_jobs integer NOT NULL,
    failed_jobs integer NOT NULL,
    failed_job_ids text NOT NULL,
    options text,
    cancelled_at integer,
    created_at integer NOT NULL,
    finished_at integer
);


ALTER TABLE public.job_batches OWNER TO bendo01;

--
-- Name: jobs; Type: TABLE; Schema: public; Owner: bendo01
--

CREATE TABLE public.jobs (
    id bigint NOT NULL,
    queue character varying(255) NOT NULL,
    payload text NOT NULL,
    attempts smallint NOT NULL,
    reserved_at integer,
    available_at integer NOT NULL,
    created_at integer NOT NULL
);


ALTER TABLE public.jobs OWNER TO bendo01;

--
-- Name: jobs_id_seq; Type: SEQUENCE; Schema: public; Owner: bendo01
--

CREATE SEQUENCE public.jobs_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.jobs_id_seq OWNER TO bendo01;

--
-- Name: jobs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bendo01
--

ALTER SEQUENCE public.jobs_id_seq OWNED BY public.jobs.id;


--
-- Name: notifications; Type: TABLE; Schema: public; Owner: bendo01
--

CREATE TABLE public.notifications (
    id uuid NOT NULL,
    type character varying(255) NOT NULL,
    notifiable_type character varying(255) NOT NULL,
    notifiable_id bigint NOT NULL,
    data text NOT NULL,
    read_at timestamp(0) without time zone,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.notifications OWNER TO bendo01;

--
-- Name: pg_loco_queue; Type: TABLE; Schema: public; Owner: bendo01
--

CREATE TABLE public.pg_loco_queue (
    id character varying NOT NULL,
    name character varying NOT NULL,
    task_data jsonb NOT NULL,
    status character varying DEFAULT 'queued'::character varying NOT NULL,
    run_at timestamp with time zone NOT NULL,
    "interval" bigint,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    tags character varying(255)
);


ALTER TABLE public.pg_loco_queue OWNER TO bendo01;

--
-- Name: seaql_migrations; Type: TABLE; Schema: public; Owner: bendo01
--

CREATE TABLE public.seaql_migrations (
    version character varying NOT NULL,
    applied_at bigint NOT NULL
);


ALTER TABLE public.seaql_migrations OWNER TO bendo01;

--
-- Name: audits id; Type: DEFAULT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.audits ALTER COLUMN id SET DEFAULT nextval('public.audits_id_seq'::regclass);


--
-- Name: failed_jobs id; Type: DEFAULT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.failed_jobs ALTER COLUMN id SET DEFAULT nextval('public.failed_jobs_id_seq'::regclass);


--
-- Name: jobs id; Type: DEFAULT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.jobs ALTER COLUMN id SET DEFAULT nextval('public.jobs_id_seq'::regclass);


--
-- Name: attend_types acr_attend_types_pkey; Type: CONSTRAINT; Schema: academic_campaign_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_reference.attend_types
    ADD CONSTRAINT acr_attend_types_pkey PRIMARY KEY (id);


--
-- Name: calendar_categories acr_calendar_categories_pkey; Type: CONSTRAINT; Schema: academic_campaign_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_reference.calendar_categories
    ADD CONSTRAINT acr_calendar_categories_pkey PRIMARY KEY (id);


--
-- Name: encounter_categories acr_encounter_categories_pkey; Type: CONSTRAINT; Schema: academic_campaign_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_reference.encounter_categories
    ADD CONSTRAINT acr_encounter_categories_pkey PRIMARY KEY (id);


--
-- Name: encounter_types acr_encounter_types_pkey; Type: CONSTRAINT; Schema: academic_campaign_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_reference.encounter_types
    ADD CONSTRAINT acr_encounter_types_pkey PRIMARY KEY (id);


--
-- Name: evaluation_types acr_evaluation_types_pkey; Type: CONSTRAINT; Schema: academic_campaign_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_reference.evaluation_types
    ADD CONSTRAINT acr_evaluation_types_pkey PRIMARY KEY (id);


--
-- Name: implementations acr_implementations_pkey; Type: CONSTRAINT; Schema: academic_campaign_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_reference.implementations
    ADD CONSTRAINT acr_implementations_pkey PRIMARY KEY (id);


--
-- Name: scopes aucr_scopes_pkey; Type: CONSTRAINT; Schema: academic_campaign_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_reference.scopes
    ADD CONSTRAINT aucr_scopes_pkey PRIMARY KEY (id);


--
-- Name: substances aucr_substances_pkey; Type: CONSTRAINT; Schema: academic_campaign_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_reference.substances
    ADD CONSTRAINT aucr_substances_pkey PRIMARY KEY (id);


--
-- Name: activities act_activities_pkey; Type: CONSTRAINT; Schema: academic_campaign_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_transaction.activities
    ADD CONSTRAINT act_activities_pkey PRIMARY KEY (id);


--
-- Name: calendar_details act_calendar_details_pkey; Type: CONSTRAINT; Schema: academic_campaign_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_transaction.calendar_details
    ADD CONSTRAINT act_calendar_details_pkey PRIMARY KEY (id);


--
-- Name: calendars act_calendars_pkey; Type: CONSTRAINT; Schema: academic_campaign_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_transaction.calendars
    ADD CONSTRAINT act_calendars_pkey PRIMARY KEY (id);


--
-- Name: class_codes act_class_codes_pkey; Type: CONSTRAINT; Schema: academic_campaign_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_transaction.class_codes
    ADD CONSTRAINT act_class_codes_pkey PRIMARY KEY (id);


--
-- Name: grades act_grades_pkey; Type: CONSTRAINT; Schema: academic_campaign_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_transaction.grades
    ADD CONSTRAINT act_grades_pkey PRIMARY KEY (id);


--
-- Name: schedules act_schedules_pkey; Type: CONSTRAINT; Schema: academic_campaign_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_transaction.schedules
    ADD CONSTRAINT act_schedules_pkey PRIMARY KEY (id);


--
-- Name: teach_decrees act_teach_decrees_pkey; Type: CONSTRAINT; Schema: academic_campaign_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_transaction.teach_decrees
    ADD CONSTRAINT act_teach_decrees_pkey PRIMARY KEY (id);


--
-- Name: teach_lecturers act_teach_lecturers_pkey; Type: CONSTRAINT; Schema: academic_campaign_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_transaction.teach_lecturers
    ADD CONSTRAINT act_teach_lecturers_pkey PRIMARY KEY (id);


--
-- Name: teaches act_teaches_pkey; Type: CONSTRAINT; Schema: academic_campaign_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_campaign_transaction.teaches
    ADD CONSTRAINT act_teaches_pkey PRIMARY KEY (id);


--
-- Name: candidates acm_candidates_pkey; Type: CONSTRAINT; Schema: academic_candidate_master; Owner: bendo01
--

ALTER TABLE ONLY academic_candidate_master.candidates
    ADD CONSTRAINT acm_candidates_pkey PRIMARY KEY (id);


--
-- Name: exam_classes acm_exam_classes_pkey; Type: CONSTRAINT; Schema: academic_candidate_master; Owner: bendo01
--

ALTER TABLE ONLY academic_candidate_master.exam_classes
    ADD CONSTRAINT acm_exam_classes_pkey PRIMARY KEY (id);


--
-- Name: document_types acr_document_types_pkey; Type: CONSTRAINT; Schema: academic_candidate_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_candidate_reference.document_types
    ADD CONSTRAINT acr_document_types_pkey PRIMARY KEY (id);


--
-- Name: phases acr_phases_pkey; Type: CONSTRAINT; Schema: academic_candidate_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_candidate_reference.phases
    ADD CONSTRAINT acr_phases_pkey PRIMARY KEY (id);


--
-- Name: registration_categories acr_registration_categories_pkey; Type: CONSTRAINT; Schema: academic_candidate_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_candidate_reference.registration_categories
    ADD CONSTRAINT acr_registration_categories_pkey PRIMARY KEY (id);


--
-- Name: registration_types registration_types_pkey; Type: CONSTRAINT; Schema: academic_candidate_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_candidate_reference.registration_types
    ADD CONSTRAINT registration_types_pkey PRIMARY KEY (id);


--
-- Name: candidate_unit_choices act_candidate_unit_choices_pkey; Type: CONSTRAINT; Schema: academic_candidate_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_candidate_transaction.candidate_unit_choices
    ADD CONSTRAINT act_candidate_unit_choices_pkey PRIMARY KEY (id);


--
-- Name: documents act_documents_pkey; Type: CONSTRAINT; Schema: academic_candidate_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_candidate_transaction.documents
    ADD CONSTRAINT act_documents_pkey PRIMARY KEY (id);


--
-- Name: exams act_exams_pkey; Type: CONSTRAINT; Schema: academic_candidate_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_candidate_transaction.exams
    ADD CONSTRAINT act_exams_pkey PRIMARY KEY (id);


--
-- Name: curriculum_details academic_course_master_curriculum_details_pkey; Type: CONSTRAINT; Schema: academic_course_master; Owner: bendo01
--

ALTER TABLE ONLY academic_course_master.curriculum_details
    ADD CONSTRAINT academic_course_master_curriculum_details_pkey PRIMARY KEY (id);


--
-- Name: concentrations acm_concentrations_pkey; Type: CONSTRAINT; Schema: academic_course_master; Owner: bendo01
--

ALTER TABLE ONLY academic_course_master.concentrations
    ADD CONSTRAINT acm_concentrations_pkey PRIMARY KEY (id);


--
-- Name: course_evaluation_plannings acm_course_evaluation_plannings_pkey; Type: CONSTRAINT; Schema: academic_course_master; Owner: bendo01
--

ALTER TABLE ONLY academic_course_master.course_evaluation_plannings
    ADD CONSTRAINT acm_course_evaluation_plannings_pkey PRIMARY KEY (id);


--
-- Name: course_learn_plannings acm_course_learn_plannings_pkey; Type: CONSTRAINT; Schema: academic_course_master; Owner: bendo01
--

ALTER TABLE ONLY academic_course_master.course_learn_plannings
    ADD CONSTRAINT acm_course_learn_plannings_pkey PRIMARY KEY (id);


--
-- Name: courses acm_courses_pkey; Type: CONSTRAINT; Schema: academic_course_master; Owner: bendo01
--

ALTER TABLE ONLY academic_course_master.courses
    ADD CONSTRAINT acm_courses_pkey PRIMARY KEY (id);


--
-- Name: curriculums acm_curriculums_pkey; Type: CONSTRAINT; Schema: academic_course_master; Owner: bendo01
--

ALTER TABLE ONLY academic_course_master.curriculums
    ADD CONSTRAINT acm_curriculums_pkey PRIMARY KEY (id);


--
-- Name: competences acr_competences_pkey; Type: CONSTRAINT; Schema: academic_course_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_course_reference.competences
    ADD CONSTRAINT acr_competences_pkey PRIMARY KEY (id);


--
-- Name: course_evaluation_bases acr_course_evaluation_bases_pkey; Type: CONSTRAINT; Schema: academic_course_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_course_reference.course_evaluation_bases
    ADD CONSTRAINT acr_course_evaluation_bases_pkey PRIMARY KEY (id);


--
-- Name: curriculum_types acr_curriculum_types_pkey; Type: CONSTRAINT; Schema: academic_course_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_course_reference.curriculum_types
    ADD CONSTRAINT acr_curriculum_types_pkey PRIMARY KEY (id);


--
-- Name: groups acr_groups_pkey; Type: CONSTRAINT; Schema: academic_course_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_course_reference.groups
    ADD CONSTRAINT acr_groups_pkey PRIMARY KEY (id);


--
-- Name: semesters acr_semesters_pkey; Type: CONSTRAINT; Schema: academic_course_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_course_reference.semesters
    ADD CONSTRAINT acr_semesters_pkey PRIMARY KEY (id);


--
-- Name: varieties acr_varieties_pkey; Type: CONSTRAINT; Schema: academic_course_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_course_reference.varieties
    ADD CONSTRAINT acr_varieties_pkey PRIMARY KEY (id);


--
-- Name: academic_year_categories agr_academic_year_categories_pkey; Type: CONSTRAINT; Schema: academic_general_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_general_reference.academic_year_categories
    ADD CONSTRAINT agr_academic_year_categories_pkey PRIMARY KEY (id);


--
-- Name: academic_years agr_academic_years_pkey; Type: CONSTRAINT; Schema: academic_general_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_general_reference.academic_years
    ADD CONSTRAINT agr_academic_years_pkey PRIMARY KEY (id);


--
-- Name: lecturers alm_lecturers_pkey; Type: CONSTRAINT; Schema: academic_lecturer_master; Owner: bendo01
--

ALTER TABLE ONLY academic_lecturer_master.lecturers
    ADD CONSTRAINT alm_lecturers_pkey PRIMARY KEY (id);


--
-- Name: contracts alr_contracts_pkey; Type: CONSTRAINT; Schema: academic_lecturer_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_lecturer_reference.contracts
    ADD CONSTRAINT alr_contracts_pkey PRIMARY KEY (id);


--
-- Name: groups alr_groups_pkey; Type: CONSTRAINT; Schema: academic_lecturer_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_lecturer_reference.groups
    ADD CONSTRAINT alr_groups_pkey PRIMARY KEY (id);


--
-- Name: ranks alr_ranks_pkey; Type: CONSTRAINT; Schema: academic_lecturer_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_lecturer_reference.ranks
    ADD CONSTRAINT alr_ranks_pkey PRIMARY KEY (id);


--
-- Name: statuses alr_statuses_pkey; Type: CONSTRAINT; Schema: academic_lecturer_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_lecturer_reference.statuses
    ADD CONSTRAINT alr_statuses_pkey PRIMARY KEY (id);


--
-- Name: academic_groups alt_academic_groups_pkey; Type: CONSTRAINT; Schema: academic_lecturer_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_lecturer_transaction.academic_groups
    ADD CONSTRAINT alt_academic_groups_pkey PRIMARY KEY (id);


--
-- Name: academic_ranks alt_academic_ranks_pkey; Type: CONSTRAINT; Schema: academic_lecturer_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_lecturer_transaction.academic_ranks
    ADD CONSTRAINT alt_academic_ranks_pkey PRIMARY KEY (id);


--
-- Name: homebases alt_homebases_pkey; Type: CONSTRAINT; Schema: academic_lecturer_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_lecturer_transaction.homebases
    ADD CONSTRAINT alt_homebases_pkey PRIMARY KEY (id);


--
-- Name: evaluator_types academic_prior_learning_recognition_reference_evaluator_types_p; Type: CONSTRAINT; Schema: academic_prior_learning_recognition_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_prior_learning_recognition_reference.evaluator_types
    ADD CONSTRAINT academic_prior_learning_recognition_reference_evaluator_types_p PRIMARY KEY (id);


--
-- Name: evidence_categories academic_prior_learning_recognition_reference_evidence_categori; Type: CONSTRAINT; Schema: academic_prior_learning_recognition_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_prior_learning_recognition_reference.evidence_categories
    ADD CONSTRAINT academic_prior_learning_recognition_reference_evidence_categori PRIMARY KEY (id);


--
-- Name: evidence_types academic_prior_learning_recognition_reference_evidence_types_pk; Type: CONSTRAINT; Schema: academic_prior_learning_recognition_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_prior_learning_recognition_reference.evidence_types
    ADD CONSTRAINT academic_prior_learning_recognition_reference_evidence_types_pk PRIMARY KEY (id);


--
-- Name: professionalisms academic_prior_learning_recognition_reference_professionalisms_; Type: CONSTRAINT; Schema: academic_prior_learning_recognition_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_prior_learning_recognition_reference.professionalisms
    ADD CONSTRAINT academic_prior_learning_recognition_reference_professionalisms_ PRIMARY KEY (id);


--
-- Name: decrees academic_prior_learning_recognition_transaction_decrees_pkey; Type: CONSTRAINT; Schema: academic_prior_learning_recognition_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_prior_learning_recognition_transaction.decrees
    ADD CONSTRAINT academic_prior_learning_recognition_transaction_decrees_pkey PRIMARY KEY (id);


--
-- Name: evaluation_details academic_prior_learning_recognition_transaction_evaluation_deta; Type: CONSTRAINT; Schema: academic_prior_learning_recognition_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_prior_learning_recognition_transaction.evaluation_details
    ADD CONSTRAINT academic_prior_learning_recognition_transaction_evaluation_deta PRIMARY KEY (id);


--
-- Name: evaluations academic_prior_learning_recognition_transaction_evaluations_pke; Type: CONSTRAINT; Schema: academic_prior_learning_recognition_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_prior_learning_recognition_transaction.evaluations
    ADD CONSTRAINT academic_prior_learning_recognition_transaction_evaluations_pke PRIMARY KEY (id);


--
-- Name: evaluators academic_prior_learning_recognition_transaction_evaluators_pkey; Type: CONSTRAINT; Schema: academic_prior_learning_recognition_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_prior_learning_recognition_transaction.evaluators
    ADD CONSTRAINT academic_prior_learning_recognition_transaction_evaluators_pkey PRIMARY KEY (id);


--
-- Name: recognitions academic_prior_learning_recognition_transaction_recognitions_pk; Type: CONSTRAINT; Schema: academic_prior_learning_recognition_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_prior_learning_recognition_transaction.recognitions
    ADD CONSTRAINT academic_prior_learning_recognition_transaction_recognitions_pk PRIMARY KEY (id);


--
-- Name: counsellors academic_student_adviser_counsellors_pkey; Type: CONSTRAINT; Schema: academic_student_adviser; Owner: bendo01
--

ALTER TABLE ONLY academic_student_adviser.counsellors
    ADD CONSTRAINT academic_student_adviser_counsellors_pkey PRIMARY KEY (id);


--
-- Name: decrees academic_student_adviser_decrees_pkey; Type: CONSTRAINT; Schema: academic_student_adviser; Owner: bendo01
--

ALTER TABLE ONLY academic_student_adviser.decrees
    ADD CONSTRAINT academic_student_adviser_decrees_pkey PRIMARY KEY (id);


--
-- Name: student_activities academic_student_campaign_activities_pkey; Type: CONSTRAINT; Schema: academic_student_campaign; Owner: bendo01
--

ALTER TABLE ONLY academic_student_campaign.student_activities
    ADD CONSTRAINT academic_student_campaign_activities_pkey PRIMARY KEY (id);


--
-- Name: convertions academic_student_campaign_convertions_pkey; Type: CONSTRAINT; Schema: academic_student_campaign; Owner: bendo01
--

ALTER TABLE ONLY academic_student_campaign.convertions
    ADD CONSTRAINT academic_student_campaign_convertions_pkey PRIMARY KEY (id);


--
-- Name: detail_activities academic_student_campaign_detail_activities_pkey; Type: CONSTRAINT; Schema: academic_student_campaign; Owner: bendo01
--

ALTER TABLE ONLY academic_student_campaign.detail_activities
    ADD CONSTRAINT academic_student_campaign_detail_activities_pkey PRIMARY KEY (id);


--
-- Name: detail_activity_evaluation_components academic_student_campaign_detail_activity_evaluation_components; Type: CONSTRAINT; Schema: academic_student_campaign; Owner: bendo01
--

ALTER TABLE ONLY academic_student_campaign.detail_activity_evaluation_components
    ADD CONSTRAINT academic_student_campaign_detail_activity_evaluation_components PRIMARY KEY (id);


--
-- Name: adviser_categories academic_student_final_assignment_reference_adviser_categories_; Type: CONSTRAINT; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_reference.adviser_categories
    ADD CONSTRAINT academic_student_final_assignment_reference_adviser_categories_ PRIMARY KEY (id);


--
-- Name: approval_types academic_student_final_assignment_reference_approval_types_pkey; Type: CONSTRAINT; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_reference.approval_types
    ADD CONSTRAINT academic_student_final_assignment_reference_approval_types_pkey PRIMARY KEY (id);


--
-- Name: categories academic_student_final_assignment_reference_categories_pkey; Type: CONSTRAINT; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_reference.categories
    ADD CONSTRAINT academic_student_final_assignment_reference_categories_pkey PRIMARY KEY (id);


--
-- Name: requirements academic_student_final_assignment_reference_requirements_pkey; Type: CONSTRAINT; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_reference.requirements
    ADD CONSTRAINT academic_student_final_assignment_reference_requirements_pkey PRIMARY KEY (id);


--
-- Name: stages academic_student_final_assignment_reference_stages_pkey; Type: CONSTRAINT; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_reference.stages
    ADD CONSTRAINT academic_student_final_assignment_reference_stages_pkey PRIMARY KEY (id);


--
-- Name: varieties academic_student_final_assignment_reference_varieties_pkey; Type: CONSTRAINT; Schema: academic_student_final_assignment_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_reference.varieties
    ADD CONSTRAINT academic_student_final_assignment_reference_varieties_pkey PRIMARY KEY (id);


--
-- Name: advisers academic_student_final_assignment_transaction_advisers_pkey; Type: CONSTRAINT; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_transaction.advisers
    ADD CONSTRAINT academic_student_final_assignment_transaction_advisers_pkey PRIMARY KEY (id);


--
-- Name: evaluation_details academic_student_final_assignment_transaction_evaluation_detail; Type: CONSTRAINT; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_transaction.evaluation_details
    ADD CONSTRAINT academic_student_final_assignment_transaction_evaluation_detail PRIMARY KEY (id);


--
-- Name: evaluation_summaries academic_student_final_assignment_transaction_evaluation_summar; Type: CONSTRAINT; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_transaction.evaluation_summaries
    ADD CONSTRAINT academic_student_final_assignment_transaction_evaluation_summar PRIMARY KEY (id);


--
-- Name: final_assignment_decrees academic_student_final_assignment_transaction_final_assignment_; Type: CONSTRAINT; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_transaction.final_assignment_decrees
    ADD CONSTRAINT academic_student_final_assignment_transaction_final_assignment_ PRIMARY KEY (id);


--
-- Name: prerequisites academic_student_final_assignment_transaction_prerequisites_pke; Type: CONSTRAINT; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_transaction.prerequisites
    ADD CONSTRAINT academic_student_final_assignment_transaction_prerequisites_pke PRIMARY KEY (id);


--
-- Name: schedules academic_student_final_assignment_transaction_schedules_pkey; Type: CONSTRAINT; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_transaction.schedules
    ADD CONSTRAINT academic_student_final_assignment_transaction_schedules_pkey PRIMARY KEY (id);


--
-- Name: submissions academic_student_final_assignment_transaction_submissions_pkey; Type: CONSTRAINT; Schema: academic_student_final_assignment_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_student_final_assignment_transaction.submissions
    ADD CONSTRAINT academic_student_final_assignment_transaction_submissions_pkey PRIMARY KEY (id);


--
-- Name: images asm_images_pkey; Type: CONSTRAINT; Schema: academic_student_master; Owner: bendo01
--

ALTER TABLE ONLY academic_student_master.images
    ADD CONSTRAINT asm_images_pkey PRIMARY KEY (id);


--
-- Name: students asm_students_pkey; Type: CONSTRAINT; Schema: academic_student_master; Owner: bendo01
--

ALTER TABLE ONLY academic_student_master.students
    ADD CONSTRAINT asm_students_pkey PRIMARY KEY (id);


--
-- Name: registrations asr__registrations_pkey; Type: CONSTRAINT; Schema: academic_student_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_student_reference.registrations
    ADD CONSTRAINT asr__registrations_pkey PRIMARY KEY (id);


--
-- Name: finances asr_finances_pkey; Type: CONSTRAINT; Schema: academic_student_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_student_reference.finances
    ADD CONSTRAINT asr_finances_pkey PRIMARY KEY (id);


--
-- Name: resign_statuses asr_resign_statuses_pkey; Type: CONSTRAINT; Schema: academic_student_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_student_reference.resign_statuses
    ADD CONSTRAINT asr_resign_statuses_pkey PRIMARY KEY (id);


--
-- Name: selection_types asr_selection_types_pkey; Type: CONSTRAINT; Schema: academic_student_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_student_reference.selection_types
    ADD CONSTRAINT asr_selection_types_pkey PRIMARY KEY (id);


--
-- Name: statuses asr_statuses_pkey; Type: CONSTRAINT; Schema: academic_student_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_student_reference.statuses
    ADD CONSTRAINT asr_statuses_pkey PRIMARY KEY (id);


--
-- Name: answers asvm_answers_pkey; Type: CONSTRAINT; Schema: academic_survey_master; Owner: bendo01
--

ALTER TABLE ONLY academic_survey_master.answers
    ADD CONSTRAINT asvm_answers_pkey PRIMARY KEY (id);


--
-- Name: bundle_question asvm_bundle_question_pkey; Type: CONSTRAINT; Schema: academic_survey_master; Owner: bendo01
--

ALTER TABLE ONLY academic_survey_master.bundle_question
    ADD CONSTRAINT asvm_bundle_question_pkey PRIMARY KEY (id);


--
-- Name: bundles asvm_bundles_pkey; Type: CONSTRAINT; Schema: academic_survey_master; Owner: bendo01
--

ALTER TABLE ONLY academic_survey_master.bundles
    ADD CONSTRAINT asvm_bundles_pkey PRIMARY KEY (id);


--
-- Name: questions asvm_questions_pkey; Type: CONSTRAINT; Schema: academic_survey_master; Owner: bendo01
--

ALTER TABLE ONLY academic_survey_master.questions
    ADD CONSTRAINT asvm_questions_pkey PRIMARY KEY (id);


--
-- Name: bundle_categories asvr_bundle_categories_pkey; Type: CONSTRAINT; Schema: academic_survey_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_survey_reference.bundle_categories
    ADD CONSTRAINT asvr_bundle_categories_pkey PRIMARY KEY (id);


--
-- Name: question_varieties asvr_question_varieties_pkey; Type: CONSTRAINT; Schema: academic_survey_reference; Owner: bendo01
--

ALTER TABLE ONLY academic_survey_reference.question_varieties
    ADD CONSTRAINT asvr_question_varieties_pkey PRIMARY KEY (id);


--
-- Name: conducts asvt_conducts_pkey; Type: CONSTRAINT; Schema: academic_survey_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_survey_transaction.conducts
    ADD CONSTRAINT asvt_conducts_pkey PRIMARY KEY (id);


--
-- Name: responds asvt_responds_pkey; Type: CONSTRAINT; Schema: academic_survey_transaction; Owner: bendo01
--

ALTER TABLE ONLY academic_survey_transaction.responds
    ADD CONSTRAINT asvt_responds_pkey PRIMARY KEY (id);


--
-- Name: permission_position_type auth_permission_position_type_pkey; Type: CONSTRAINT; Schema: auth; Owner: bendo01
--

ALTER TABLE ONLY auth.permission_position_type
    ADD CONSTRAINT auth_permission_position_type_pkey PRIMARY KEY (id);


--
-- Name: permission_user auth_permission_user_pkey; Type: CONSTRAINT; Schema: auth; Owner: bendo01
--

ALTER TABLE ONLY auth.permission_user
    ADD CONSTRAINT auth_permission_user_pkey PRIMARY KEY (id);


--
-- Name: permissions auth_permissions_pkey; Type: CONSTRAINT; Schema: auth; Owner: bendo01
--

ALTER TABLE ONLY auth.permissions
    ADD CONSTRAINT auth_permissions_pkey PRIMARY KEY (id);


--
-- Name: user_position_type auth_user_position_type_pkey; Type: CONSTRAINT; Schema: auth; Owner: bendo01
--

ALTER TABLE ONLY auth.user_position_type
    ADD CONSTRAINT auth_user_position_type_pkey PRIMARY KEY (id);


--
-- Name: users auth_users_pkey; Type: CONSTRAINT; Schema: auth; Owner: bendo01
--

ALTER TABLE ONLY auth.users
    ADD CONSTRAINT auth_users_pkey PRIMARY KEY (id);


--
-- Name: verifications auth_verifications_pkey; Type: CONSTRAINT; Schema: auth; Owner: bendo01
--

ALTER TABLE ONLY auth.verifications
    ADD CONSTRAINT auth_verifications_pkey PRIMARY KEY (id);


--
-- Name: roles roles_pkey; Type: CONSTRAINT; Schema: auth; Owner: bendo01
--

ALTER TABLE ONLY auth.roles
    ADD CONSTRAINT roles_pkey PRIMARY KEY (id);


--
-- Name: buildings bm_buildings_pkey; Type: CONSTRAINT; Schema: building_master; Owner: bendo01
--

ALTER TABLE ONLY building_master.buildings
    ADD CONSTRAINT bm_buildings_pkey PRIMARY KEY (id);


--
-- Name: rooms bm_rooms_pkey; Type: CONSTRAINT; Schema: building_master; Owner: bendo01
--

ALTER TABLE ONLY building_master.rooms
    ADD CONSTRAINT bm_rooms_pkey PRIMARY KEY (id);


--
-- Name: categories br_categories_pkey; Type: CONSTRAINT; Schema: building_reference; Owner: bendo01
--

ALTER TABLE ONLY building_reference.categories
    ADD CONSTRAINT br_categories_pkey PRIMARY KEY (id);


--
-- Name: conditions br_conditions_pkey; Type: CONSTRAINT; Schema: building_reference; Owner: bendo01
--

ALTER TABLE ONLY building_reference.conditions
    ADD CONSTRAINT br_conditions_pkey PRIMARY KEY (id);


--
-- Name: room_types br_room_types_pkey; Type: CONSTRAINT; Schema: building_reference; Owner: bendo01
--

ALTER TABLE ONLY building_reference.room_types
    ADD CONSTRAINT br_room_types_pkey PRIMARY KEY (id);


--
-- Name: varieties br_varieties_pkey; Type: CONSTRAINT; Schema: building_reference; Owner: bendo01
--

ALTER TABLE ONLY building_reference.varieties
    ADD CONSTRAINT br_varieties_pkey PRIMARY KEY (id);


--
-- Name: electronic_mails cm_electronic_mails_pkey; Type: CONSTRAINT; Schema: contact_master; Owner: bendo01
--

ALTER TABLE ONLY contact_master.electronic_mails
    ADD CONSTRAINT cm_electronic_mails_pkey PRIMARY KEY (id);


--
-- Name: phones cm_phones_pkey; Type: CONSTRAINT; Schema: contact_master; Owner: bendo01
--

ALTER TABLE ONLY contact_master.phones
    ADD CONSTRAINT cm_phones_pkey PRIMARY KEY (id);


--
-- Name: residences cm_residences_pkey; Type: CONSTRAINT; Schema: contact_master; Owner: bendo01
--

ALTER TABLE ONLY contact_master.residences
    ADD CONSTRAINT cm_residences_pkey PRIMARY KEY (id);


--
-- Name: websites cm_websites_pkey; Type: CONSTRAINT; Schema: contact_master; Owner: bendo01
--

ALTER TABLE ONLY contact_master.websites
    ADD CONSTRAINT cm_websites_pkey PRIMARY KEY (id);


--
-- Name: electronic_mail_types cr_electronic_mail_types_pkey; Type: CONSTRAINT; Schema: contact_reference; Owner: bendo01
--

ALTER TABLE ONLY contact_reference.electronic_mail_types
    ADD CONSTRAINT cr_electronic_mail_types_pkey PRIMARY KEY (id);


--
-- Name: phone_types cr_phone_types_pkey; Type: CONSTRAINT; Schema: contact_reference; Owner: bendo01
--

ALTER TABLE ONLY contact_reference.phone_types
    ADD CONSTRAINT cr_phone_types_pkey PRIMARY KEY (id);


--
-- Name: residence_types cr_residence_types_pkey; Type: CONSTRAINT; Schema: contact_reference; Owner: bendo01
--

ALTER TABLE ONLY contact_reference.residence_types
    ADD CONSTRAINT cr_residence_types_pkey PRIMARY KEY (id);


--
-- Name: website_types cr_website_types_pkey; Type: CONSTRAINT; Schema: contact_reference; Owner: bendo01
--

ALTER TABLE ONLY contact_reference.website_types
    ADD CONSTRAINT cr_website_types_pkey PRIMARY KEY (id);


--
-- Name: archive_types document_reference_archive_types_pkey; Type: CONSTRAINT; Schema: document_reference; Owner: bendo01
--

ALTER TABLE ONLY document_reference.archive_types
    ADD CONSTRAINT document_reference_archive_types_pkey PRIMARY KEY (id);


--
-- Name: archives document_transaction_archives_pkey; Type: CONSTRAINT; Schema: document_transaction; Owner: bendo01
--

ALTER TABLE ONLY document_transaction.archives
    ADD CONSTRAINT document_transaction_archives_pkey PRIMARY KEY (id);


--
-- Name: estimasi feeder_akumulasi_estimasi_pkey; Type: CONSTRAINT; Schema: feeder_akumulasi; Owner: bendo01
--

ALTER TABLE ONLY feeder_akumulasi.estimasi
    ADD CONSTRAINT feeder_akumulasi_estimasi_pkey PRIMARY KEY (id);


--
-- Name: jumlah_data feeder_akumulasi_jumlah_data_pkey; Type: CONSTRAINT; Schema: feeder_akumulasi; Owner: bendo01
--

ALTER TABLE ONLY feeder_akumulasi.jumlah_data
    ADD CONSTRAINT feeder_akumulasi_jumlah_data_pkey PRIMARY KEY (id);


--
-- Name: kredential feeder_akun_kredential_pkey; Type: CONSTRAINT; Schema: feeder_akun; Owner: bendo01
--

ALTER TABLE ONLY feeder_akun.kredential
    ADD CONSTRAINT feeder_akun_kredential_pkey PRIMARY KEY (id);


--
-- Name: aktifitas_kuliah_mahasiswa feeder_master_aktifitas_kuliah_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.aktifitas_kuliah_mahasiswa
    ADD CONSTRAINT feeder_master_aktifitas_kuliah_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: aktifitas_mahasiswa feeder_master_aktifitas_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.aktifitas_mahasiswa
    ADD CONSTRAINT feeder_master_aktifitas_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: aktifitas_mengajar_dosen feeder_master_aktifitas_mengajar_dosen_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.aktifitas_mengajar_dosen
    ADD CONSTRAINT feeder_master_aktifitas_mengajar_dosen_pkey PRIMARY KEY (id);


--
-- Name: anggota_aktifitas_mahasiswa feeder_master_anggota_aktifitas_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.anggota_aktifitas_mahasiswa
    ADD CONSTRAINT feeder_master_anggota_aktifitas_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: bidang_minat_perguruan_tinggi feeder_master_bidang_minat_perguruan_tinggi_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.bidang_minat_perguruan_tinggi
    ADD CONSTRAINT feeder_master_bidang_minat_perguruan_tinggi_pkey PRIMARY KEY (id);


--
-- Name: bimbing_mahasiswa feeder_master_bimbing_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.bimbing_mahasiswa
    ADD CONSTRAINT feeder_master_bimbing_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: biodata_dosen feeder_master_biodata_dosen_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.biodata_dosen
    ADD CONSTRAINT feeder_master_biodata_dosen_pkey PRIMARY KEY (id);


--
-- Name: biodata_mahasiswa feeder_master_biodata_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.biodata_mahasiswa
    ADD CONSTRAINT feeder_master_biodata_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: dosen_pembimbing feeder_master_dosen_pembimbing_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.dosen_pembimbing
    ADD CONSTRAINT feeder_master_dosen_pembimbing_pkey PRIMARY KEY (id);


--
-- Name: dosen feeder_master_dosen_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.dosen
    ADD CONSTRAINT feeder_master_dosen_pkey PRIMARY KEY (id);


--
-- Name: fakultas feeder_master_fakultas_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.fakultas
    ADD CONSTRAINT feeder_master_fakultas_pkey PRIMARY KEY (id);


--
-- Name: hitung_transkrip_angkatan_mahasiswa feeder_master_hitung_transkrip_angkatan_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.hitung_transkrip_angkatan_mahasiswa
    ADD CONSTRAINT feeder_master_hitung_transkrip_angkatan_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: kartu_rencana_studi_mahasiswa feeder_master_kartu_rencana_studi_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.kartu_rencana_studi_mahasiswa
    ADD CONSTRAINT feeder_master_kartu_rencana_studi_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: kelas_kuliah feeder_master_kelas_kuliah_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.kelas_kuliah
    ADD CONSTRAINT feeder_master_kelas_kuliah_pkey PRIMARY KEY (id);


--
-- Name: komponen_evaluasi_kelas feeder_master_komponen_evaluasi_kelas_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.komponen_evaluasi_kelas
    ADD CONSTRAINT feeder_master_komponen_evaluasi_kelas_pkey PRIMARY KEY (id);


--
-- Name: konsistensi_data feeder_master_konsistensi_data_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.konsistensi_data
    ADD CONSTRAINT feeder_master_konsistensi_data_pkey PRIMARY KEY (id);


--
-- Name: konversi_kampus_merdeka feeder_master_konversi_kampus_merdeka_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.konversi_kampus_merdeka
    ADD CONSTRAINT feeder_master_konversi_kampus_merdeka_pkey PRIMARY KEY (id);


--
-- Name: kurikulum feeder_master_kurikulum_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.kurikulum
    ADD CONSTRAINT feeder_master_kurikulum_pkey PRIMARY KEY (id);


--
-- Name: mahasiswa_bimbingan_dosen feeder_master_mahasiswa_bimbingan_dosen_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.mahasiswa_bimbingan_dosen
    ADD CONSTRAINT feeder_master_mahasiswa_bimbingan_dosen_pkey PRIMARY KEY (id);


--
-- Name: mahasiswa_lulus_dropout feeder_master_mahasiswa_lulus_dropout_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.mahasiswa_lulus_dropout
    ADD CONSTRAINT feeder_master_mahasiswa_lulus_dropout_pkey PRIMARY KEY (id);


--
-- Name: mahasiswa feeder_master_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.mahasiswa
    ADD CONSTRAINT feeder_master_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: matakuliah_kurikulum feeder_master_matakuliah_kurikulum_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.matakuliah_kurikulum
    ADD CONSTRAINT feeder_master_matakuliah_kurikulum_pkey PRIMARY KEY (id);


--
-- Name: matakuliah feeder_master_matakuliah_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.matakuliah
    ADD CONSTRAINT feeder_master_matakuliah_pkey PRIMARY KEY (id);


--
-- Name: nilai_perkuliahan_kelas feeder_master_nilai_perkuliahan_kelas_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.nilai_perkuliahan_kelas
    ADD CONSTRAINT feeder_master_nilai_perkuliahan_kelas_pkey PRIMARY KEY (id);


--
-- Name: nilai_transfer_pendidikan_mahasiswa feeder_master_nilai_transfer_pendidikan_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.nilai_transfer_pendidikan_mahasiswa
    ADD CONSTRAINT feeder_master_nilai_transfer_pendidikan_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: penugasan_dosen feeder_master_penugasan_dosen_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.penugasan_dosen
    ADD CONSTRAINT feeder_master_penugasan_dosen_pkey PRIMARY KEY (id);


--
-- Name: perguruan_tinggi feeder_master_perguruan_tinggi_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.perguruan_tinggi
    ADD CONSTRAINT feeder_master_perguruan_tinggi_pkey PRIMARY KEY (id);


--
-- Name: periode_aktif feeder_master_periode_aktif_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.periode_aktif
    ADD CONSTRAINT feeder_master_periode_aktif_pkey PRIMARY KEY (id);


--
-- Name: periode_perkuliahan feeder_master_periode_perkuliahan_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.periode_perkuliahan
    ADD CONSTRAINT feeder_master_periode_perkuliahan_pkey PRIMARY KEY (id);


--
-- Name: perkuliahan_mahasiswa feeder_master_perkuliahan_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.perkuliahan_mahasiswa
    ADD CONSTRAINT feeder_master_perkuliahan_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: peserta_kelas_kuliah feeder_master_peserta_kelas_kuliah_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.peserta_kelas_kuliah
    ADD CONSTRAINT feeder_master_peserta_kelas_kuliah_pkey PRIMARY KEY (id);


--
-- Name: prestasi_mahasiswa feeder_master_prestasi_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.prestasi_mahasiswa
    ADD CONSTRAINT feeder_master_prestasi_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: profil_perguruan_tinggi feeder_master_profil_perguruan_tinggi_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.profil_perguruan_tinggi
    ADD CONSTRAINT feeder_master_profil_perguruan_tinggi_pkey PRIMARY KEY (id);


--
-- Name: program_studi feeder_master_program_studi_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.program_studi
    ADD CONSTRAINT feeder_master_program_studi_pkey PRIMARY KEY (id);


--
-- Name: rencana_evaluasi feeder_master_rencana_evaluasi_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.rencana_evaluasi
    ADD CONSTRAINT feeder_master_rencana_evaluasi_pkey PRIMARY KEY (id);


--
-- Name: riwayat_fungsional_dosen feeder_master_riwayat_fungsional_dosen_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.riwayat_fungsional_dosen
    ADD CONSTRAINT feeder_master_riwayat_fungsional_dosen_pkey PRIMARY KEY (id);


--
-- Name: riwayat_nilai_mahasiswa feeder_master_riwayat_nilai_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.riwayat_nilai_mahasiswa
    ADD CONSTRAINT feeder_master_riwayat_nilai_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: riwayat_pangkat_dosen feeder_master_riwayat_pangkat_dosen_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.riwayat_pangkat_dosen
    ADD CONSTRAINT feeder_master_riwayat_pangkat_dosen_pkey PRIMARY KEY (id);


--
-- Name: riwayat_pendidikan_dosen feeder_master_riwayat_pendidikan_dosen_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.riwayat_pendidikan_dosen
    ADD CONSTRAINT feeder_master_riwayat_pendidikan_dosen_pkey PRIMARY KEY (id);


--
-- Name: riwayat_penelitian_dosen feeder_master_riwayat_penelitian_dosen_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.riwayat_penelitian_dosen
    ADD CONSTRAINT feeder_master_riwayat_penelitian_dosen_pkey PRIMARY KEY (id);


--
-- Name: riwayat_sertifikasi_dosen feeder_master_riwayat_sertifikasi_dosen_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.riwayat_sertifikasi_dosen
    ADD CONSTRAINT feeder_master_riwayat_sertifikasi_dosen_pkey PRIMARY KEY (id);


--
-- Name: skala_nilai_program_studi feeder_master_skala_nilai_program_studi_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.skala_nilai_program_studi
    ADD CONSTRAINT feeder_master_skala_nilai_program_studi_pkey PRIMARY KEY (id);


--
-- Name: substansi_matakuliah feeder_master_substansi_matakuliah_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.substansi_matakuliah
    ADD CONSTRAINT feeder_master_substansi_matakuliah_pkey PRIMARY KEY (id);


--
-- Name: transkrip_mahasiswa feeder_master_transkrip_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.transkrip_mahasiswa
    ADD CONSTRAINT feeder_master_transkrip_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: uji_mahasiswa feeder_master_uji_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.uji_mahasiswa
    ADD CONSTRAINT feeder_master_uji_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: dosen_pengajar_kelas_kuliah pfeeder_master_dosen_pengajar_kelas_kuliah_pkey; Type: CONSTRAINT; Schema: feeder_master; Owner: bendo01
--

ALTER TABLE ONLY feeder_master.dosen_pengajar_kelas_kuliah
    ADD CONSTRAINT pfeeder_master_dosen_pengajar_kelas_kuliah_pkey PRIMARY KEY (id);


--
-- Name: agama feeder_referensi_agama_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.agama
    ADD CONSTRAINT feeder_referensi_agama_pkey PRIMARY KEY (id);


--
-- Name: alat_transportasi feeder_referensi_alat_transportasi_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.alat_transportasi
    ADD CONSTRAINT feeder_referensi_alat_transportasi_pkey PRIMARY KEY (id);


--
-- Name: bentuk_pendidikan feeder_referensi_bentuk_pendidikan_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.bentuk_pendidikan
    ADD CONSTRAINT feeder_referensi_bentuk_pendidikan_pkey PRIMARY KEY (id);


--
-- Name: ikatan_kerja_sumber_daya_manusia feeder_referensi_ikatan_kerja_sumber_daya_manusia_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.ikatan_kerja_sumber_daya_manusia
    ADD CONSTRAINT feeder_referensi_ikatan_kerja_sumber_daya_manusia_pkey PRIMARY KEY (id);


--
-- Name: jabatan_fungsional feeder_referensi_jabatan_fungsional_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jabatan_fungsional
    ADD CONSTRAINT feeder_referensi_jabatan_fungsional_pkey PRIMARY KEY (id);


--
-- Name: jalur_masuk feeder_referensi_jalur_masuk_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jalur_masuk
    ADD CONSTRAINT feeder_referensi_jalur_masuk_pkey PRIMARY KEY (id);


--
-- Name: jenis_aktifitas_mahasiswa feeder_referensi_jenis_aktifitas_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jenis_aktifitas_mahasiswa
    ADD CONSTRAINT feeder_referensi_jenis_aktifitas_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: jenis_evaluasi feeder_referensi_jenis_evaluasi_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jenis_evaluasi
    ADD CONSTRAINT feeder_referensi_jenis_evaluasi_pkey PRIMARY KEY (id);


--
-- Name: jenis_keluar feeder_referensi_jenis_keluar_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jenis_keluar
    ADD CONSTRAINT feeder_referensi_jenis_keluar_pkey PRIMARY KEY (id);


--
-- Name: jenis_pendaftaran feeder_referensi_jenis_pendaftaran_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jenis_pendaftaran
    ADD CONSTRAINT feeder_referensi_jenis_pendaftaran_pkey PRIMARY KEY (id);


--
-- Name: jenis_prestasi feeder_referensi_jenis_prestasi_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jenis_prestasi
    ADD CONSTRAINT feeder_referensi_jenis_prestasi_pkey PRIMARY KEY (id);


--
-- Name: jenis_satuan_manajemen_sumberdaya feeder_referensi_jenis_satuan_manajemen_sumberdaya_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jenis_satuan_manajemen_sumberdaya
    ADD CONSTRAINT feeder_referensi_jenis_satuan_manajemen_sumberdaya_pkey PRIMARY KEY (id);


--
-- Name: jenis_sertifikasi feeder_referensi_jenis_sertifikasi_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jenis_sertifikasi
    ADD CONSTRAINT feeder_referensi_jenis_sertifikasi_pkey PRIMARY KEY (id);


--
-- Name: jenis_substansi feeder_referensi_jenis_substansi_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jenis_substansi
    ADD CONSTRAINT feeder_referensi_jenis_substansi_pkey PRIMARY KEY (id);


--
-- Name: jenis_tinggal feeder_referensi_jenis_tinggal_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jenis_tinggal
    ADD CONSTRAINT feeder_referensi_jenis_tinggal_pkey PRIMARY KEY (id);


--
-- Name: jenjang_pendidikan feeder_referensi_jenjang_pendidikan_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.jenjang_pendidikan
    ADD CONSTRAINT feeder_referensi_jenjang_pendidikan_pkey PRIMARY KEY (id);


--
-- Name: kategori_kegiatan feeder_referensi_kategori_kegiatan_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.kategori_kegiatan
    ADD CONSTRAINT feeder_referensi_kategori_kegiatan_pkey PRIMARY KEY (id);


--
-- Name: kebutuhan_khusus feeder_referensi_kebutuhan_khusus_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.kebutuhan_khusus
    ADD CONSTRAINT feeder_referensi_kebutuhan_khusus_pkey PRIMARY KEY (id);


--
-- Name: lembaga_pengangkat feeder_referensi_lembaga_pengangkat_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.lembaga_pengangkat
    ADD CONSTRAINT feeder_referensi_lembaga_pengangkat_pkey PRIMARY KEY (id);


--
-- Name: level_wilayah feeder_referensi_level_wilayah_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.level_wilayah
    ADD CONSTRAINT feeder_referensi_level_wilayah_pkey PRIMARY KEY (id);


--
-- Name: negara feeder_referensi_negara_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.negara
    ADD CONSTRAINT feeder_referensi_negara_pkey PRIMARY KEY (id);


--
-- Name: pangkat_golongan feeder_referensi_pangkat_golongan_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.pangkat_golongan
    ADD CONSTRAINT feeder_referensi_pangkat_golongan_pkey PRIMARY KEY (id);


--
-- Name: pekerjaan feeder_referensi_pekerjaan_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.pekerjaan
    ADD CONSTRAINT feeder_referensi_pekerjaan_pkey PRIMARY KEY (id);


--
-- Name: pembiayaan feeder_referensi_pembiayaan_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.pembiayaan
    ADD CONSTRAINT feeder_referensi_pembiayaan_pkey PRIMARY KEY (id);


--
-- Name: penghasilan feeder_referensi_penghasilan_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.penghasilan
    ADD CONSTRAINT feeder_referensi_penghasilan_pkey PRIMARY KEY (id);


--
-- Name: periode_lampau feeder_referensi_periode_lampau_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.periode_lampau
    ADD CONSTRAINT feeder_referensi_periode_lampau_pkey PRIMARY KEY (id);


--
-- Name: semester feeder_referensi_semester_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.semester
    ADD CONSTRAINT feeder_referensi_semester_pkey PRIMARY KEY (id);


--
-- Name: status_keaktifan_pegawai feeder_referensi_status_keaktifan_pegawai_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.status_keaktifan_pegawai
    ADD CONSTRAINT feeder_referensi_status_keaktifan_pegawai_pkey PRIMARY KEY (id);


--
-- Name: status_kepegawaian feeder_referensi_status_kepegawaian_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.status_kepegawaian
    ADD CONSTRAINT feeder_referensi_status_kepegawaian_pkey PRIMARY KEY (id);


--
-- Name: status_mahasiswa feeder_referensi_status_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.status_mahasiswa
    ADD CONSTRAINT feeder_referensi_status_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: tahun_ajaran feeder_referensi_tahun_ajaran_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.tahun_ajaran
    ADD CONSTRAINT feeder_referensi_tahun_ajaran_pkey PRIMARY KEY (id);


--
-- Name: tingkat_prestasi feeder_referensi_tingkat_prestasi_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.tingkat_prestasi
    ADD CONSTRAINT feeder_referensi_tingkat_prestasi_pkey PRIMARY KEY (id);


--
-- Name: wilayah feeder_referensi_wilayah_pkey; Type: CONSTRAINT; Schema: feeder_referensi; Owner: bendo01
--

ALTER TABLE ONLY feeder_referensi.wilayah
    ADD CONSTRAINT feeder_referensi_wilayah_pkey PRIMARY KEY (id);


--
-- Name: indeks_prestasi_sementara_mahasiswa feeder_rekapitulasi_indeks_prestasi_sementara_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_rekapitulasi; Owner: bendo01
--

ALTER TABLE ONLY feeder_rekapitulasi.indeks_prestasi_sementara_mahasiswa
    ADD CONSTRAINT feeder_rekapitulasi_indeks_prestasi_sementara_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: jumlah_dosen feeder_rekapitulasi_jumlah_dosen_pkey; Type: CONSTRAINT; Schema: feeder_rekapitulasi; Owner: bendo01
--

ALTER TABLE ONLY feeder_rekapitulasi.jumlah_dosen
    ADD CONSTRAINT feeder_rekapitulasi_jumlah_dosen_pkey PRIMARY KEY (id);


--
-- Name: jumlah_mahasiswa feeder_rekapitulasi_jumlah_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_rekapitulasi; Owner: bendo01
--

ALTER TABLE ONLY feeder_rekapitulasi.jumlah_mahasiswa
    ADD CONSTRAINT feeder_rekapitulasi_jumlah_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: kartu_hasil_studi_mahasiswa feeder_rekapitulasi_kartu_hasil_studi_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_rekapitulasi; Owner: bendo01
--

ALTER TABLE ONLY feeder_rekapitulasi.kartu_hasil_studi_mahasiswa
    ADD CONSTRAINT feeder_rekapitulasi_kartu_hasil_studi_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: kartu_rencana_studi_mahasiswa feeder_rekapitulasi_kartu_rencana_studi_mahasiswa_pkey; Type: CONSTRAINT; Schema: feeder_rekapitulasi; Owner: bendo01
--

ALTER TABLE ONLY feeder_rekapitulasi.kartu_rencana_studi_mahasiswa
    ADD CONSTRAINT feeder_rekapitulasi_kartu_rencana_studi_mahasiswa_pkey PRIMARY KEY (id);


--
-- Name: laporan feeder_rekapitulasi_laporans_pkey; Type: CONSTRAINT; Schema: feeder_rekapitulasi; Owner: bendo01
--

ALTER TABLE ONLY feeder_rekapitulasi.laporan
    ADD CONSTRAINT feeder_rekapitulasi_laporans_pkey PRIMARY KEY (id);


--
-- Name: employees im_employees_pkey; Type: CONSTRAINT; Schema: institution_master; Owner: bendo01
--

ALTER TABLE ONLY institution_master.employees
    ADD CONSTRAINT im_employees_pkey PRIMARY KEY (id);


--
-- Name: institutions im_institutions_pkey; Type: CONSTRAINT; Schema: institution_master; Owner: bendo01
--

ALTER TABLE ONLY institution_master.institutions
    ADD CONSTRAINT im_institutions_pkey PRIMARY KEY (id);


--
-- Name: staffes im_staffes_pkey; Type: CONSTRAINT; Schema: institution_master; Owner: bendo01
--

ALTER TABLE ONLY institution_master.staffes
    ADD CONSTRAINT im_staffes_pkey PRIMARY KEY (id);


--
-- Name: units im_units_pkey; Type: CONSTRAINT; Schema: institution_master; Owner: bendo01
--

ALTER TABLE ONLY institution_master.units
    ADD CONSTRAINT im_units_pkey PRIMARY KEY (id);


--
-- Name: categories ir_categories_pkey; Type: CONSTRAINT; Schema: institution_reference; Owner: bendo01
--

ALTER TABLE ONLY institution_reference.categories
    ADD CONSTRAINT ir_categories_pkey PRIMARY KEY (id);


--
-- Name: position_types ir_position_types_pkey; Type: CONSTRAINT; Schema: institution_reference; Owner: bendo01
--

ALTER TABLE ONLY institution_reference.position_types
    ADD CONSTRAINT ir_position_types_pkey PRIMARY KEY (id);


--
-- Name: unit_types ir_unit_types_pkey; Type: CONSTRAINT; Schema: institution_reference; Owner: bendo01
--

ALTER TABLE ONLY institution_reference.unit_types
    ADD CONSTRAINT ir_unit_types_pkey PRIMARY KEY (id);


--
-- Name: varieties ir_varieties_pkey; Type: CONSTRAINT; Schema: institution_reference; Owner: bendo01
--

ALTER TABLE ONLY institution_reference.varieties
    ADD CONSTRAINT ir_varieties_pkey PRIMARY KEY (id);


--
-- Name: categories literate_categories_pkey; Type: CONSTRAINT; Schema: literate; Owner: bendo01
--

ALTER TABLE ONLY literate.categories
    ADD CONSTRAINT literate_categories_pkey PRIMARY KEY (id);


--
-- Name: educations literate_educations_pkey; Type: CONSTRAINT; Schema: literate; Owner: bendo01
--

ALTER TABLE ONLY literate.educations
    ADD CONSTRAINT literate_educations_pkey PRIMARY KEY (id);


--
-- Name: groups literate_groups_pkey; Type: CONSTRAINT; Schema: literate; Owner: bendo01
--

ALTER TABLE ONLY literate.groups
    ADD CONSTRAINT literate_groups_pkey PRIMARY KEY (id);


--
-- Name: levels literate_levels_pkey; Type: CONSTRAINT; Schema: literate; Owner: bendo01
--

ALTER TABLE ONLY literate.levels
    ADD CONSTRAINT literate_levels_pkey PRIMARY KEY (id);


--
-- Name: varieties literate_varieties_pkey; Type: CONSTRAINT; Schema: literate; Owner: bendo01
--

ALTER TABLE ONLY literate.varieties
    ADD CONSTRAINT literate_varieties_pkey PRIMARY KEY (id);


--
-- Name: continents location_continents_pkey; Type: CONSTRAINT; Schema: location; Owner: bendo01
--

ALTER TABLE ONLY location.continents
    ADD CONSTRAINT location_continents_pkey PRIMARY KEY (id);


--
-- Name: countries location_countries_pkey; Type: CONSTRAINT; Schema: location; Owner: bendo01
--

ALTER TABLE ONLY location.countries
    ADD CONSTRAINT location_countries_pkey PRIMARY KEY (id);


--
-- Name: provinces location_provinces_pkey; Type: CONSTRAINT; Schema: location; Owner: bendo01
--

ALTER TABLE ONLY location.provinces
    ADD CONSTRAINT location_provinces_pkey PRIMARY KEY (id);


--
-- Name: regencies location_regencies_pkey; Type: CONSTRAINT; Schema: location; Owner: bendo01
--

ALTER TABLE ONLY location.regencies
    ADD CONSTRAINT location_regencies_pkey PRIMARY KEY (id);


--
-- Name: regency_types location_regency_types_pkey; Type: CONSTRAINT; Schema: location; Owner: bendo01
--

ALTER TABLE ONLY location.regency_types
    ADD CONSTRAINT location_regency_types_pkey PRIMARY KEY (id);


--
-- Name: regions location_regions_pkey; Type: CONSTRAINT; Schema: location; Owner: bendo01
--

ALTER TABLE ONLY location.regions
    ADD CONSTRAINT location_regions_pkey PRIMARY KEY (id);


--
-- Name: sub_districts location_sub_districts_pkey; Type: CONSTRAINT; Schema: location; Owner: bendo01
--

ALTER TABLE ONLY location.sub_districts
    ADD CONSTRAINT location_sub_districts_pkey PRIMARY KEY (id);


--
-- Name: villages location_villages_pkey; Type: CONSTRAINT; Schema: location; Owner: bendo01
--

ALTER TABLE ONLY location.villages
    ADD CONSTRAINT location_villages_pkey PRIMARY KEY (id);


--
-- Name: accounts payment_midtrans_accounts_pkey; Type: CONSTRAINT; Schema: payment_midtrans; Owner: bendo01
--

ALTER TABLE ONLY payment_midtrans.accounts
    ADD CONSTRAINT payment_midtrans_accounts_pkey PRIMARY KEY (id);


--
-- Name: billing_addresses payment_midtrans_billing_addresses_pkey; Type: CONSTRAINT; Schema: payment_midtrans; Owner: bendo01
--

ALTER TABLE ONLY payment_midtrans.billing_addresses
    ADD CONSTRAINT payment_midtrans_billing_addresses_pkey PRIMARY KEY (id);


--
-- Name: customer_details payment_midtrans_customer_details_pkey; Type: CONSTRAINT; Schema: payment_midtrans; Owner: bendo01
--

ALTER TABLE ONLY payment_midtrans.customer_details
    ADD CONSTRAINT payment_midtrans_customer_details_pkey PRIMARY KEY (id);


--
-- Name: item_details payment_midtrans_item_details_pkey; Type: CONSTRAINT; Schema: payment_midtrans; Owner: bendo01
--

ALTER TABLE ONLY payment_midtrans.item_details
    ADD CONSTRAINT payment_midtrans_item_details_pkey PRIMARY KEY (id);


--
-- Name: shipping_addresses payment_midtrans_shipping_addresses_pkey; Type: CONSTRAINT; Schema: payment_midtrans; Owner: bendo01
--

ALTER TABLE ONLY payment_midtrans.shipping_addresses
    ADD CONSTRAINT payment_midtrans_shipping_addresses_pkey PRIMARY KEY (id);


--
-- Name: transaction_details payment_midtrans_transaction_details_pkey; Type: CONSTRAINT; Schema: payment_midtrans; Owner: bendo01
--

ALTER TABLE ONLY payment_midtrans.transaction_details
    ADD CONSTRAINT payment_midtrans_transaction_details_pkey PRIMARY KEY (id);


--
-- Name: biodatas pm_biodatas_pkey; Type: CONSTRAINT; Schema: person_master; Owner: bendo01
--

ALTER TABLE ONLY person_master.biodatas
    ADD CONSTRAINT pm_biodatas_pkey PRIMARY KEY (id);


--
-- Name: family_card_members pm_family_card_members_pkey; Type: CONSTRAINT; Schema: person_master; Owner: bendo01
--

ALTER TABLE ONLY person_master.family_card_members
    ADD CONSTRAINT pm_family_card_members_pkey PRIMARY KEY (id);


--
-- Name: family_cards pm_family_cards_pkey; Type: CONSTRAINT; Schema: person_master; Owner: bendo01
--

ALTER TABLE ONLY person_master.family_cards
    ADD CONSTRAINT pm_family_cards_pkey PRIMARY KEY (id);


--
-- Name: images pm_images_pkey; Type: CONSTRAINT; Schema: person_master; Owner: bendo01
--

ALTER TABLE ONLY person_master.images
    ADD CONSTRAINT pm_images_pkey PRIMARY KEY (id);


--
-- Name: individuals pm_individuals_pkey; Type: CONSTRAINT; Schema: person_master; Owner: bendo01
--

ALTER TABLE ONLY person_master.individuals
    ADD CONSTRAINT pm_individuals_pkey PRIMARY KEY (id);


--
-- Name: age_classifications person_reference_age_classifications_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.age_classifications
    ADD CONSTRAINT person_reference_age_classifications_pkey PRIMARY KEY (id);


--
-- Name: blood_types person_reference_blood_types_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.blood_types
    ADD CONSTRAINT person_reference_blood_types_pkey PRIMARY KEY (id);


--
-- Name: eye_colors person_reference_eye_colors_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.eye_colors
    ADD CONSTRAINT person_reference_eye_colors_pkey PRIMARY KEY (id);


--
-- Name: genders person_reference_genders_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.genders
    ADD CONSTRAINT person_reference_genders_pkey PRIMARY KEY (id);


--
-- Name: hair_colors person_reference_hair_colors_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.hair_colors
    ADD CONSTRAINT person_reference_hair_colors_pkey PRIMARY KEY (id);


--
-- Name: hair_types person_reference_hair_types_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.hair_types
    ADD CONSTRAINT person_reference_hair_types_pkey PRIMARY KEY (id);


--
-- Name: identification_types person_reference_identification_types_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.identification_types
    ADD CONSTRAINT person_reference_identification_types_pkey PRIMARY KEY (id);


--
-- Name: incomes person_reference_incomes_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.incomes
    ADD CONSTRAINT person_reference_incomes_pkey PRIMARY KEY (id);


--
-- Name: marital_statuses person_reference_marital_statuses_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.marital_statuses
    ADD CONSTRAINT person_reference_marital_statuses_pkey PRIMARY KEY (id);


--
-- Name: occupations person_reference_occupations_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.occupations
    ADD CONSTRAINT person_reference_occupations_pkey PRIMARY KEY (id);


--
-- Name: professions person_reference_professions_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.professions
    ADD CONSTRAINT person_reference_professions_pkey PRIMARY KEY (id);


--
-- Name: relative_types person_reference_relative_types_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.relative_types
    ADD CONSTRAINT person_reference_relative_types_pkey PRIMARY KEY (id);


--
-- Name: religions person_reference_religions_pkey; Type: CONSTRAINT; Schema: person_reference; Owner: bendo01
--

ALTER TABLE ONLY person_reference.religions
    ADD CONSTRAINT person_reference_religions_pkey PRIMARY KEY (id);


--
-- Name: audits audits_pkey; Type: CONSTRAINT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.audits
    ADD CONSTRAINT audits_pkey PRIMARY KEY (id);


--
-- Name: cache_locks cache_locks_pkey; Type: CONSTRAINT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.cache_locks
    ADD CONSTRAINT cache_locks_pkey PRIMARY KEY (key);


--
-- Name: cache cache_pkey; Type: CONSTRAINT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.cache
    ADD CONSTRAINT cache_pkey PRIMARY KEY (key);


--
-- Name: failed_jobs failed_jobs_pkey; Type: CONSTRAINT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.failed_jobs
    ADD CONSTRAINT failed_jobs_pkey PRIMARY KEY (id);


--
-- Name: job_batches job_batches_pkey; Type: CONSTRAINT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.job_batches
    ADD CONSTRAINT job_batches_pkey PRIMARY KEY (id);


--
-- Name: jobs jobs_pkey; Type: CONSTRAINT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.jobs
    ADD CONSTRAINT jobs_pkey PRIMARY KEY (id);


--
-- Name: notifications notifications_pkey; Type: CONSTRAINT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.notifications
    ADD CONSTRAINT notifications_pkey PRIMARY KEY (id);


--
-- Name: failed_jobs public_failed_jobs_uuid_unique; Type: CONSTRAINT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.failed_jobs
    ADD CONSTRAINT public_failed_jobs_uuid_unique UNIQUE (uuid);


--
-- Name: seaql_migrations seaql_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: bendo01
--

ALTER TABLE ONLY public.seaql_migrations
    ADD CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version);


--
-- Name: audits_user_id_user_type_index; Type: INDEX; Schema: public; Owner: bendo01
--

CREATE INDEX audits_user_id_user_type_index ON public.audits USING btree (user_id, user_type);


--
-- Name: notifications_notifiable_type_notifiable_id_index; Type: INDEX; Schema: public; Owner: bendo01
--

CREATE INDEX notifications_notifiable_type_notifiable_id_index ON public.notifications USING btree (notifiable_type, notifiable_id);


--
-- Name: public_jobs_queue_index; Type: INDEX; Schema: public; Owner: bendo01
--

CREATE INDEX public_jobs_queue_index ON public.jobs USING btree (queue);


--
-- PostgreSQL database dump complete
--

