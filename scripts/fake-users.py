import os
import sqlite3
from faker import Faker
import random
import argparse

# Initialize Faker
fake = Faker()

# Constants
NUMBER_OF_USERS_PER_LETTER = 5


def create_fake_users(db_connection):
    cursor = db_connection.cursor()
    values_users = []
    values_groups = []
    values_user_group_map = []
    user_id = random.randint(2, 1_000_000_000)

    for letter in "ABCDEFGHAIJKLMNOPQRSTUVWXYZ1-":  # From A (0) to Z (25)
        for i in range(NUMBER_OF_USERS_PER_LETTER):
            # Create a unique user name
            user_name = f"{letter} {fake.unique.last_name()}"

            # Generate a random amount of money (0 to 100 euros, in cents)
            user_money = random.randint(0, 10000)  # 0 to 10000 cents

            values_users.append(
                (
                    user_id,
                    user_name,
                    user_money,
                    False,  # is_system_user
                    fake.date_time_this_century(),  # created_at
                    False,  # disabled
                )
            )

            # Add corresponding group values
            group_id = user_id
            values_groups.append((group_id,))
            values_user_group_map.append((group_id, user_id))

            user_id += 1

    # Insert users into Users table
    cursor.executemany(
        "INSERT INTO Users (id, nickname, money, is_system_user, created_at, disabled) VALUES (?, ?, ?, ?, ?, ?)",
        values_users,
    )

    # Insert groups into Groups table
    cursor.executemany("INSERT INTO Groups (id) VALUES (?)", values_groups)

    # Insert into UserGroupMap table
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
    elif os.path.exists("../tmp/db.sqlite"):  # from scripts folder
        path = "../tmp/db.sqlite"
    elif os.path.exists("./tmp/db.sqlite"):  # from project root
        path = "./tmp/db.sqlite"
    else:
        print("Error: Cannot find db file.")
        exit(1)

    with sqlite3.connect(path) as conn:
        create_fake_users(conn)
