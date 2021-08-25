# Axon Contracts

Build contracts:

``` sh
capsule build
```

Run tests:

``` sh
capsule test
```

## Tx Manual

### 1 admin创建Sidechain Config Cell和Side chain State Cell

input ouput

ckb_cell ------->     Sidechain Config Cell(TS模式识别)

null Sidechain State Cell

### 2 Checker抵押Muse Token生成Checker Bond Cell

input ouput

Muse_cell ------->     Checker Bond Cell

### 3 Checker解除Checker Bond Cell退回Muse Token

input ouput

Checker Bond Cell(LS模式识别)   ------->    Muse_cell

### 4 Checker加入侧链

input ouput

Sidechain Config Cell(TS模式识别)   ------->    Sidechain Config Cell

Checker Bond Cell Checker Bond Cell

Checker Info Cell

### 5 Checker退出侧链

input ouput

Sidechain Config Cell(TS模式识别)   ------->    Sidechain Config Cell

Checker Bond Cell Checker Bond Cell

Checker Info Cell

### 6 Collator发布任务

需要支持发布空任务?

Dep:
Sidechain Config Cell

input ouput

Sidechain State Cell(TS模式识别)   ------->    Sidechain State Cell //修改确认,最新高度

Sidechain Bond Cell Sidechain Bond Cell //增加unlock高度

Check Task Cell... //commit_threshold个check task cell

### 7 Check确认任务

Dep:
Sidechain Config Cell

input ouput

Checker Info Cell(TS模式识别)   ------->    Checker Info Cell //mode 改变

Check Task Cell

### 8 Check发起挑战

Dep:
Sidechain Config Cell

input output

Checker Info Cell(TS模式识别)   ------->    Checker Info Cell //mode 改变

Check Task Cell ------->    Check Task Cell... //challenge_threshold个check task cell

### 9 Check响应挑战

Dep:
Sidechain Config Cell

input ouput

Checker Info Cell(TS模式识别)   ------->    Checker Info Cell //mode 改变 Check Task Cell

### 10 Collator确认任务

Dep:
Sidechain Config Cell

input ouput

Sidechain State Cell(TS模式识别)   ------->    Sidechain State Cell
[Checker Info Cell]        ------->    [Checker Info Cell]
Sidechain Fee Cell ------->    Sidechain Fee Cell

### 11 Collator确认挑战

input ouput

Sidechain Config Cell ------->    Sidechain Config Cell//如果可以发起此tx,那么一定有人发起了"假"的挑战,需要取消资格 Sidechain State Cell(TS模式识别)
------->    Sidechain State Cell
[Checker Info Cell]//mode==task-pass ------->    [Checker Info Cell]
[Checker Info Cell]//mode=challenge-pass ------->    [Checker Info Cell]
[Checker Info Cell]//mode=challenge-reject ------->    [Checker Info Cell]
Sidechain Fee Cell ------->    Sidechain Fee Cell

### 12 Collator刷新任务

input ouput
[Check Task Cell](TS模式识别)     ------->    [Check Task Cell]

### 13 Checker提取收益

input ouput Checker Info Cell(TS模式识别)     ------->    Checker Info Cell Muse Cell ------->    Muse Cell Sidechain Fee
Cell ------->    Sidechain Fee Cell

### 14 Collator解押

Dep:
Sidechain State Cell

input ouput Sidechain Bond Cell(LS模式识别)    ------->    Muse Cell
