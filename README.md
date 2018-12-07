[![Build status](https://ci.appveyor.com/api/projects/status/durbdaa492i8nah8?svg=true)](https://ci.appveyor.com/project/v-i-k-r-a-m/adtc-kgn)
=======

:exclamation: **For learning purpose only**

# ADTC-Kgn    

## Key Generation    

### Software Editions    
"SG", Single License 1 PC    
"PS", Personal License 3 PCs    
"HM", Home License 10 PCs    
"TM", Team License (> 10 PCs)     
"EP", Enterprise License (>100 PCs)    

### Character mappings    
M = 0    
Q = 1    
S = 2    
Y = 3    
L = 4    
J = 5    
Z = 6    
C = 7    
H = 8    
G = 9    

### Key Construction    
Key length >=19    
Key format = [xxxx-xxxx-xxxx-xxxx] or [xxxx-xxxx-xxxx-xxxx-xxxxxx]    
Key chars = {M, Q, S, Y, L, J, Z, C, H, G}    

Key[0-4] = App Signature = NRSP    
Key[4-1] = "-"    
Key[5] = {M, Q, S, Y, L, J}    
Key[6] = {C, G, H, J, L, M, Q, S, Y, Z}    
Key[7] = {M, Q}    
if Key[7] = "M" then Key[8] = {C, G, H, J, L, Q, S, Y, Z}    
if Key[7] = "Q" then Key[8] = {M, Q}    
Key[9-1] = "-"    
Key[10] = {M, Q, S, Y}     
if Key[10] = "M" then Key[11] = {C, G, H, J, L, Q, S, Y, Z}    
if Key[10] = "Q" & "S" then Key[11] = {C, G, H, J, L, M, Q, S, Y, Z}    
if Key[10] = "Y" then Key[11] = {M, Q}    
Key[12-2] = {"SG", "PS", "HM", "TM", "EP"}    
Key[14, 1] = "-"    
Key[15] = {M, Q, S}    
if Key[15] = "M" then Key[16] = {C, G, H, J, L, M, Q, S, Y, Z}    
if Key[15] = "Q" then Key[16] = {C, G, H, J, L, M, Q, S, Y, Z}    
if Key[15] = "S" then Key[16] = {M, Q, S, Y}    
Key[17] ={M, Q, S, Y, L, J}    
Key[18] = {C, G, H, J, L, M, Q, S, Y, Z}    

Used With Team License and Enterprise License    
Key[19, 1] == "-"    
11>= Key[20-6] <= 999999    

example    
[xxxx-xxxx-xxxx-xxxx]    
[NRSP-QJMC-SCxx-MCSJ] where xx ={"SG", "PS", "HM"}    
if xx ={"TM", "EP"} then [NRSP-QJMC-SCxx-MCSJ-xxxxxx] where xxxxxx is the number of PCs    
also, 11 >= xxxxxx  <= 999999    
Derive xxxxxx from Character mappings    
for example no. of PCs = 500     
So xxxxxx = 000500    
If M = 0 and J = 5, then 000500 = MMMJMM    
[NRSP-QJMC-SCxx-MCSJ-xxxxxx] becomes [NRSP-QJMC-SCxx-MCSJ-MMMJMM]    
