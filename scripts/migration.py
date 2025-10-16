#!/usr/bin/env python3
import sqlite3
import argparse
from datetime import timedelta, datetime

USER_ID_OFFSET = 2

USER_AUFLADUNG = 1
USER_SNACKBAR = 0

TIME_DELTA = timedelta(hours=0)


def load_date(date: str):
    format = "%Y-%m-%d %H:%M:%S"

    return (datetime.strptime(date, format) + TIME_DELTA).strftime(format)


def load_backup(file: str):
    old_db = sqlite3.connect(":memory:")

    with open(file) as f:
        db_backup_file: str = f.read()

    if len(db_backup_file) == 0:
        print("Backup file is empty")
        exit(1)

    new_db_backup_file = ""

    for line in db_backup_file.splitlines():
        line = line.strip()
        if (
            line.startswith("KEY")
            or line.startswith("UNIQUE")
            or line.startswith("CONSTRAINT")
            or line.startswith("/")
            or line.startswith("LOCK")
            or line.startswith("UNLOCK")
            or line.startswith("--")
            or line.startswith("DROP TABLE IF EXISTS")
        ):
            if line.endswith(");"):
                line = ");"
            continue

        elif len(line) == 0:
            continue

        elif "ENGINE" in line:
            line = ");"

        elif "AUTO_INCREMENT" in line:
            line = line.replace("AUTO_INCREMENT", "")
            line = line.removesuffix(",")
            line = line.strip()
            line = line + ","

        new_db_backup_file += line + "\n"

    line_count = 0
    in_create_table = False
    split_lines: list[str] = new_db_backup_file.splitlines()
    for line in split_lines:
        if line.startswith("CREATE TABLE"):
            in_create_table = True

        elif line.endswith(");") and in_create_table:
            current_line: str = split_lines[line_count - 1]
            current_line = current_line.removesuffix(",")

            split_lines[line_count - 1] = current_line
            in_create_table = False

        line_count += 1

    new_db_backup_file = "\n".join(split_lines)

    db_backup_file = new_db_backup_file

    chunk = ""
    conn = old_db.cursor()

    for char in db_backup_file:
        chunk += char
        if char == ";":
            try:
                conn.execute(chunk)
            except Exception as e:
                print(f"Failed to run chunk:\n{chunk} \n{e}")
                exit(1)
            else:
                chunk = ""

    conn.close()

    old_db.commit()

    return old_db


def migrate_users(old_db, new_db):
    old_conn = old_db.cursor()
    new_conn = new_db.cursor()
    users = old_conn.execute("select * from user").fetchall()
    values = []

    # print(f"{users}")

    for user in users:
        user_id, user_name, user_money, user_disabled, user_created = (
            user[0],
            user[1],
            user[3],
            user[4],
            user[5],
        )
        values.append(
            (
                user_id + USER_ID_OFFSET,
                user_name,
                user_money,
                False,
                load_date(user_created),
                user_disabled,
            )
        )

    new_conn.executemany(
        "insert into Users(id, nickname, money, is_system_user, created_at, disabled) values (?, ?, ?, ?, ?, ?)",
        values,
    )

    values = []
    for user in users:
        user_id = user[0]
        values.append((user_id + USER_ID_OFFSET,))

    new_conn.executemany("insert into Groups(id) values (?)", values)

    values = []
    for user in users:
        user_id = user[0]
        group_id = user_id
        values.append((group_id + USER_ID_OFFSET, user_id + USER_ID_OFFSET))

    new_conn.executemany("insert into UserGroupMap(gid, uid) values (?, ?)", values)

    new_conn.close()
    old_conn.close()


def migrate_articles(old_db, new_db):
    old_conn = old_db.cursor()
    new_conn = new_db.cursor()

    deactivated_article_ids = set()
    for id in old_conn.execute("select id from article").fetchall():
        deactivated_article_ids.add(id[0])

    articles = old_conn.execute("select * from article where active = 1").fetchall()

    def add_articles(articles):
        values = []

        for article in articles:
            (
                article_id,
                _,
                article_name,
                article_barcode,
                p_article_amount,
                p_article_active,
                _,
                _,
            ) = article

            skip = False
            for added_id, added_name, _ in values:
                if added_name == article_name:
                    skip = True
                    break

            if not skip:
                values.append((article_id, article_name, not p_article_active))

        new_conn.executemany(
            "insert into Articles(id, name, is_disabled) values (?, ?, ?)", values
        )

    add_articles(articles)

    for article in articles:
        article_id, _, _, article_barcode, *rest = article

        if article_barcode is not None:
            new_conn.execute(
                "insert into ArticleBarcodes(article_id, barcode_content) values (?, ?)",
                [article_id, article_barcode],
            )

    article_pre_id_to_current_id = dict()

    def add_article_cost_map(articles):
        for article in articles:
            article_id_new, *rest = article
            prev_amount = None
            previous_articles = []
            while True:
                (
                    article_id,
                    article_pre_id,
                    article_name,
                    article_barcode,
                    article_amount,
                    _,
                    article_created,
                    _,
                ) = article

                # print(f"{article}")

                article_created = load_date(article_created)

                deactivated_article_ids.remove(article_id)

                previous_articles.append(article)

                article_pre_id_to_current_id[article_id] = article_id_new

                if article_pre_id is None:
                    break

                article = old_conn.execute(
                    "select * from article where id = ?", [article_pre_id]
                ).fetchone()

            previous_articles.reverse()
            for article in previous_articles:
                (
                    article_id,
                    article_pre_id,
                    article_name,
                    article_barcode,
                    article_amount,
                    _,
                    article_created,
                    _,
                ) = article
                if prev_amount != article_amount:
                    new_conn.execute(
                        "insert into ArticleCostMap(article_id, cost, effective_since) values (?, ?, ?)",
                        [article_id_new, article_amount, article_created],
                    )
                prev_amount = article_amount

    add_article_cost_map(articles)

    deactivated_articles = []

    for deactivated_article_id in deactivated_article_ids:
        article = old_conn.execute(
            "select * from article where id = ?", [deactivated_article_id]
        ).fetchone()
        deactivated_articles.append(article)

    add_articles(deactivated_articles)

    add_article_cost_map(deactivated_articles)

    old_conn.close()
    new_conn.close()

    return article_pre_id_to_current_id


def migrate_transactions(old_db, new_db, article_pre_id_to_current_id):
    old_conn = old_db.cursor()
    new_conn = new_db.cursor()

    transactions = old_conn.execute("select * from transactions").fetchall()

    for transaction in transactions:
        (
            t_id,
            t_user_id,
            t_article_id,
            t_recipient_t_id,
            t_sender_t_id,
            t_quantity,
            t_comment,
            t_amount,
            t_deleted,
            t_created,
        ) = transaction

        t_created = load_date(t_created)

        if t_article_id is None:
            if t_recipient_t_id is None and t_sender_t_id is None:
                if not t_deleted:
                    snackbar_money = new_conn.execute(
                        f"select money from Users where id = {USER_AUFLADUNG}"
                    ).fetchone()[0]
                    snackbar_money -= t_amount
                    new_conn.execute(
                        f"update Users set money = ? where id = {USER_AUFLADUNG} ",
                        [snackbar_money],
                    )
                if t_amount >= 0:
                    # deposit
                    new_conn.execute(
                        "insert into Transactions(id, sender, receiver, is_undone, money, timestamp) values (?, ?, ?, ?, ?, ?)",
                        [
                            t_id,
                            USER_AUFLADUNG,
                            t_user_id + USER_ID_OFFSET,
                            t_deleted,
                            abs(t_amount),
                            t_created,
                        ],
                    )
                else:
                    # withdraw
                    new_conn.execute(
                        "insert into Transactions(id, sender, receiver, is_undone, money, timestamp) values (?, ?, ?, ?, ?, ?)",
                        [
                            t_id,
                            t_user_id + USER_ID_OFFSET,
                            USER_AUFLADUNG,
                            t_deleted,
                            abs(t_amount),
                            t_created,
                        ],
                    )

            else:
                if t_recipient_t_id is not None:
                    # we (t_id) are sender
                    recipient_transaction = old_conn.execute(
                        "select * from transactions where id = ?", [t_recipient_t_id]
                    ).fetchone()

                    new_conn.execute(
                        "insert into Transactions(sender, receiver, is_undone, description, money, timestamp) values (?, ?, ?, ?, ?, ?)",
                        [
                            t_user_id + USER_ID_OFFSET,
                            recipient_transaction[1] + USER_ID_OFFSET,
                            t_deleted,
                            t_comment,
                            abs(t_amount),
                            t_created,
                        ],
                    )
                else:
                    # already handled with new db schema
                    pass
        else:
            # we bough something
            article_id_new = article_pre_id_to_current_id.get(t_article_id)

            if not t_deleted:
                snackbar_money = new_conn.execute(
                    f"select money from Users where id = {USER_SNACKBAR}"
                ).fetchone()[0]
                snackbar_money += abs(t_amount)
                new_conn.execute(
                    f"update Users set money = ? where id = {USER_SNACKBAR} ",
                    [snackbar_money],
                )

            new_conn.execute(
                "insert into Transactions(sender, receiver, is_undone, t_type_data, money, timestamp) values (?, ?, ?, ?, ?, ?)",
                [
                    t_user_id + USER_ID_OFFSET,
                    USER_SNACKBAR,
                    t_deleted,
                    article_id_new,
                    abs(t_amount),
                    t_created,
                ],
            )


def check_databases(old_db, new_db):
    old_conn = old_db.cursor()
    new_conn = new_db.cursor()

    user_money_dict = dict()

    transactions = old_conn.execute(
        "select * from transactions where deleted = 0"
    ).fetchall()

    for transaction in transactions:
        (
            t_id,
            t_user_id,
            t_article_id,
            t_recipient_t_id,
            t_sender_t_id,
            t_quantity,
            t_comment,
            t_amount,
            t_deleted,
            t_created,
        ) = transaction

        if user_money_dict.get(t_user_id) is None:
            user_money_dict[t_user_id] = 0

        user_money_dict[t_user_id] += t_amount

    users_old = old_conn.execute(
        "select id, balance, name from user where disabled = 0"
    ).fetchall()

    failed = False

    for user in users_old:
        u_id, u_balance, u_name = user

        if user_money_dict.get(u_id, 0) != u_balance:
            print(
                f"Found mismatch in balance for user {u_id}, {u_name}. Should be {user_money_dict.get(u_id, 0)}, but is {u_balance}"
            )
            failed = True

    if failed:
        print("Failed check of old_db, exiting...")
        exit(1)

    user_money_dict = dict()

    transactions = new_conn.execute(
        "select * from Transactions where is_undone = 0"
    ).fetchall()

    for transaction in transactions:
        (
            t_id,
            t_sender_id,
            t_receiver_id,
            t_is_undone,
            t_t_type_data,
            t_money,
            t_description,
            t_timestamp,
        ) = transaction

        if user_money_dict.get(t_sender_id) is None:
            user_money_dict[t_sender_id] = 0

        if user_money_dict.get(t_receiver_id) is None:
            user_money_dict[t_receiver_id] = 0

        user_money_dict[t_sender_id] -= t_money
        user_money_dict[t_receiver_id] += t_money

    users = new_conn.execute(
        "select id, nickname, money from Users where disabled = 0"
    ).fetchall()

    failed = False
    for user in users:
        u_id, u_nickname, u_money = user

        if user_money_dict.get(u_id, 0) != u_money:
            print(
                f"Found mismatch in balance for user {u_id}, {u_nickname}. Should be {user_money_dict.get(u_id, 0)}, but is {u_money}"
            )
            failed = True

    if failed:
        print("Failed check of new_db, exiting...")
        exit(1)

    for user in users:
        for old_user in users_old:
            if old_user[0] + USER_ID_OFFSET == user[0]:
                if old_user[1] != user[2]:
                    failed = True
                    print(
                        f"User Balance differs in old and new database, old: {old_user[1]} -> new: {user[2]}"
                    )

    if failed:
        print("migrating user balances went wrong")
        exit(1)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-f", "--file", required=True, help="The file to load the backup from"
    )
    parser.add_argument(
        "-o", "--out", required=True, help="The path to the resuling sqlite db"
    )

    parser.add_argument(
        "-c",
        "--check",
        help="Check the old database and new database for consistency",
        action="store_true",
    )

    parsed = parser.parse_args()

    old_db = load_backup(parsed.file)
    new_db = sqlite3.connect(parsed.out)

    migrate_users(old_db, new_db)
    pre_id_map = migrate_articles(old_db, new_db)
    migrate_transactions(old_db, new_db, pre_id_map)

    if parsed.check:
        check_databases(old_db, new_db)

    new_db.commit()

    print(f"New database can be found at {parsed.out}")


if __name__ == "__main__":
    main()
