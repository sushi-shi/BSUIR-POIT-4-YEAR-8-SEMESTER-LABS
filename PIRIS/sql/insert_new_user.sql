INSERT INTO bank.client (
  surname,
  name,
  father_name,
  birthday,
  gender,
  passport_series,
  passport_number,
  passport_issued_by,
  passport_issued_on,
  passport_identification_number,
  birth_place,
  current_city,
  current_address,
  mobile_phone,
  home_phone,
  email,
  marital_status,
  citizenship,
  disability,
  is_pensioner,
  monthly_income
) VALUES (
  $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
  $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
  $21
)
