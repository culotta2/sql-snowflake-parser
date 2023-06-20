select 
  car,
  SUM(price) as total_price,
  COUNT(car) as total_count
from cars
;
