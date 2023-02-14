--
begin;

create table factbook (
  year int,
  date date,
  shares text,
  trades text,
  dollars text
);

\copy factbook from 'data/factbook.csv' with delimiter E'\t' null '';

alter table factbook
  alter shares type bigint using replace(shares, ',', '')::bigint,
  alter trades type bigint using replace(trades, ',', '')::bigint,
  alter dollars type bigint using substring(replace(dollars, ',', '') from 2)::numeric;

commit;

--
select * from factbook;

\d factbook;

--
\set start '2010-01-01'

select date,
  to_char(shares, '99G999G999G999') as shares,
  to_char(trades, '99G999G999') as trades,
  to_char(dollars, 'L99G999G999G999') as dollars
from factbook
where date >= date :'start' and date < date :'start' + interval '1 month'
order by date;

---
prepare foo as
  select date, shares, trades, dollars from factbook
  where date >= $1::date and date < $1::date + interval '1 month'
  order by date;

execute foo('2010-01-01');

---
\set start '2010-01-01'

select cast(calendar.entry as date) as date,
  coalesce(shares, 0) as shares,
  coalesce(trades, 0) as trades,
  to_char(coalesce(dollars, 0), 'L99G999G999G999') as dollars
from generate_series
  (date :'start', date :'start' + interval '1 month' - interval '1 day', interval '1 day')
  as calendar(entry)
left join factbook on factbook.date = calendar.entry
order by date;
