import os
import sqlite3
from faker import Faker
import random
import argparse
from datetime import datetime


def adapt_datetime(dt: datetime) -> str:
    """Convert a datetime → ISO‑8601 string for SQLite."""
    return dt.isoformat(sep=" ", timespec="seconds")


def convert_datetime(s: bytes) -> datetime:
    """Convert ISO‑8601 string back → datetime."""
    return datetime.fromisoformat(s.decode())


fake = Faker()
NUMBER_OF_USERS_PER_LETTER = 5


def create_fake_users(db_connection):
    cursor = db_connection.cursor()
    values_users = []
    values_groups = []
    values_user_group_map = []
    user_id = random.randint(2, 1_000_000_000)

    for letter in "ABCDEFGHAIJKLMNOPQRSTUVWXYZ1-":
        for _ in range(NUMBER_OF_USERS_PER_LETTER):
            user_name = f"{letter} {fake.unique.last_name()}"
            user_money = random.randint(0, 10000)
            created_at = fake.date_time_this_century()

            values_users.append(
                (
                    user_id,
                    user_name,
                    user_money,
                    False,
                    created_at,
                    False,
                )
            )

            group_id = user_id
            values_groups.append((group_id,))
            values_user_group_map.append((group_id, user_id))

            user_id += 1

    cursor.executemany(
        "INSERT INTO Users (id, nickname, money, is_system_user, created_at, disabled) "
        "VALUES (?, ?, ?, ?, ?, ?)",
        values_users,
    )
    cursor.executemany("INSERT INTO Groups (id) VALUES (?)", values_groups)
    cursor.executemany(
        "INSERT INTO UserGroupMap (gid, uid) VALUES (?, ?)", values_user_group_map
    )

    db_connection.commit()
    cursor.close()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Generate fake users for the database."
    )
    parser.add_argument("--db", type=str, help="Path to the SQLite database file.")
    args = parser.parse_args()

    if args.db:
        path = args.db
    elif os.path.exists("../tmp/db.sqlite"):
        path = "../tmp/db.sqlite"
    elif os.path.exists("./tmp/db.sqlite"):
        path = "./tmp/db.sqlite"
    else:
        print("Error: Cannot find db file.")
        exit(1)

    sqlite3.register_adapter(datetime, adapt_datetime)
    sqlite3.register_converter("timestamp", convert_datetime)

    with sqlite3.connect(
        path, detect_types=sqlite3.PARSE_DECLTYPES | sqlite3.PARSE_COLNAMES
    ) as conn:
        create_fake_users(conn)
