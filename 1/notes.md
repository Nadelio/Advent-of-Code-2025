1. Dial starts at 50
  - constrain dial range between 0 and 99 (inclusively)
2. Read in all lines of input
3. Split the lines into direction and distance, convert to a signed delta
  - 'L' is negative, 'R' is positive
4. Add delta to dial
5. Check if dial > 99 or dial < 1
  - if true, increment zero count
6. Divide dial by 100
  - add remainder to zero count

# 

dial = 99
R2
dial = 101 -> 1
delta = 2
current - last = -98 -> 98
diff = 98
diff + delta > 100 -> inc zero
# 
dial = 50
L4
dial = 46
delta = 4
diff = 46 - 50 -> 4
delta + diff = 8 < 100 -> nop

