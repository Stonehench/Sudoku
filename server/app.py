from flask import Flask, request

import mariadb
import sys
import uuid

# Connect to MariaDB Platform
try:
    pool = mariadb.ConnectionPool(
      host="jensogkarsten.site",
      port=3306,
      user="02170",
      password="123kage",
      database="Scoreboard",
      pool_name="scoreboard-pool",
      pool_size=20,
      pool_validation_interval=250)

    # conn = mariadb.connect(
    #     user="02170",
    #     password="123kage",
    #     host="jensogkarsten.site",
    #     port=3306,
    #     database="Scoreboard",
    # )
    # conn.autocommit = True
    # conn.auto_reconnect = True
except mariadb.Error as e:
    print(f"Error connecting to MariaDB Platform: {e}")
    sys.exit(1)


app = Flask("Sudoku Scoreboard")


@app.route("/scoreboard")
def scoreboard():
    conn = pool.get_connection()
    cursor = conn.cursor()
    cursor.execute("select * from userscores")

    if "user_id" in request.args:
        req_user_id = request.args["user_id"]
        print(req_user_id)
    else:
        req_user_id = ""

    data = []

    for user_id, username, value, lasth in cursor:
        data.append(
            {
                "username": username, 
                "value": int(value), 
                "you": req_user_id == user_id,
                "lasth": int(lasth) if lasth is not None else 0
            }
        )
    conn.close()
    return data


@app.route("/login", methods=["POST"])
def login():
    username = request.form["username"]
    password = request.form["password"]

    conn = pool.get_connection()
    cursor = conn.cursor()
    cursor.execute(
        "select user_id from users where username = ? and password = ?",
        [username, password],
    )

    user = cursor.fetchone()
    conn.close()
    if user:
        return {"user_id": user[0]}
    else:
        return {}, 404


@app.route("/register", methods=["POST"])
def register():
    user_id = str(uuid.uuid4())
    username = request.form["username"]
    password = request.form["password"]

    conn = pool.get_connection()
    cursor = conn.cursor()
    cursor.execute("insert into users values (?,?,?)", [user_id, username, password])
    
    conn.commit()
    conn.close()
    return {
        "username": username,
        "user_id": user_id,
    }


@app.route("/add_score", methods=["POST"])
def add_score():
    user_id = request.form["user_id"]
    value = request.form["value"]

    conn = pool.get_connection()
    cursor = conn.cursor()
    cursor.execute("insert into scores (user_id, value) values (?,?)", [user_id, value])

    conn.commit()
    conn.close()
    return {}


@app.route("/change_passwd", methods=["POST"])
def change_passw():
    user_id = request.form["user_id"]
    new_passwd = request.form["password"]

    conn = pool.get_connection()
    cursor = conn.cursor()
    cursor.execute(
        "update users set password = ? where user_id = ?", [new_passwd, user_id]
    )
    conn.commit()
    conn.close()
    return {}
