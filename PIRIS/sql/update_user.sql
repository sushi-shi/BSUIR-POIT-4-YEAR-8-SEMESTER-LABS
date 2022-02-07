UPDATE bank.client 
SET
  surname = $1,
  name = $2,
  father_name = $3,
  birthday = $4,
  gender = $5,
  passport_series = $6,
  passport_number = $7,
  passport_issued_by = $8,
  passport_issued_on = $9,
  passport_identification_number = $10,
  birth_place = $11,
  current_city = $12,
  current_address = $13,
  mobile_phone = $14,
  home_phone = $15,
  email = $16,
  marital_status = $17,
  citizenship = $18,
  disability = $19,
  is_pensioner = $20,
  monthly_income = $21
WHERE
  id = $22
