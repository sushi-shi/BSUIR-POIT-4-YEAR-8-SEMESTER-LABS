SELECT 
  id, deposit_program, duration, percentage, currency, min_sum
FROM 
  bank.deposit_program_data
ORDER BY
  deposit_program ASC;

