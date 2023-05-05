---- UPDATE..SET..FROM..LEFT JOIN..WHERE..RETURNING
-- CAUTION: both t.key_md5(target table) and ki.key_md5(join table) must be set as conditions
UPDATE key_invocations t
  SET used = t.used + 1
  FROM key_invocations ki
  LEFT JOIN api_keys ak ON ki.key_md5 = ak.key_md5
  WHERE t.key_md5 = 'xxxx' AND t.date_utc = '2023-05-05' AND ki.key_md5 = 'xxxx' AND ki.used < ak.daily_quote
  RETURNING t.used, t.failed, ak.daily_quote;

