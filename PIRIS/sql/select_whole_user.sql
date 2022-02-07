SELECT
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
FROM bank.client
WHERE id = $1
ORDER BY
surname ASC
