import pandas as pd

count = 0
dial = 50
last = dial

turns = pd.read_csv("./puzzle1.txt")

for turn in turns["dial"]:
    dir = turn[:1]
    num = int(turn[1:])

    if (dir == "R"): dial += num
    else: dial -= num

    #if (dial <= 0): count += (abs(dial)//100 + 1)
    #if (dial >= 100): count += (dial//100)

    if (abs(dial) > 100): count += abs(dial)//100
    elif (dial < 0 and last > 0): count += 1

    print ("Turn: " + turn + ", Dial: " + str(dial))

    dial = dial % 100
    last = dial
    
    if (dial == 0): count += 1
    print ("Count: " + str(count))

print(count)