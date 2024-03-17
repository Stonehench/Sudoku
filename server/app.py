from flask import Flask, request

import mariadb
import sys

# Connect to MariaDB Platform
try:
    conn = mariadb.connect(
        user="02170",
        password="123kage",
        host="jensogkarsten.site",
        port=3306,
        database="Scoreboard",
    )
    conn.autocommit = True
except mariadb.Error as e:
    print(f"Error connecting to MariaDB Platform: {e}")
    sys.exit(1)


app = Flask("Sudoku Scoreboard")


@app.route("/scoreboard")
def scoreboard():
    cursor = conn.cursor()
    cursor.execute("select * from userscores")

    if "user_id" in request.args:
        req_user_id = request.args["user_id"]
        print(req_user_id)
    else:
        req_user_id = ""

    data = []

    for user_id, username, value in cursor:
        data.append(
            {"username": username, "value": int(value), "you": req_user_id == user_id}
        )

    return data


@app.route("/login/<user_id>")
def login(user_id: str):
    cursor = conn.cursor()
    cursor.execute("select username from users where user_id = ?", [user_id])

    user = cursor.fetchone()
    if user:
        return {"username": user[0]}
    else:
        return {}


@app.route("/register/<user_id>/<username>")
def register(user_id: str, username: str):
    cursor = conn.cursor()
    cursor.execute("insert into users values (?,?)", [user_id, username])
    return {}


@app.route("/add_score/<user_id>/<value>")
def add_score(user_id: str, value: int):
    cursor = conn.cursor()
    cursor.execute("insert into scores (user_id, value) values (?,?)", [user_id, value])
    return {}
