-- 코드를 입력하세요
SELECT *

FROM 
(
SELECT to_char(sales_date, 'YYYY-MM-DD') as SALES_DATE , PRODUCT_ID, USER_ID, SALES_AMOUNT from ONLINE_SALE

    UNION ALL

SELECT to_char(sales_date, 'YYYY-MM-DD') as SALES_DATE , PRODUCT_ID, NULL AS USER_ID, SALES_AMOUNT from offline_sale
    )
    
WHERE sales_date between '2022-03-01' and '2022-03-31'
order by SALES_DATE, PRODUCT_ID, USER_ID