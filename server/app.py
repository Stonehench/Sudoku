from flask import Flask, request

import mariadb
import sys
import uuid

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
    conn.auto_reconnect = True
except mariadb.Error as e:
    print(f"Error connecting to MariaDB Platform: {e}")
    sys.exit(1)


app = Flask("Sudoku Scoreboard")


@app.route("/scoreboard")
def scoreboard():
    cursor = conn.cursor()
    cursor.execute("select * from userscores order by value desc")

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


@app.route("/login", methods=["POST"])
def login():
    # user_id = request.form["user_id"]
    username = request.form["username"]
    password = request.form["password"]

    cursor = conn.cursor()
    cursor.execute(
        "select user_id from users where username = ? and password = ?",
        [username, password],
    )

    user = cursor.fetchone()
    if user:
        return {"user_id": user[0]}
    else:
        return {}, 404


@app.route("/register", methods=["POST"])
def register(username: str, password: str):
    user_id = uuid.uuid4()
    username = request.form["username"]
    password = request.form["password"]

    cursor = conn.cursor()
    cursor.execute("insert into users values (?,?,?)", [user_id, username, password])
    return {}


@app.route("/add_score", methods = ["POST"])
def add_score(user_id: str, value: int):
    user_id = request.form["user_id"]
    value = request.form["value"]

    cursor = conn.cursor()
    cursor.execute("insert into scores (user_id, value) values (?,?)", [user_id, value])
    return {}
