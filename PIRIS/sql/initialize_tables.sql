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
