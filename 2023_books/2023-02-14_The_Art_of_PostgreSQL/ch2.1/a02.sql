---
create table misc (
  date date,
  name text,
  dollars bigint
);

insert into misc values
  ('2023-01-01', 'rover', 11),
  ('2023-01-01', 'rover', 0),
  ('2023-01-02', 'alice', 7),
  ('2023-01-02', 'alice', 21),
  ('2023-01-02', 'alice', 11),
  ('2023-01-02', 'rover', 8),
  ('2023-01-02', 'bob', 13),
  ('2023-01-03', 'bob', 8),
  ('2023-01-03', 'alice', 8),
  ('2023-01-03', 'alice', 18);

---
select name, avg(dollars) from misc group by name;

/* output
 name  |         avg         
-------+---------------------
 bob   | 10.5000000000000000
 rover |  6.3333333333333333
 alice | 13.0000000000000000
*/

select
  date, name, dollars,
  avg(dollars) over(partition by name) as avg_dollars
from misc order by date, name;

/* output
    date    | name  | dollars |     avg_dollars     
------------+-------+---------+---------------------
 2023-01-01 | rover |      11 |  6.3333333333333333
 2023-01-01 | rover |       0 |  6.3333333333333333
 2023-01-02 | alice |       7 | 13.0000000000000000
 2023-01-02 | alice |      21 | 13.0000000000000000
 2023-01-02 | alice |      11 | 13.0000000000000000
 2023-01-02 | bob   |      13 | 10.5000000000000000
 2023-01-02 | rover |       8 |  6.3333333333333333
 2023-01-03 | alice |      18 | 13.0000000000000000
 2023-01-03 | alice |       8 | 13.0000000000000000
 2023-01-03 | bob   |       8 | 10.5000000000000000
*/

---
select date, avg(dollars) from misc group by date;

/* output
    date    |         avg         
------------+---------------------
 2023-01-01 |  5.5000000000000000
 2023-01-02 | 12.0000000000000000
 2023-01-03 | 11.3333333333333333
*/

select
  date, name, dollars,
  avg(dollars) over(partition by date) as avg_dollars
from misc order by date, name;

/* output
    date    | name  | dollars |     avg_dollars     
------------+-------+---------+---------------------
 2023-01-01 | rover |       0 |  5.5000000000000000
 2023-01-01 | rover |      11 |  5.5000000000000000
 2023-01-02 | alice |       7 | 12.0000000000000000
 2023-01-02 | alice |      21 | 12.0000000000000000
 2023-01-02 | alice |      11 | 12.0000000000000000
 2023-01-02 | bob   |      13 | 12.0000000000000000
 2023-01-02 | rover |       8 | 12.0000000000000000
 2023-01-03 | alice |       8 | 11.3333333333333333
 2023-01-03 | alice |      18 | 11.3333333333333333
 2023-01-03 | bob   |       8 | 11.3333333333333333
*/
