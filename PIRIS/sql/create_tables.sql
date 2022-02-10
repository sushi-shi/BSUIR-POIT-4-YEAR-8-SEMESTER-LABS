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
  PRIMARY KEY (id),
  FOREIGN KEY (current_city) REFERENCES bank.city(id),
  FOREIGN KEY (marital_status) REFERENCES bank.marital_status(id),
  FOREIGN KEY (citizenship) REFERENCES bank.citizenship(id),
  FOREIGN KEY (disability) REFERENCES bank.disability(id),
  CONSTRAINT AK_PassportNumberUnique UNIQUE(passport_number),
  CONSTRAINT AK_PassportIDUnique UNIQUE(passport_identification_number)
);

CREATE TABLE IF NOT EXISTS bank.currency (
  id SERIAL,
  name CHAR(3) NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS bank.deposit_program (
  id SERIAL,
  name varchar(50) NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  is_revokable BOOL NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS bank.deposit_program_data (
  id SERIAL,
  deposit_program INT NOT NULL,
  duration INT NOT NULL,
  percentage FLOAT NOT NULL,
  currency INT NOT NULL,
  min_sum INT NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (deposit_program) REFERENCES bank.deposit_program(id),
  FOREIGN KEY (currency) REFERENCES bank.currency(id)
);

CREATE TABLE IF NOT EXISTS bank.deposit (
  id SERIAL,
  client_id INT NOT NULL,
  amount INT NOT NULL,
  deposit_program INT NOT NULL,
  deposit_program_data INT NOT NULL, -- Can be ignored
  num INT NOT NULL, -- What is it used for?
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  current_account INT NOT NULL, -- What is it used for?
  percents_account INT NOT NULL, -- And this?
  PRIMARY KEY (id),
  FOREIGN KEY (deposit_program) REFERENCES bank.deposit_program(id),
  FOREIGN KEY (deposit_program_data) REFERENCES bank.deposit_program_data(id),
  FOREIGN KEY (client_id) REFERENCES bank.client(id)
);

/* CREATE TABLE IF NOT EXISTS bank.BankAccountType ( */
/* 	Id int identity primary key */
/* 	, Name nvarchar(50) not null */
/* ); */

/* CREATE TABLE bank.BankAccount ( */
/* 	Id int identity primary key */
/* 	, Number bigint not null */
/* 	, [Type] int not null */
/* 	, Passive bit not null */
/* 	, Currency int not null */
/* 	, ClientId bigint */
/* 	, Debet float not null default 0 */
/* 	, Kredit float not null default 0 */
/* 	, Deleted bit not null default 0 */
/* 	, foreign key ([Type]) references bank.BankAccountType(Id) */
/* 	, foreign key (Currency) references bank.Currency(Id) */
/* 	, foreign key (ClientId) references client.Client(Id) */
/* ); */


/* CREATE TABLE bank.transactions ( */
/* 	Id int identity primary key, */
/* 	[Date] datetime not null, */
/* 	[From] int, */
/* 	[To] int, */
/* 	BalanceFromBefore float not null, */
/* 	BalanceToBefore float not null, */
/* 	[Sum] float not null */
/* ); */


/* CREATE TABLE bank.CurrentDate ( */
/* 	Id int identity primary key, */
/* 	[Date] date not null */
/* ); */
