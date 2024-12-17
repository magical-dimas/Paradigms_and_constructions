import math
import sys

def ans (dis, a, b):
    l = []
    if(dis < 0):
        return l
    if(dis == 0):
        if (-b/(2*a) > 0):
            l.append(-math.sqrt(-b/(2*a))) 
            l.append(math.sqrt(-b/(2*a)))
            return l
    if ((-b-math.sqrt(dis))/(2*a) > 0):
        l.append(-math.sqrt((-b-math.sqrt(dis))/(2*a))) 
        l.append(math.sqrt((-b-math.sqrt(dis))/(2*a)))
    if ((-b+math.sqrt(dis))/(2*a) > 0):
        l.append(-math.sqrt((-b+math.sqrt(dis))/(2*a))) 
        l.append(math.sqrt((-b+math.sqrt(dis))/(2*a)))
    return l

def enter():
    coef = 0
    flag = True
    while(flag):
        try:
            coef = float(input())
            flag = False
        except:
            print("Incorrect input")
    return coef

if (__name__ == "__main__" and len(sys.argv)==4):
    try:
        a = float(sys.argv[1])
        b = float(sys.argv[2])
        c = float(sys.argv[3])
    except:
        a = enter()
        b = enter()
        c = enter()
else:
    a = enter()
    b = enter()
    c = enter()
dis = pow(abs(b), 2) - 4*a*c
res = ans(dis, a, b)
res.sort()
print("dis|roots")
print(dis, res)