INSERT INTO bank.city (id, name) VALUES 
  (0, 'Minsk'), 
  (1, 'Brest'), 
  (2, 'Grodno'),
  (3, 'Vitebsk'), 
  (4, 'Gomel'), 
  (5, 'Mogilev')
ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name;

INSERT INTO bank.disability (id, name) VALUES 
  (0, 'None'), 
  (1, 'First'), 
  (2, 'Second'),
  (3, 'Third')
ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name;

INSERT INTO bank.marital_status (id, name) VALUES 
  (0, 'Single'), 
  (1, 'Married'), 
  (2, 'Divorced'),
  (3, 'Widowed') 
ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name;

INSERT INTO bank.citizenship (id, name) VALUES 
  (0, 'Belarus'), 
  (1, 'Russia'), 
  (2, 'Ukraine')
ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name;

INSERT INTO bank.currency (id, name) VALUES
  (0, 'BYN'), 
  (1, 'USD'), 
  (2, 'EUR'),
  (3, 'RUB')
ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name;

INSERT INTO bank.deposit_program (id, name, start_date, end_date, is_revokable) VALUES
  (0, 'Хуткі', '2020/01/01', '2027/01/01', FALSE),
	(1, 'На мару', '2015/01/01', '2025/01/01', TRUE)
ON CONFLICT (id) DO UPDATE SET 
  (name, start_date, end_date, is_revokable) = 
    (EXCLUDED.name, EXCLUDED.start_date, EXCLUDED.end_date, EXCLUDED.is_revokable);

INSERT INTO bank.deposit_program_data
  (id, deposit_program, duration, percentage, currency, min_sum)
VALUES
  (1, 1, 95, 14.5, 1, 50),
  (2, 1, 185, 13, 1, 50),
  (3, 1, 370, 13, 1, 50),
  (4, 0, 35, 18.6, 1, 50),
  (5, 0, 95, 18.91, 1, 50),
  (6, 0, 185, 15, 1, 50),
  (7, 0, 275, 14.5, 1, 50),
  (8, 0, 370, 19, 1, 50),
  (9, 0, 735, 19, 1, 50),
  (10, 0, 1110, 12.6, 1, 50),
  (11, 1, 95, 0.75, 2, 25),
  (12, 1, 185, 0.75, 2, 25),
  (13, 1, 370, 0.75, 2, 25),
  (14, 0, 35, 1.25, 2, 25),
  (15, 0, 95, 1.25, 2, 25),
  (16, 0, 185, 1.25, 2, 25),
  (17, 0, 275, 1.25, 2, 25),
  (18, 0, 370, 2.3, 2, 25),
  (19, 0, 735, 2.3, 2, 25),
  (20, 0, 1110, 2.3, 2, 25)
ON CONFLICT (id) DO UPDATE SET 
  (deposit_program, duration, percentage, currency, min_sum) = 
    (EXCLUDED.deposit_program, EXCLUDED.duration, EXCLUDED.percentage, EXCLUDED.currency, EXCLUDED.min_sum);
