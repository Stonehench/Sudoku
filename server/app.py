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
        database="Scoreboard"

    )
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
