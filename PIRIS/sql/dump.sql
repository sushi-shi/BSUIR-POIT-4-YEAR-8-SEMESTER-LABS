--
-- PostgreSQL database dump
--

-- Dumped from database version 11.13
-- Dumped by pg_dump version 13.5

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
-- Name: bank; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA bank;


ALTER SCHEMA bank OWNER TO postgres;

SET default_tablespace = '';

--
-- Name: citizenship; Type: TABLE; Schema: bank; Owner: postgres
--

CREATE TABLE bank.citizenship (
    id integer NOT NULL,
    name character varying(45) NOT NULL
);


ALTER TABLE bank.citizenship OWNER TO postgres;

--
-- Name: citizenship_id_seq; Type: SEQUENCE; Schema: bank; Owner: postgres
--

CREATE SEQUENCE bank.citizenship_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE bank.citizenship_id_seq OWNER TO postgres;

--
-- Name: citizenship_id_seq; Type: SEQUENCE OWNED BY; Schema: bank; Owner: postgres
--

ALTER SEQUENCE bank.citizenship_id_seq OWNED BY bank.citizenship.id;


--
-- Name: city; Type: TABLE; Schema: bank; Owner: postgres
--

CREATE TABLE bank.city (
    id integer NOT NULL,
    name character varying(45) NOT NULL
);


ALTER TABLE bank.city OWNER TO postgres;

--
-- Name: city_id_seq; Type: SEQUENCE; Schema: bank; Owner: postgres
--

CREATE SEQUENCE bank.city_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE bank.city_id_seq OWNER TO postgres;

--
-- Name: city_id_seq; Type: SEQUENCE OWNED BY; Schema: bank; Owner: postgres
--

ALTER SEQUENCE bank.city_id_seq OWNED BY bank.city.id;


--
-- Name: client; Type: TABLE; Schema: bank; Owner: postgres
--

CREATE TABLE bank.client (
    id integer NOT NULL,
    surname character varying(50) NOT NULL,
    name character varying(50) NOT NULL,
    father_name character varying(50) NOT NULL,
    birthday date NOT NULL,
    gender integer NOT NULL,
    passport_series character(2) NOT NULL,
    passport_number character(7) NOT NULL,
    passport_issued_by character varying(100) NOT NULL,
    passport_issued_on date NOT NULL,
    passport_identification_number character(15),
    birth_place character varying(100) NOT NULL,
    current_city integer NOT NULL,
    current_address character varying(200) NOT NULL,
    mobile_phone character(12),
    home_phone character(12),
    email character varying(60),
    marital_status integer NOT NULL,
    citizenship integer NOT NULL,
    disability integer NOT NULL,
    is_pensioner boolean NOT NULL,
    monthly_income character varying(50)
);


ALTER TABLE bank.client OWNER TO postgres;

--
-- Name: client_id_seq; Type: SEQUENCE; Schema: bank; Owner: postgres
--

CREATE SEQUENCE bank.client_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE bank.client_id_seq OWNER TO postgres;

--
-- Name: client_id_seq; Type: SEQUENCE OWNED BY; Schema: bank; Owner: postgres
--

ALTER SEQUENCE bank.client_id_seq OWNED BY bank.client.id;


--
-- Name: disability; Type: TABLE; Schema: bank; Owner: postgres
--

CREATE TABLE bank.disability (
    id integer NOT NULL,
    name character varying(45) NOT NULL
);


ALTER TABLE bank.disability OWNER TO postgres;

--
-- Name: disability_id_seq; Type: SEQUENCE; Schema: bank; Owner: postgres
--

CREATE SEQUENCE bank.disability_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE bank.disability_id_seq OWNER TO postgres;

--
-- Name: disability_id_seq; Type: SEQUENCE OWNED BY; Schema: bank; Owner: postgres
--

ALTER SEQUENCE bank.disability_id_seq OWNED BY bank.disability.id;


--
-- Name: marital_status; Type: TABLE; Schema: bank; Owner: postgres
--

CREATE TABLE bank.marital_status (
    id integer NOT NULL,
    name character varying(45) NOT NULL
);


ALTER TABLE bank.marital_status OWNER TO postgres;

--
-- Name: marital_status_id_seq; Type: SEQUENCE; Schema: bank; Owner: postgres
--

CREATE SEQUENCE bank.marital_status_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE bank.marital_status_id_seq OWNER TO postgres;

--
-- Name: marital_status_id_seq; Type: SEQUENCE OWNED BY; Schema: bank; Owner: postgres
--

ALTER SEQUENCE bank.marital_status_id_seq OWNED BY bank.marital_status.id;


--
-- Name: citizenship; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.citizenship (
    id integer NOT NULL,
    name character varying(45) NOT NULL
);


ALTER TABLE public.citizenship OWNER TO postgres;

--
-- Name: city; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.city (
    id integer NOT NULL,
    name character varying(45) NOT NULL
);


ALTER TABLE public.city OWNER TO postgres;

--
-- Name: disability; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.disability (
    id integer NOT NULL,
    name character varying(45) NOT NULL
);


ALTER TABLE public.disability OWNER TO postgres;

--
-- Name: marital_status; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.marital_status (
    id integer NOT NULL,
    name character varying(45) NOT NULL
);


ALTER TABLE public.marital_status OWNER TO postgres;

--
-- Name: citizenship id; Type: DEFAULT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.citizenship ALTER COLUMN id SET DEFAULT nextval('bank.citizenship_id_seq'::regclass);


--
-- Name: city id; Type: DEFAULT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.city ALTER COLUMN id SET DEFAULT nextval('bank.city_id_seq'::regclass);


--
-- Name: client id; Type: DEFAULT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.client ALTER COLUMN id SET DEFAULT nextval('bank.client_id_seq'::regclass);


--
-- Name: disability id; Type: DEFAULT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.disability ALTER COLUMN id SET DEFAULT nextval('bank.disability_id_seq'::regclass);


--
-- Name: marital_status id; Type: DEFAULT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.marital_status ALTER COLUMN id SET DEFAULT nextval('bank.marital_status_id_seq'::regclass);


--
-- Data for Name: citizenship; Type: TABLE DATA; Schema: bank; Owner: postgres
--

COPY bank.citizenship (id, name) FROM stdin;
0	Belarus
1	Russia
2	Ukraine
\.


--
-- Data for Name: city; Type: TABLE DATA; Schema: bank; Owner: postgres
--

COPY bank.city (id, name) FROM stdin;
0	Minsk
1	Brest
2	Grodno
3	Vitebsk
4	Gomel
5	Mogilev
\.


--
-- Data for Name: client; Type: TABLE DATA; Schema: bank; Owner: postgres
--

COPY bank.client (id, surname, name, father_name, birthday, gender, passport_series, passport_number, passport_issued_by, passport_issued_on, passport_identification_number, birth_place, current_city, current_address, mobile_phone, home_phone, email, marital_status, citizenship, disability, is_pensioner, monthly_income) FROM stdin;
1	Вайтусенок	Иван	Александрович	2015-02-05	0	МР	3466194	Ленинский райсполком	2014-02-07	123455432112345	Минское	0	Минская улица	\N	\N	\N	0	0	0	f	\N
2	adsf	dsfsd	asdf	2017-02-07	0	sd	1334443	dsaf dsfa sdf	2022-02-17	321123321122321	sdfasdf dsaf	1	daf dafs adf	\N	\N	\N	0	0	0	f	\N
3	adsf	dsfsd	asdf	2017-02-07	0	sd	3234443	dsaf dsfa sdf	2022-02-17	323123321123121	sdfasdf dsaf	1	daf dafs adf	\N	\N	\N	0	0	0	f	\N
4	adsf	dsfsd	asdf	2017-02-07	0	sd	3044443	dsaf dsfa sdf	2022-02-17	321523321123221	sdfasdf dsaf	1	daf dafs adf	\N	\N	\N	0	0	0	f	\N
5	adsf	dsfsd	asdf	2017-02-07	0	sd	3335443	dsaf dsfa sdf	2022-02-17	321123321123331	sdfasdf dsaf	1	daf dafs adf	\N	\N	\N	0	0	0	f	\N
\.


--
-- Data for Name: disability; Type: TABLE DATA; Schema: bank; Owner: postgres
--

COPY bank.disability (id, name) FROM stdin;
0	None
1	First
2	Second
3	Third
\.


--
-- Data for Name: marital_status; Type: TABLE DATA; Schema: bank; Owner: postgres
--

COPY bank.marital_status (id, name) FROM stdin;
0	Single
1	Married
2	Divorced
3	Widowed
\.


--
-- Data for Name: citizenship; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.citizenship (id, name) FROM stdin;
\.


--
-- Data for Name: city; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.city (id, name) FROM stdin;
\.


--
-- Data for Name: disability; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.disability (id, name) FROM stdin;
\.


--
-- Data for Name: marital_status; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.marital_status (id, name) FROM stdin;
\.


--
-- Name: citizenship_id_seq; Type: SEQUENCE SET; Schema: bank; Owner: postgres
--

SELECT pg_catalog.setval('bank.citizenship_id_seq', 1, false);


--
-- Name: city_id_seq; Type: SEQUENCE SET; Schema: bank; Owner: postgres
--

SELECT pg_catalog.setval('bank.city_id_seq', 3, true);


--
-- Name: client_id_seq; Type: SEQUENCE SET; Schema: bank; Owner: postgres
--

SELECT pg_catalog.setval('bank.client_id_seq', 2, true);


--
-- Name: disability_id_seq; Type: SEQUENCE SET; Schema: bank; Owner: postgres
--

SELECT pg_catalog.setval('bank.disability_id_seq', 1, false);


--
-- Name: marital_status_id_seq; Type: SEQUENCE SET; Schema: bank; Owner: postgres
--

SELECT pg_catalog.setval('bank.marital_status_id_seq', 1, false);


--
-- Name: client ak_passportidunique; Type: CONSTRAINT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.client
    ADD CONSTRAINT ak_passportidunique UNIQUE (passport_identification_number);


--
-- Name: client ak_passportnumberunique; Type: CONSTRAINT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.client
    ADD CONSTRAINT ak_passportnumberunique UNIQUE (passport_number);


--
-- Name: citizenship citizenship_pkey; Type: CONSTRAINT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.citizenship
    ADD CONSTRAINT citizenship_pkey PRIMARY KEY (id);


--
-- Name: city city_pkey; Type: CONSTRAINT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.city
    ADD CONSTRAINT city_pkey PRIMARY KEY (id);


--
-- Name: disability disability_pkey; Type: CONSTRAINT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.disability
    ADD CONSTRAINT disability_pkey PRIMARY KEY (id);


--
-- Name: marital_status marital_status_pkey; Type: CONSTRAINT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.marital_status
    ADD CONSTRAINT marital_status_pkey PRIMARY KEY (id);


--
-- Name: citizenship citizenship_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.citizenship
    ADD CONSTRAINT citizenship_pkey PRIMARY KEY (id);


--
-- Name: city city_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.city
    ADD CONSTRAINT city_pkey PRIMARY KEY (id);


--
-- Name: disability disability_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.disability
    ADD CONSTRAINT disability_pkey PRIMARY KEY (id);


--
-- Name: marital_status marital_status_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.marital_status
    ADD CONSTRAINT marital_status_pkey PRIMARY KEY (id);


--
-- Name: client client_citizenship_fkey; Type: FK CONSTRAINT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.client
    ADD CONSTRAINT client_citizenship_fkey FOREIGN KEY (citizenship) REFERENCES bank.citizenship(id);


--
-- Name: client client_current_city_fkey; Type: FK CONSTRAINT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.client
    ADD CONSTRAINT client_current_city_fkey FOREIGN KEY (current_city) REFERENCES bank.city(id);


--
-- Name: client client_disability_fkey; Type: FK CONSTRAINT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.client
    ADD CONSTRAINT client_disability_fkey FOREIGN KEY (disability) REFERENCES bank.disability(id);


--
-- Name: client client_marital_status_fkey; Type: FK CONSTRAINT; Schema: bank; Owner: postgres
--

ALTER TABLE ONLY bank.client
    ADD CONSTRAINT client_marital_status_fkey FOREIGN KEY (marital_status) REFERENCES bank.marital_status(id);


--
-- Name: SCHEMA bank; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA bank TO bank_admin;


--
-- Name: TABLE citizenship; Type: ACL; Schema: bank; Owner: postgres
--

GRANT ALL ON TABLE bank.citizenship TO bank_admin;


--
-- Name: SEQUENCE citizenship_id_seq; Type: ACL; Schema: bank; Owner: postgres
--

GRANT ALL ON SEQUENCE bank.citizenship_id_seq TO bank_admin;


--
-- Name: TABLE city; Type: ACL; Schema: bank; Owner: postgres
--

GRANT ALL ON TABLE bank.city TO bank_admin;


--
-- Name: SEQUENCE city_id_seq; Type: ACL; Schema: bank; Owner: postgres
--

GRANT ALL ON SEQUENCE bank.city_id_seq TO bank_admin;


--
-- Name: TABLE client; Type: ACL; Schema: bank; Owner: postgres
--

GRANT ALL ON TABLE bank.client TO bank_admin;


--
-- Name: SEQUENCE client_id_seq; Type: ACL; Schema: bank; Owner: postgres
--

GRANT ALL ON SEQUENCE bank.client_id_seq TO bank_admin;


--
-- Name: TABLE disability; Type: ACL; Schema: bank; Owner: postgres
--

GRANT ALL ON TABLE bank.disability TO bank_admin;


--
-- Name: SEQUENCE disability_id_seq; Type: ACL; Schema: bank; Owner: postgres
--

GRANT ALL ON SEQUENCE bank.disability_id_seq TO bank_admin;


--
-- Name: TABLE marital_status; Type: ACL; Schema: bank; Owner: postgres
--

GRANT ALL ON TABLE bank.marital_status TO bank_admin;


--
-- Name: SEQUENCE marital_status_id_seq; Type: ACL; Schema: bank; Owner: postgres
--

GRANT ALL ON SEQUENCE bank.marital_status_id_seq TO bank_admin;


--
-- PostgreSQL database dump complete
--

