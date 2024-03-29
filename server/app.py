from flask import Flask, request

import mariadb
import sys
import uuid
import subprocess
import datetime

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
        pool_validation_interval=250,
    )

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


def mk_sudoku(diffculty: str):
    if diffculty == None:
        args = []
    else:
        args = [diffculty]

    output = subprocess.run(
        ["cargo", "run", "--bin", "solver", "--release", "--", "--generate", *args],
        stdout=subprocess.PIPE,
    )

    return output.stdout.decode()


@app.route("/streak", methods=["POST"])
def streak():
    conn = pool.get_connection()
    cursor = conn.cursor()

    cursor.execute(
        "select submittet from scores where user_id = ? order by submittet desc",
        [request.form["user_id"]],
    )

    startdate = datetime.date(2000, 1, 1)

    data = []
    for stamp in cursor:
        datearr = [int(x) for x in str(stamp[0]).split()[0].split("-")]
        cdate = datetime.date(datearr[0], datearr[1], datearr[2])
        datediff = (cdate - startdate).days
        if len(data) == 0 or data[-1] != datediff:
            data.append(datediff)

    todaydiff = (datetime.date.today() - startdate).days
    # todaydiff = 8851

    streak = 0
    if data[0] == todaydiff or data[0] == todaydiff - 1:
        streak = 1
        # Today or yesterday
        for i in range(len(data) - 1):
            if data[i] == data[i + 1] + 1:
                streak += 1
            else:
                break

    conn.commit()
    conn.close()

    return {"streak": streak, "multiplier": 1.1**streak}


@app.route("/daily")
def get_daily():
    conn = pool.get_connection()
    cursor = conn.cursor()
    cursor.execute("select puzzle from DailyChallenges order by dato desc limit 1")

    data = cursor.fetchone()

    if data is None:
        data = mk_sudoku(None)
        cursor.execute("insert into DailyChallenges (puzzle) values (?)", [data])
    else:
        data = data[0]

    conn.commit()
    conn.close()

    return {"puzzle": data}


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
                "lasth": int(lasth) if lasth is not None else 0,
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
    if "daily_dato" in request.form:
        daily_dato = request.form["daily_dato"]
    else:
        daily_dato = None

    conn = pool.get_connection()
    cursor = conn.cursor()
    cursor.execute(
        "insert into scores (user_id, value, dailty_dato) values (?,?,?)",
        [user_id, value, daily_dato],
    )

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
