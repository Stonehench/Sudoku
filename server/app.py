from flask import Flask

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
