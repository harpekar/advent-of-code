#!/usr/bin/python3

from itertools import cycle

f = open("./input.txt", "r")

intcode = f.read()

f.close()

int_codes = intcode[:-1].split(",") #Remove newline character and convert file into an array

int_codes = list(map(int,int_codes)) #Map list of strings to ints

int_cycle = cycle(int_codes) 

while (1):

    opcode, first_ind, second_ind, res_index = [next(int_cycle) for i in range(4)]

    first_num, second_num = int_codes[first_ind], int_codes[second_ind]

    if (opcode == 99):
        break
    elif (opcode == 1):
        int_codes[res_index] = first_num + second_num
    else:
        int_codes[res_index] = first_num * second_num 


print ("Program finished.  \n Position 0 is: ", int_codes[0])
