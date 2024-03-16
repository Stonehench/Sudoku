from flask import Flask

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
def hello_world():
    return [
        {"username": "Jens", "value": 13, "you": False},
        {"username": "Obamma", "value": 12, "you": False},
        {"username": "Karzten", "value": 9, "you": False},
        {"username": "FreDDie", "value": 5, "you": True},
        {"username": "Heinzz", "value": 3, "you": False},
        {"username": "Ketchup", "value": 2, "you": False},
    ]


@app.route("/login/<user_id>")
def login(user_id: str):
    cursor = conn.cursor()
    cursor.execute("select username from users where user_id = ?", [user_id])

    print(cursor)
    user = cursor.fetchone()[0]
    if user:
        return {"username": user}
    else:
        return {}


@app.route("/register/<user_id>/<username>")
def register(user_id: str, username: str):
    cursor = conn.cursor()
    cursor.execute("insert into users values (?,?)", [user_id, username])
    return {}
