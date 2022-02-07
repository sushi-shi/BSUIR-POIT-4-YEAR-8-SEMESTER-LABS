CREATE SCHEMA IF NOT EXISTS bank;

CREATE TABLE IF NOT EXISTS bank.disability (
  id SERIAL,
  name varchar(45) NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS bank.city (
  id SERIAL,
  name varchar(45) NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS bank.marital_status (
  id SERIAL,
  name varchar(45) NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS bank.citizenship (
  id SERIAL,
  name varchar(45) NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS bank.client (
  id SERIAL,
  surname VARCHAR(50) NOT NULL,
  name VARCHAR(50) NOT NULL,
  father_name VARCHAR(50) NOT NULL,
  birthday DATE NOT NULL,
  gender INT NOT NULL,
  passport_series CHAR(2) NOT NULL,
  passport_number CHAR(7) NOT NULL,
  passport_issued_by VARCHAR(100) NOT NULL,
  passport_issued_on DATE NOT NULL,
  passport_identification_number CHAR(14),
  birth_place VARCHAR(100) NOT NULL,
  current_city INT NOT NULL,
  current_address VARCHAR(200) NOT NULL,
  mobile_phone CHAR(12) NULL,
  home_phone CHAR(12) NULL,
  email VARCHAR(60) NULL,
  marital_status INT NOT NULL,
  citizenship INT NOT NULL,
  disability INT NOT NULL,
  is_pensioner BOOL NOT NULL,
  monthly_income VARCHAR(50) NULL,
  foreign key (current_city) references bank.city(id),
  foreign key (marital_status) references bank.marital_status(id),
  foreign key (citizenship) references bank.citizenship(id),
  foreign key (disability) references bank.disability(id),
  constraint AK_PassportNumberUnique UNIQUE(passport_number),
  constraint AK_PassportIDUnique UNIQUE(passport_identification_number)
);

