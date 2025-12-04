import pandas as pd

count = 0
dial = 50

turns = pd.read_csv("./puzzle1.txt")

for turn in turns["dial"]:
    dir = turn[:1]
    num = int(turn[1:])
    last = dial
    flag = True

    if (dir == "R"): dial += num
    else: dial -= num

    if (abs(dial) > 100): 
        count += abs(dial)//100
        flag = False
    if (dial < 0 and last > 0): count += 1

    #print ("Turn: " + turn + ", Dial Total: " + str(dial))

    dial = dial % 100

    #print("Dial adjusted: " + str(dial))
    
    if (dial == 0 and flag): count += 1
    #print ("Count: " + str(count))

print(count)