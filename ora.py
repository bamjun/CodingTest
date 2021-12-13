import cx_Oracle

# "USER_ID":"c##ats",
# "PASSWORD":"1234",
# "HOST":"localhost",
# "PORT":"1521",
# "SERVICE_NAME":"orcl"
connection = cx_Oracle.connect('c##ats','1234','localhost:1521/orcl')

cur = connection.cursor()

# sql = """
#    select *
#    from TB_ACCNT where user_id = :bundu1
#    """

sql = """
   select *
   from TB_ACCNT
   """

# cur.execute(sql, bundu1='khy5116')

for name in cur.execute(sql):
   print(name)


cur.close()
