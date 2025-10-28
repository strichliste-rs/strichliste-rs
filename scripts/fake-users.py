# requirements:
# - faker

import os
import sqlite3
from faker import Faker
import random

# Initialize Faker
fake = Faker()

# Constants
USER_ID_OFFSET = 1000  # Change per your requirements
NUMBER_OF_USERS_PER_LETTER = 5


def create_fake_users(db_connection):
    cursor = db_connection.cursor()
    values_users = []
    values_groups = []
    values_user_group_map = []

    for letter in range(26):  # From A (0) to Z (25)
        for i in range(NUMBER_OF_USERS_PER_LETTER):
            # Create a unique user name
            user_name = f"{chr(65 + letter)} {fake.unique.last_name()}"

            # Generate a random amount of money (0 to 100 euros, in cents)
            user_money = random.randint(0, 10000)  # 0 to 10000 cents

            # Create user details, user ID adjusted by USER_ID_OFFSET
            user_id = letter * NUMBER_OF_USERS_PER_LETTER + i + USER_ID_OFFSET
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
    if os.path.exists("../tmp/db.sqlite"):  # from scripts folder
        path = "../tmp/db.sqlite"
    elif os.path.exists("./tmp/db.sqlite"):  # from project root
        path = "./tmp/db.sqlite"
    else:
        print("tmp/db.sqlite does not seem to exits.")
        os.exit()

    with sqlite3.connect(path) as conn:
        create_fake_users(conn)
