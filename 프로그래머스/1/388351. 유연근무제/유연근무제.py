def solution(schedules, timelogs, startday):
    weekdays = [1, 2, 3, 4, 5]  # 월(1) ~ 금(5)만 체크
    cnt = 0

    for emp, logs in zip(schedules, timelogs):
        success = True

        for i in range(7):
            day = (startday + i - 1) % 7 + 1  # 요일을 계산 (1~7)

            if day in weekdays:
                limit = emp + 10 if (emp % 100) < 50 else emp + 10
                hour, minute = divmod(emp, 100)
                minute += 10
                if minute >= 60:
                    hour += 1
                    minute -= 60
                limit = hour * 100 + minute

                if logs[i] > limit:
                    success = False
                    break

        if success:
            cnt += 1

    return cnt