from flask import Flask

app = Flask("Sudoku Scoreboard")


@app.route("/")
def hello_world():
    return [
        {"username": "jens", "value": 13},
        {"username": "obamma", "value": 12},
        {"username": "karsten", "value": 9},
        {"username": "freddie", "value": 5},
    ]
