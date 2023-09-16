(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32 i32) (result i64)))
  (type (;2;) (func (result i64)))
  (type (;3;) (func (param i32 i32) (result i64)))
  (type (;4;) (func (param i32 i32 i32 i32) (result i64)))
  (type (;5;) (func (param i32 i32 i64) (result i64)))
  (type (;6;) (func (param i32) (result i64)))
  (type (;7;) (func (param i32 i32 i32) (result i32)))
  (import "env" "_g" (func $_g (type 0)))
  (import "env" "otxn_field" (func $otxn_field (type 1)))
  (import "env" "ledger_seq" (func $ledger_seq (type 2)))
  (import "env" "hook_account" (func $hook_account (type 3)))
  (import "env" "emit" (func $emit (type 4)))
  (import "env" "rollback" (func $rollback (type 5)))
  (import "env" "accept" (func $accept (type 5)))
  (func $cbak (type 6) (param i32) (result i64)
    i64.const 0)
  (func $hook (type 6) (param i32) (result i64)
    (local i32 i32 i64 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 528
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 0
    i32.const 0
    i32.load offset=1048584
    i32.const 1
    i32.add
    local.tee 2
    i32.store offset=1048584
    local.get 2
    i32.const 1
    call $_g
    drop
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 28
          i32.add
          i32.const 20
          i32.const 524289
          call $otxn_field
          i64.const 0
          i64.lt_s
          br_if 0 (;@3;)
          call $ledger_seq
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 28
              i32.add
              i32.const 20
              call $hook_account
              local.tee 4
              i64.const 0
              i64.lt_s
              br_if 0 (;@5;)
              local.get 1
              i32.const 280
              i32.add
              i32.const 2
              i32.add
              local.tee 2
              local.get 1
              i32.const 28
              i32.add
              i32.const 2
              i32.add
              i32.load8_u
              i32.store8
              local.get 1
              local.get 1
              i64.load offset=35 align=1
              i64.store offset=8
              local.get 1
              local.get 1
              i32.const 28
              i32.add
              i32.const 12
              i32.add
              i64.load align=1
              i64.store offset=13 align=1
              local.get 1
              i32.const 280
              i32.add
              i32.const 12
              i32.add
              local.tee 5
              local.get 1
              i64.load offset=13 align=1
              i64.store align=1
              local.get 1
              local.get 1
              i32.load16_u offset=28 align=1
              i32.store16 offset=280
              local.get 1
              local.get 1
              i64.load offset=8
              i64.store offset=287 align=1
              local.get 1
              local.get 1
              i32.load offset=31 align=1
              i32.store offset=283 align=1
              local.get 5
              i32.load8_u
              local.set 5
              local.get 2
              i32.load8_u
              local.set 6
              local.get 1
              i32.load8_u offset=299
              local.set 7
              local.get 1
              i32.load8_u offset=298
              local.set 8
              local.get 1
              i32.load8_u offset=297
              local.set 9
              local.get 1
              i32.load8_u offset=296
              local.set 10
              local.get 1
              i32.load8_u offset=295
              local.set 11
              local.get 1
              i32.load8_u offset=294
              local.set 12
              local.get 1
              i32.load8_u offset=293
              local.set 13
              local.get 1
              i32.load8_u offset=291
              local.set 14
              local.get 1
              i32.load8_u offset=290
              local.set 15
              local.get 1
              i32.load8_u offset=289
              local.set 16
              local.get 1
              i32.load8_u offset=288
              local.set 17
              local.get 1
              i32.load8_u offset=287
              local.set 18
              local.get 1
              i32.load8_u offset=286
              local.set 19
              local.get 1
              i32.load8_u offset=285
              local.set 20
              local.get 1
              i32.load8_u offset=284
              local.set 21
              local.get 1
              i32.load8_u offset=283
              local.set 22
              local.get 1
              i32.load8_u offset=281
              local.set 23
              local.get 1
              i32.load8_u offset=280
              local.set 24
              local.get 1
              i32.const 18
              i32.store8 offset=28
              local.get 1
              i32.const 0
              i32.store8 offset=29
              local.get 1
              i32.const 0
              i32.store8 offset=30
              local.get 1
              i32.const 34
              i32.store8 offset=31
              local.get 1
              i32.const 128
              i32.store8 offset=32
              local.get 1
              i32.const 0
              i32.store8 offset=33
              local.get 1
              i32.const 0
              i32.store8 offset=34
              local.get 1
              i32.const 0
              i32.store8 offset=35
              local.get 1
              i32.const 35
              i32.store8 offset=36
              local.get 1
              i32.const 0
              i32.store8 offset=37
              local.get 1
              i32.const 0
              i32.store8 offset=38
              local.get 1
              i32.const 0
              i32.store8 offset=39
              local.get 1
              i32.const 0
              i32.store8 offset=40
              local.get 1
              i32.const 36
              i32.store8 offset=41
              local.get 1
              i32.const 0
              i32.store8 offset=42
              local.get 1
              i32.const 0
              i32.store8 offset=43
              local.get 1
              i32.const 0
              i32.store8 offset=44
              local.get 1
              i32.const 0
              i32.store8 offset=45
              local.get 1
              i32.const 46
              i32.store8 offset=46
              local.get 1
              i32.const 0
              i32.store8 offset=47
              local.get 1
              i32.const 0
              i32.store8 offset=48
              local.get 1
              i32.const 0
              i32.store8 offset=49
              local.get 1
              i32.const 0
              i32.store8 offset=50
              local.get 1
              i32.const 32
              i32.store8 offset=51
              local.get 1
              local.get 3
              i32.wrap_i64
              local.tee 25
              i32.const 1
              i32.add
              local.tee 2
              i32.const 16
              i32.shr_u
              i32.store8 offset=54
              local.get 1
              i32.const 26
              i32.store8 offset=52
              local.get 1
              local.get 2
              i32.const 8
              i32.shr_u
              i32.store8 offset=55
              local.get 1
              local.get 2
              i32.const 24
              i32.shr_u
              i32.store8 offset=53
              local.get 1
              local.get 2
              i32.store8 offset=56
              local.get 1
              i32.const 32
              i32.store8 offset=57
              local.get 1
              local.get 25
              i32.const 5
              i32.add
              local.tee 2
              i32.const 16
              i32.shr_u
              i32.store8 offset=60
              local.get 1
              i32.const 27
              i32.store8 offset=58
              local.get 1
              local.get 2
              i32.const 8
              i32.shr_u
              i32.store8 offset=61
              local.get 1
              local.get 2
              i32.const 24
              i32.shr_u
              i32.store8 offset=59
              local.get 1
              local.get 2
              i32.store8 offset=62
              local.get 1
              i32.const 97
              i32.store8 offset=63
              local.get 1
              i32.const 64
              i32.store8 offset=64
              local.get 1
              i32.const 0
              i32.store8 offset=65
              local.get 1
              i32.const 0
              i32.store8 offset=66
              local.get 1
              i32.const 0
              i32.store8 offset=67
              local.get 1
              i32.const 0
              i32.store8 offset=68
              local.get 1
              i32.const 0
              i32.store8 offset=69
              local.get 1
              i32.const 3
              i32.store8 offset=70
              local.get 1
              i32.const 232
              i32.store8 offset=71
              local.get 1
              i32.const 104
              i32.store8 offset=72
              local.get 1
              i32.const 64
              i32.store8 offset=73
              local.get 1
              i32.const 0
              i32.store8 offset=74
              local.get 1
              i32.const 0
              i32.store8 offset=75
              local.get 1
              i32.const 0
              i32.store8 offset=76
              local.get 1
              i32.const 0
              i32.store8 offset=77
              local.get 1
              i32.const 0
              i32.store8 offset=78
              local.get 1
              i32.const 0
              i32.store8 offset=79
              local.get 1
              i32.const 0
              i32.store8 offset=80
              local.get 1
              i32.const 115
              i32.store8 offset=81
              local.get 1
              i32.const 33
              i32.store8 offset=82
              local.get 1
              i32.const 0
              i32.store8 offset=83
              local.get 1
              i32.const 0
              i32.store8 offset=84
              local.get 1
              i32.const 0
              i32.store8 offset=85
              local.get 1
              i32.const 0
              i32.store8 offset=86
              local.get 1
              i32.const 0
              i32.store8 offset=87
              local.get 1
              i32.const 0
              i32.store8 offset=88
              local.get 1
              i32.const 0
              i32.store8 offset=89
              local.get 1
              i32.const 0
              i32.store8 offset=90
              local.get 1
              i32.const 0
              i32.store8 offset=91
              local.get 1
              i32.const 0
              i32.store8 offset=92
              local.get 1
              i32.const 0
              i32.store8 offset=93
              local.get 1
              i32.const 0
              i32.store8 offset=94
              local.get 1
              i32.const 0
              i32.store8 offset=95
              local.get 1
              i32.const 0
              i32.store8 offset=96
              local.get 1
              i32.const 0
              i32.store8 offset=97
              local.get 1
              i32.const 0
              i32.store8 offset=98
              local.get 1
              i32.const 0
              i32.store8 offset=99
              local.get 1
              i32.const 0
              i32.store8 offset=100
              local.get 1
              i32.const 0
              i32.store8 offset=101
              local.get 1
              i32.const 0
              i32.store8 offset=102
              local.get 1
              i32.const 0
              i32.store8 offset=103
              local.get 1
              i32.const 0
              i32.store8 offset=104
              local.get 1
              i32.const 0
              i32.store8 offset=105
              local.get 1
              i32.const 0
              i32.store8 offset=106
              local.get 1
              i32.const 0
              i32.store8 offset=107
              local.get 1
              i32.const 0
              i32.store8 offset=108
              local.get 1
              i32.const 0
              i32.store8 offset=109
              local.get 1
              i32.const 0
              i32.store8 offset=110
              local.get 1
              i32.const 0
              i32.store8 offset=111
              local.get 1
              i32.const 0
              i32.store8 offset=112
              local.get 1
              i32.const 0
              i32.store8 offset=113
              local.get 1
              i32.const 0
              i32.store8 offset=114
              local.get 1
              i32.const 0
              i32.store8 offset=115
              local.get 1
              i32.const 88
              i32.store offset=276
              i32.const 0
              i32.const 0
              i32.load offset=1048584
              i32.const 1
              i32.add
              local.tee 2
              i32.store offset=1048584
              local.get 2
              i32.const 55
              call $_g
              drop
              local.get 1
              i32.const 28
              i32.add
              local.get 1
              i32.load offset=276
              i32.add
              i32.const 129
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 1
              i32.add
              i32.const 20
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 2
              i32.add
              local.get 24
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 3
              i32.add
              local.get 23
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 4
              i32.add
              local.get 6
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 5
              i32.add
              local.get 22
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 6
              i32.add
              local.get 21
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 7
              i32.add
              local.get 20
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 8
              i32.add
              local.get 19
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 9
              i32.add
              local.get 18
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 10
              i32.add
              local.get 17
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 11
              i32.add
              local.get 16
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 12
              i32.add
              local.get 15
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 13
              i32.add
              local.get 14
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 14
              i32.add
              local.get 5
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 15
              i32.add
              local.get 13
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 16
              i32.add
              local.get 12
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 17
              i32.add
              local.get 11
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 18
              i32.add
              local.get 10
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 19
              i32.add
              local.get 9
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 20
              i32.add
              local.get 8
              i32.store8
              local.get 1
              i32.load offset=276
              local.get 1
              i32.const 28
              i32.add
              i32.add
              i32.const 21
              i32.add
              local.get 7
              i32.store8
              local.get 1
              local.get 1
              i32.load offset=276
              i32.const 22
              i32.add
              i32.store offset=276
              i32.const 0
              i32.const 0
              i32.load offset=1048584
              i32.const 1
              i32.add
              local.tee 2
              i32.store offset=1048584
              local.get 2
              i32.const 56
              call $_g
              drop
              local.get 1
              i32.const 280
              i32.add
              local.get 1
              i32.const 28
              i32.add
              i32.const 248
              call $memcpy
              drop
              local.get 1
              i32.load8_u offset=343
              local.set 2
              local.get 1
              i32.load8_u offset=342
              local.set 5
              local.get 1
              i32.load8_u offset=341
              local.set 6
              local.get 1
              i32.load8_u offset=340
              local.set 7
              local.get 1
              i32.load8_u offset=339
              local.set 8
              local.get 1
              i32.load8_u offset=338
              local.set 9
              local.get 1
              i32.load8_u offset=337
              local.set 10
              local.get 1
              i32.load8_u offset=336
              local.set 11
              local.get 1
              i32.load8_u offset=335
              local.set 12
              local.get 1
              i32.load8_u offset=334
              local.set 13
              local.get 1
              i32.load8_u offset=333
              local.set 14
              local.get 1
              i32.load8_u offset=332
              local.set 15
              local.get 1
              i32.load8_u offset=331
              local.set 16
              local.get 1
              i32.load8_u offset=330
              local.set 17
              local.get 1
              i32.load8_u offset=329
              local.set 18
              local.get 1
              i32.load8_u offset=328
              local.set 19
              local.get 1
              i32.load8_u offset=327
              local.set 20
              local.get 1
              i32.load8_u offset=326
              local.set 21
              local.get 1
              i32.load8_u offset=325
              local.set 22
              local.get 1
              i32.load8_u offset=324
              local.set 23
              local.get 1
              i32.load8_u offset=323
              local.set 24
              local.get 1
              i32.load8_u offset=322
              local.set 25
              local.get 1
              i32.load8_u offset=321
              local.set 26
              local.get 1
              i32.load8_u offset=320
              local.set 27
              local.get 1
              i32.load8_u offset=319
              local.set 28
              local.get 1
              i32.load8_u offset=318
              local.set 29
              local.get 1
              i32.load8_u offset=317
              local.set 30
              local.get 1
              i32.load8_u offset=316
              local.set 31
              local.get 1
              i32.load8_u offset=315
              local.set 32
              local.get 1
              i32.load8_u offset=314
              local.set 33
              local.get 1
              i32.load8_u offset=313
              local.set 34
              local.get 1
              i32.load8_u offset=312
              local.set 35
              local.get 1
              i32.load8_u offset=311
              local.set 36
              local.get 1
              i32.load8_u offset=310
              local.set 37
              local.get 1
              i32.load8_u offset=309
              local.set 38
              local.get 1
              i32.load8_u offset=308
              local.set 39
              local.get 1
              i32.load8_u offset=307
              local.set 40
              local.get 1
              i32.load8_u offset=306
              local.set 41
              local.get 1
              i32.load8_u offset=305
              local.set 42
              local.get 1
              i32.load8_u offset=304
              local.set 43
              local.get 1
              i32.load8_u offset=303
              local.set 44
              local.get 1
              i32.load8_u offset=302
              local.set 45
              local.get 1
              i32.load8_u offset=301
              local.set 46
              local.get 1
              i32.load8_u offset=300
              local.set 47
              local.get 1
              i32.load8_u offset=299
              local.set 48
              local.get 1
              i32.load8_u offset=298
              local.set 49
              local.get 1
              i32.load8_u offset=297
              local.set 50
              local.get 1
              i32.load8_u offset=296
              local.set 51
              local.get 1
              i32.load8_u offset=295
              local.set 52
              local.get 1
              i32.load8_u offset=294
              local.set 53
              local.get 1
              i32.load8_u offset=293
              local.set 54
              local.get 1
              i32.load8_u offset=292
              local.set 55
              local.get 1
              i32.load8_u offset=291
              local.set 56
              local.get 1
              i32.load8_u offset=290
              local.set 57
              local.get 1
              i32.load8_u offset=289
              local.set 58
              local.get 1
              i32.load8_u offset=288
              local.set 59
              local.get 1
              i32.load8_u offset=287
              local.set 60
              local.get 1
              i32.load8_u offset=286
              local.set 61
              local.get 1
              i32.load8_u offset=285
              local.set 62
              local.get 1
              i32.load8_u offset=284
              local.set 63
              local.get 1
              i32.load8_u offset=283
              local.set 64
              local.get 1
              i32.load8_u offset=282
              local.set 65
              local.get 1
              i32.load8_u offset=281
              local.set 66
              local.get 1
              i32.load8_u offset=280
              local.set 67
              local.get 1
              i32.load8_u offset=407
              local.set 68
              local.get 1
              i32.load8_u offset=406
              local.set 69
              local.get 1
              i32.load8_u offset=405
              local.set 70
              local.get 1
              i32.load8_u offset=404
              local.set 71
              local.get 1
              i32.load8_u offset=403
              local.set 72
              local.get 1
              i32.load8_u offset=402
              local.set 73
              local.get 1
              i32.load8_u offset=401
              local.set 74
              local.get 1
              i32.load8_u offset=400
              local.set 75
              local.get 1
              i32.load8_u offset=399
              local.set 76
              local.get 1
              i32.load8_u offset=398
              local.set 77
              local.get 1
              i32.load8_u offset=397
              local.set 78
              local.get 1
              i32.load8_u offset=396
              local.set 79
              local.get 1
              i32.load8_u offset=395
              local.set 80
              local.get 1
              i32.load8_u offset=394
              local.set 81
              local.get 1
              i32.load8_u offset=393
              local.set 82
              local.get 1
              i32.load8_u offset=392
              local.set 83
              local.get 1
              i32.load8_u offset=391
              local.set 84
              local.get 1
              i32.load8_u offset=390
              local.set 85
              local.get 1
              i32.load8_u offset=389
              local.set 86
              local.get 1
              i32.load8_u offset=388
              local.set 87
              local.get 1
              i32.load8_u offset=387
              local.set 88
              local.get 1
              i32.load8_u offset=386
              local.set 89
              local.get 1
              i32.load8_u offset=385
              local.set 90
              local.get 1
              i32.load8_u offset=384
              local.set 91
              local.get 1
              i32.load8_u offset=383
              local.set 92
              local.get 1
              i32.load8_u offset=382
              local.set 93
              local.get 1
              i32.load8_u offset=381
              local.set 94
              local.get 1
              i32.load8_u offset=380
              local.set 95
              local.get 1
              i32.load8_u offset=379
              local.set 96
              local.get 1
              i32.load8_u offset=378
              local.set 97
              local.get 1
              i32.load8_u offset=377
              local.set 98
              local.get 1
              i32.load8_u offset=376
              local.set 99
              local.get 1
              i32.load8_u offset=375
              local.set 100
              local.get 1
              i32.load8_u offset=374
              local.set 101
              local.get 1
              i32.load8_u offset=373
              local.set 102
              local.get 1
              i32.load8_u offset=372
              local.set 103
              local.get 1
              i32.load8_u offset=371
              local.set 104
              local.get 1
              i32.load8_u offset=370
              local.set 105
              local.get 1
              i32.load8_u offset=369
              local.set 106
              local.get 1
              i32.load8_u offset=368
              local.set 107
              local.get 1
              i32.load8_u offset=367
              local.set 108
              local.get 1
              i32.load8_u offset=366
              local.set 109
              local.get 1
              i32.load8_u offset=365
              local.set 110
              local.get 1
              i32.load8_u offset=364
              local.set 111
              local.get 1
              i32.load8_u offset=363
              local.set 112
              local.get 1
              i32.load8_u offset=362
              local.set 113
              local.get 1
              i32.load8_u offset=361
              local.set 114
              local.get 1
              i32.load8_u offset=360
              local.set 115
              local.get 1
              i32.load8_u offset=359
              local.set 116
              local.get 1
              i32.load8_u offset=358
              local.set 117
              local.get 1
              i32.load8_u offset=357
              local.set 118
              local.get 1
              i32.load8_u offset=356
              local.set 119
              local.get 1
              i32.load8_u offset=355
              local.set 120
              local.get 1
              i32.load8_u offset=354
              local.set 121
              local.get 1
              i32.load8_u offset=353
              local.set 122
              local.get 1
              i32.load8_u offset=352
              local.set 123
              local.get 1
              i32.load8_u offset=351
              local.set 124
              local.get 1
              i32.load8_u offset=350
              local.set 125
              local.get 1
              i32.load8_u offset=349
              local.set 126
              local.get 1
              i32.load8_u offset=348
              local.set 127
              local.get 1
              i32.load8_u offset=347
              local.set 128
              local.get 1
              i32.load8_u offset=346
              local.set 129
              local.get 1
              i32.load8_u offset=345
              local.set 130
              local.get 1
              i32.load8_u offset=344
              local.set 131
              local.get 1
              i32.load8_u offset=471
              local.set 132
              local.get 1
              i32.load8_u offset=470
              local.set 133
              local.get 1
              i32.load8_u offset=469
              local.set 134
              local.get 1
              i32.load8_u offset=468
              local.set 135
              local.get 1
              i32.load8_u offset=467
              local.set 136
              local.get 1
              i32.load8_u offset=466
              local.set 137
              local.get 1
              i32.load8_u offset=465
              local.set 138
              local.get 1
              i32.load8_u offset=464
              local.set 139
              local.get 1
              i32.load8_u offset=463
              local.set 140
              local.get 1
              i32.load8_u offset=462
              local.set 141
              local.get 1
              i32.load8_u offset=461
              local.set 142
              local.get 1
              i32.load8_u offset=460
              local.set 143
              local.get 1
              i32.load8_u offset=459
              local.set 144
              local.get 1
              i32.load8_u offset=458
              local.set 145
              local.get 1
              i32.load8_u offset=457
              local.set 146
              local.get 1
              i32.load8_u offset=456
              local.set 147
              local.get 1
              i32.load8_u offset=455
              local.set 148
              local.get 1
              i32.load8_u offset=454
              local.set 149
              local.get 1
              i32.load8_u offset=453
              local.set 150
              local.get 1
              i32.load8_u offset=452
              local.set 151
              local.get 1
              i32.load8_u offset=451
              local.set 152
              local.get 1
              i32.load8_u offset=450
              local.set 153
              local.get 1
              i32.load8_u offset=449
              local.set 154
              local.get 1
              i32.load8_u offset=448
              local.set 155
              local.get 1
              i32.load8_u offset=447
              local.set 156
              local.get 1
              i32.load8_u offset=446
              local.set 157
              local.get 1
              i32.load8_u offset=445
              local.set 158
              local.get 1
              i32.load8_u offset=444
              local.set 159
              local.get 1
              i32.load8_u offset=443
              local.set 160
              local.get 1
              i32.load8_u offset=442
              local.set 161
              local.get 1
              i32.load8_u offset=441
              local.set 162
              local.get 1
              i32.load8_u offset=440
              local.set 163
              local.get 1
              i32.load8_u offset=439
              local.set 164
              local.get 1
              i32.load8_u offset=438
              local.set 165
              local.get 1
              i32.load8_u offset=437
              local.set 166
              local.get 1
              i32.load8_u offset=436
              local.set 167
              local.get 1
              i32.load8_u offset=435
              local.set 168
              local.get 1
              i32.load8_u offset=434
              local.set 169
              local.get 1
              i32.load8_u offset=433
              local.set 170
              local.get 1
              i32.load8_u offset=432
              local.set 171
              local.get 1
              i32.load8_u offset=431
              local.set 172
              local.get 1
              i32.load8_u offset=430
              local.set 173
              local.get 1
              i32.load8_u offset=429
              local.set 174
              local.get 1
              i32.load8_u offset=428
              local.set 175
              local.get 1
              i32.load8_u offset=427
              local.set 176
              local.get 1
              i32.load8_u offset=426
              local.set 177
              local.get 1
              i32.load8_u offset=425
              local.set 178
              local.get 1
              i32.load8_u offset=424
              local.set 179
              local.get 1
              i32.load8_u offset=423
              local.set 180
              local.get 1
              i32.load8_u offset=422
              local.set 181
              local.get 1
              i32.load8_u offset=421
              local.set 182
              local.get 1
              i32.load8_u offset=420
              local.set 183
              local.get 1
              i32.load8_u offset=419
              local.set 184
              local.get 1
              i32.load8_u offset=418
              local.set 185
              local.get 1
              i32.load8_u offset=417
              local.set 186
              local.get 1
              i32.load8_u offset=416
              local.set 187
              local.get 1
              i32.load8_u offset=415
              local.set 188
              local.get 1
              i32.load8_u offset=414
              local.set 189
              local.get 1
              i32.load8_u offset=413
              local.set 190
              local.get 1
              i32.load8_u offset=412
              local.set 191
              local.get 1
              i32.load8_u offset=411
              local.set 192
              local.get 1
              i32.load8_u offset=410
              local.set 193
              local.get 1
              i32.load8_u offset=409
              local.set 194
              local.get 1
              i32.load8_u offset=408
              local.set 195
              local.get 1
              i32.load8_u offset=527
              local.set 196
              local.get 1
              i32.load8_u offset=526
              local.set 197
              local.get 1
              i32.load8_u offset=525
              local.set 198
              local.get 1
              i32.load8_u offset=524
              local.set 199
              local.get 1
              i32.load8_u offset=523
              local.set 200
              local.get 1
              i32.load8_u offset=522
              local.set 201
              local.get 1
              i32.load8_u offset=521
              local.set 202
              local.get 1
              i32.load8_u offset=520
              local.set 203
              local.get 1
              i32.load8_u offset=519
              local.set 204
              local.get 1
              i32.load8_u offset=518
              local.set 205
              local.get 1
              i32.load8_u offset=517
              local.set 206
              local.get 1
              i32.load8_u offset=516
              local.set 207
              local.get 1
              i32.load8_u offset=515
              local.set 208
              local.get 1
              i32.load8_u offset=514
              local.set 209
              local.get 1
              i32.load8_u offset=513
              local.set 210
              local.get 1
              i32.load8_u offset=512
              local.set 211
              local.get 1
              i32.load8_u offset=511
              local.set 212
              local.get 1
              i32.load8_u offset=510
              local.set 213
              local.get 1
              i32.load8_u offset=509
              local.set 214
              local.get 1
              i32.load8_u offset=508
              local.set 215
              local.get 1
              i32.load8_u offset=507
              local.set 216
              local.get 1
              i32.load8_u offset=506
              local.set 217
              local.get 1
              i32.load8_u offset=505
              local.set 218
              local.get 1
              i32.load8_u offset=504
              local.set 219
              local.get 1
              i32.load8_u offset=503
              local.set 220
              local.get 1
              i32.load8_u offset=502
              local.set 221
              local.get 1
              i32.load8_u offset=501
              local.set 222
              local.get 1
              i32.load8_u offset=500
              local.set 223
              local.get 1
              i32.load8_u offset=499
              local.set 224
              local.get 1
              i32.load8_u offset=498
              local.set 225
              local.get 1
              i32.load8_u offset=497
              local.set 226
              local.get 1
              i32.load8_u offset=496
              local.set 227
              local.get 1
              i32.load8_u offset=495
              local.set 228
              local.get 1
              i32.load8_u offset=494
              local.set 229
              local.get 1
              i32.load8_u offset=493
              local.set 230
              local.get 1
              i32.load8_u offset=492
              local.set 231
              local.get 1
              i32.load8_u offset=491
              local.set 232
              local.get 1
              i32.load8_u offset=490
              local.set 233
              local.get 1
              i32.load8_u offset=489
              local.set 234
              local.get 1
              i32.load8_u offset=488
              local.set 235
              local.get 1
              i32.load8_u offset=487
              local.set 236
              local.get 1
              i32.load8_u offset=486
              local.set 237
              local.get 1
              i32.load8_u offset=485
              local.set 238
              local.get 1
              i32.load8_u offset=484
              local.set 239
              local.get 1
              i32.load8_u offset=483
              local.set 240
              local.get 1
              i32.load8_u offset=482
              local.set 241
              local.get 1
              i32.load8_u offset=481
              local.set 242
              local.get 1
              i32.load8_u offset=480
              local.set 243
              local.get 1
              i32.load8_u offset=479
              local.set 244
              local.get 1
              i32.load8_u offset=478
              local.set 245
              local.get 1
              i32.load8_u offset=477
              local.set 246
              local.get 1
              i32.load8_u offset=476
              local.set 247
              local.get 1
              i32.load8_u offset=475
              local.set 248
              local.get 1
              i32.load8_u offset=474
              local.set 249
              local.get 1
              i32.load8_u offset=473
              local.set 250
              local.get 1
              i32.load8_u offset=472
              local.set 251
              br 1 (;@4;)
            end
            local.get 4
            i64.const 24
            i64.shr_u
            i32.wrap_i64
            local.set 61
            local.get 4
            i64.const 16
            i64.shr_u
            i32.wrap_i64
            local.set 62
            local.get 4
            i64.const 8
            i64.shr_u
            i32.wrap_i64
            local.set 63
            local.get 4
            i32.wrap_i64
            local.set 64
          end
          local.get 4
          i64.const -1
          i64.le_s
          br_if 1 (;@2;)
          local.get 1
          local.get 196
          i32.store8 offset=275
          local.get 1
          local.get 197
          i32.store8 offset=274
          local.get 1
          local.get 198
          i32.store8 offset=273
          local.get 1
          local.get 199
          i32.store8 offset=272
          local.get 1
          local.get 200
          i32.store8 offset=271
          local.get 1
          local.get 201
          i32.store8 offset=270
          local.get 1
          local.get 202
          i32.store8 offset=269
          local.get 1
          local.get 203
          i32.store8 offset=268
          local.get 1
          local.get 204
          i32.store8 offset=267
          local.get 1
          local.get 205
          i32.store8 offset=266
          local.get 1
          local.get 206
          i32.store8 offset=265
          local.get 1
          local.get 207
          i32.store8 offset=264
          local.get 1
          local.get 208
          i32.store8 offset=263
          local.get 1
          local.get 209
          i32.store8 offset=262
          local.get 1
          local.get 210
          i32.store8 offset=261
          local.get 1
          local.get 211
          i32.store8 offset=260
          local.get 1
          local.get 212
          i32.store8 offset=259
          local.get 1
          local.get 213
          i32.store8 offset=258
          local.get 1
          local.get 214
          i32.store8 offset=257
          local.get 1
          local.get 215
          i32.store8 offset=256
          local.get 1
          local.get 216
          i32.store8 offset=255
          local.get 1
          local.get 217
          i32.store8 offset=254
          local.get 1
          local.get 218
          i32.store8 offset=253
          local.get 1
          local.get 219
          i32.store8 offset=252
          local.get 1
          local.get 220
          i32.store8 offset=251
          local.get 1
          local.get 221
          i32.store8 offset=250
          local.get 1
          local.get 222
          i32.store8 offset=249
          local.get 1
          local.get 223
          i32.store8 offset=248
          local.get 1
          local.get 224
          i32.store8 offset=247
          local.get 1
          local.get 225
          i32.store8 offset=246
          local.get 1
          local.get 226
          i32.store8 offset=245
          local.get 1
          local.get 227
          i32.store8 offset=244
          local.get 1
          local.get 228
          i32.store8 offset=243
          local.get 1
          local.get 229
          i32.store8 offset=242
          local.get 1
          local.get 230
          i32.store8 offset=241
          local.get 1
          local.get 231
          i32.store8 offset=240
          local.get 1
          local.get 232
          i32.store8 offset=239
          local.get 1
          local.get 233
          i32.store8 offset=238
          local.get 1
          local.get 234
          i32.store8 offset=237
          local.get 1
          local.get 235
          i32.store8 offset=236
          local.get 1
          local.get 236
          i32.store8 offset=235
          local.get 1
          local.get 237
          i32.store8 offset=234
          local.get 1
          local.get 238
          i32.store8 offset=233
          local.get 1
          local.get 239
          i32.store8 offset=232
          local.get 1
          local.get 240
          i32.store8 offset=231
          local.get 1
          local.get 241
          i32.store8 offset=230
          local.get 1
          local.get 242
          i32.store8 offset=229
          local.get 1
          local.get 243
          i32.store8 offset=228
          local.get 1
          local.get 244
          i32.store8 offset=227
          local.get 1
          local.get 245
          i32.store8 offset=226
          local.get 1
          local.get 246
          i32.store8 offset=225
          local.get 1
          local.get 247
          i32.store8 offset=224
          local.get 1
          local.get 248
          i32.store8 offset=223
          local.get 1
          local.get 249
          i32.store8 offset=222
          local.get 1
          local.get 250
          i32.store8 offset=221
          local.get 1
          local.get 251
          i32.store8 offset=220
          local.get 1
          local.get 132
          i32.store8 offset=219
          local.get 1
          local.get 133
          i32.store8 offset=218
          local.get 1
          local.get 134
          i32.store8 offset=217
          local.get 1
          local.get 135
          i32.store8 offset=216
          local.get 1
          local.get 136
          i32.store8 offset=215
          local.get 1
          local.get 137
          i32.store8 offset=214
          local.get 1
          local.get 138
          i32.store8 offset=213
          local.get 1
          local.get 139
          i32.store8 offset=212
          local.get 1
          local.get 140
          i32.store8 offset=211
          local.get 1
          local.get 141
          i32.store8 offset=210
          local.get 1
          local.get 142
          i32.store8 offset=209
          local.get 1
          local.get 143
          i32.store8 offset=208
          local.get 1
          local.get 144
          i32.store8 offset=207
          local.get 1
          local.get 145
          i32.store8 offset=206
          local.get 1
          local.get 146
          i32.store8 offset=205
          local.get 1
          local.get 147
          i32.store8 offset=204
          local.get 1
          local.get 148
          i32.store8 offset=203
          local.get 1
          local.get 149
          i32.store8 offset=202
          local.get 1
          local.get 150
          i32.store8 offset=201
          local.get 1
          local.get 151
          i32.store8 offset=200
          local.get 1
          local.get 152
          i32.store8 offset=199
          local.get 1
          local.get 153
          i32.store8 offset=198
          local.get 1
          local.get 154
          i32.store8 offset=197
          local.get 1
          local.get 155
          i32.store8 offset=196
          local.get 1
          local.get 156
          i32.store8 offset=195
          local.get 1
          local.get 157
          i32.store8 offset=194
          local.get 1
          local.get 158
          i32.store8 offset=193
          local.get 1
          local.get 159
          i32.store8 offset=192
          local.get 1
          local.get 160
          i32.store8 offset=191
          local.get 1
          local.get 161
          i32.store8 offset=190
          local.get 1
          local.get 162
          i32.store8 offset=189
          local.get 1
          local.get 163
          i32.store8 offset=188
          local.get 1
          local.get 164
          i32.store8 offset=187
          local.get 1
          local.get 165
          i32.store8 offset=186
          local.get 1
          local.get 166
          i32.store8 offset=185
          local.get 1
          local.get 167
          i32.store8 offset=184
          local.get 1
          local.get 168
          i32.store8 offset=183
          local.get 1
          local.get 169
          i32.store8 offset=182
          local.get 1
          local.get 170
          i32.store8 offset=181
          local.get 1
          local.get 171
          i32.store8 offset=180
          local.get 1
          local.get 172
          i32.store8 offset=179
          local.get 1
          local.get 173
          i32.store8 offset=178
          local.get 1
          local.get 174
          i32.store8 offset=177
          local.get 1
          local.get 175
          i32.store8 offset=176
          local.get 1
          local.get 176
          i32.store8 offset=175
          local.get 1
          local.get 177
          i32.store8 offset=174
          local.get 1
          local.get 178
          i32.store8 offset=173
          local.get 1
          local.get 179
          i32.store8 offset=172
          local.get 1
          local.get 180
          i32.store8 offset=171
          local.get 1
          local.get 181
          i32.store8 offset=170
          local.get 1
          local.get 182
          i32.store8 offset=169
          local.get 1
          local.get 183
          i32.store8 offset=168
          local.get 1
          local.get 184
          i32.store8 offset=167
          local.get 1
          local.get 185
          i32.store8 offset=166
          local.get 1
          local.get 186
          i32.store8 offset=165
          local.get 1
          local.get 187
          i32.store8 offset=164
          local.get 1
          local.get 188
          i32.store8 offset=163
          local.get 1
          local.get 189
          i32.store8 offset=162
          local.get 1
          local.get 190
          i32.store8 offset=161
          local.get 1
          local.get 191
          i32.store8 offset=160
          local.get 1
          local.get 192
          i32.store8 offset=159
          local.get 1
          local.get 193
          i32.store8 offset=158
          local.get 1
          local.get 194
          i32.store8 offset=157
          local.get 1
          local.get 195
          i32.store8 offset=156
          local.get 1
          local.get 68
          i32.store8 offset=155
          local.get 1
          local.get 69
          i32.store8 offset=154
          local.get 1
          local.get 70
          i32.store8 offset=153
          local.get 1
          local.get 71
          i32.store8 offset=152
          local.get 1
          local.get 72
          i32.store8 offset=151
          local.get 1
          local.get 73
          i32.store8 offset=150
          local.get 1
          local.get 74
          i32.store8 offset=149
          local.get 1
          local.get 75
          i32.store8 offset=148
          local.get 1
          local.get 76
          i32.store8 offset=147
          local.get 1
          local.get 77
          i32.store8 offset=146
          local.get 1
          local.get 78
          i32.store8 offset=145
          local.get 1
          local.get 79
          i32.store8 offset=144
          local.get 1
          local.get 80
          i32.store8 offset=143
          local.get 1
          local.get 81
          i32.store8 offset=142
          local.get 1
          local.get 82
          i32.store8 offset=141
          local.get 1
          local.get 83
          i32.store8 offset=140
          local.get 1
          local.get 84
          i32.store8 offset=139
          local.get 1
          local.get 85
          i32.store8 offset=138
          local.get 1
          local.get 86
          i32.store8 offset=137
          local.get 1
          local.get 87
          i32.store8 offset=136
          local.get 1
          local.get 88
          i32.store8 offset=135
          local.get 1
          local.get 89
          i32.store8 offset=134
          local.get 1
          local.get 90
          i32.store8 offset=133
          local.get 1
          local.get 91
          i32.store8 offset=132
          local.get 1
          local.get 92
          i32.store8 offset=131
          local.get 1
          local.get 93
          i32.store8 offset=130
          local.get 1
          local.get 94
          i32.store8 offset=129
          local.get 1
          local.get 95
          i32.store8 offset=128
          local.get 1
          local.get 96
          i32.store8 offset=127
          local.get 1
          local.get 97
          i32.store8 offset=126
          local.get 1
          local.get 98
          i32.store8 offset=125
          local.get 1
          local.get 99
          i32.store8 offset=124
          local.get 1
          local.get 100
          i32.store8 offset=123
          local.get 1
          local.get 101
          i32.store8 offset=122
          local.get 1
          local.get 102
          i32.store8 offset=121
          local.get 1
          local.get 103
          i32.store8 offset=120
          local.get 1
          local.get 104
          i32.store8 offset=119
          local.get 1
          local.get 105
          i32.store8 offset=118
          local.get 1
          local.get 106
          i32.store8 offset=117
          local.get 1
          local.get 107
          i32.store8 offset=116
          local.get 1
          local.get 108
          i32.store8 offset=115
          local.get 1
          local.get 109
          i32.store8 offset=114
          local.get 1
          local.get 110
          i32.store8 offset=113
          local.get 1
          local.get 111
          i32.store8 offset=112
          local.get 1
          local.get 112
          i32.store8 offset=111
          local.get 1
          local.get 113
          i32.store8 offset=110
          local.get 1
          local.get 114
          i32.store8 offset=109
          local.get 1
          local.get 115
          i32.store8 offset=108
          local.get 1
          local.get 116
          i32.store8 offset=107
          local.get 1
          local.get 117
          i32.store8 offset=106
          local.get 1
          local.get 118
          i32.store8 offset=105
          local.get 1
          local.get 119
          i32.store8 offset=104
          local.get 1
          local.get 120
          i32.store8 offset=103
          local.get 1
          local.get 121
          i32.store8 offset=102
          local.get 1
          local.get 122
          i32.store8 offset=101
          local.get 1
          local.get 123
          i32.store8 offset=100
          local.get 1
          local.get 124
          i32.store8 offset=99
          local.get 1
          local.get 125
          i32.store8 offset=98
          local.get 1
          local.get 126
          i32.store8 offset=97
          local.get 1
          local.get 127
          i32.store8 offset=96
          local.get 1
          local.get 128
          i32.store8 offset=95
          local.get 1
          local.get 129
          i32.store8 offset=94
          local.get 1
          local.get 130
          i32.store8 offset=93
          local.get 1
          local.get 131
          i32.store8 offset=92
          local.get 1
          local.get 2
          i32.store8 offset=91
          local.get 1
          local.get 5
          i32.store8 offset=90
          local.get 1
          local.get 6
          i32.store8 offset=89
          local.get 1
          local.get 7
          i32.store8 offset=88
          local.get 1
          local.get 8
          i32.store8 offset=87
          local.get 1
          local.get 9
          i32.store8 offset=86
          local.get 1
          local.get 10
          i32.store8 offset=85
          local.get 1
          local.get 11
          i32.store8 offset=84
          local.get 1
          local.get 12
          i32.store8 offset=83
          local.get 1
          local.get 13
          i32.store8 offset=82
          local.get 1
          local.get 14
          i32.store8 offset=81
          local.get 1
          local.get 15
          i32.store8 offset=80
          local.get 1
          local.get 16
          i32.store8 offset=79
          local.get 1
          local.get 17
          i32.store8 offset=78
          local.get 1
          local.get 18
          i32.store8 offset=77
          local.get 1
          local.get 19
          i32.store8 offset=76
          local.get 1
          local.get 20
          i32.store8 offset=75
          local.get 1
          local.get 21
          i32.store8 offset=74
          local.get 1
          local.get 22
          i32.store8 offset=73
          local.get 1
          local.get 23
          i32.store8 offset=72
          local.get 1
          local.get 24
          i32.store8 offset=71
          local.get 1
          local.get 25
          i32.store8 offset=70
          local.get 1
          local.get 26
          i32.store8 offset=69
          local.get 1
          local.get 27
          i32.store8 offset=68
          local.get 1
          local.get 28
          i32.store8 offset=67
          local.get 1
          local.get 29
          i32.store8 offset=66
          local.get 1
          local.get 30
          i32.store8 offset=65
          local.get 1
          local.get 31
          i32.store8 offset=64
          local.get 1
          local.get 32
          i32.store8 offset=63
          local.get 1
          local.get 33
          i32.store8 offset=62
          local.get 1
          local.get 34
          i32.store8 offset=61
          local.get 1
          local.get 35
          i32.store8 offset=60
          local.get 1
          local.get 36
          i32.store8 offset=59
          local.get 1
          local.get 37
          i32.store8 offset=58
          local.get 1
          local.get 38
          i32.store8 offset=57
          local.get 1
          local.get 39
          i32.store8 offset=56
          local.get 1
          local.get 40
          i32.store8 offset=55
          local.get 1
          local.get 41
          i32.store8 offset=54
          local.get 1
          local.get 42
          i32.store8 offset=53
          local.get 1
          local.get 43
          i32.store8 offset=52
          local.get 1
          local.get 44
          i32.store8 offset=51
          local.get 1
          local.get 45
          i32.store8 offset=50
          local.get 1
          local.get 46
          i32.store8 offset=49
          local.get 1
          local.get 47
          i32.store8 offset=48
          local.get 1
          local.get 48
          i32.store8 offset=47
          local.get 1
          local.get 49
          i32.store8 offset=46
          local.get 1
          local.get 50
          i32.store8 offset=45
          local.get 1
          local.get 51
          i32.store8 offset=44
          local.get 1
          local.get 52
          i32.store8 offset=43
          local.get 1
          local.get 53
          i32.store8 offset=42
          local.get 1
          local.get 54
          i32.store8 offset=41
          local.get 1
          local.get 55
          i32.store8 offset=40
          local.get 1
          local.get 56
          i32.store8 offset=39
          local.get 1
          local.get 57
          i32.store8 offset=38
          local.get 1
          local.get 58
          i32.store8 offset=37
          local.get 1
          local.get 59
          i32.store8 offset=36
          local.get 1
          local.get 60
          i32.store8 offset=35
          local.get 1
          local.get 65
          i32.store8 offset=30
          local.get 1
          local.get 66
          i32.store8 offset=29
          local.get 1
          local.get 67
          i32.store8 offset=28
          local.get 1
          local.get 61
          i32.const 24
          i32.shl
          local.get 62
          i32.const 255
          i32.and
          i32.const 16
          i32.shl
          i32.or
          local.get 63
          i32.const 255
          i32.and
          i32.const 8
          i32.shl
          i32.or
          local.get 64
          i32.const 255
          i32.and
          i32.or
          i32.store offset=31 align=1
          local.get 1
          i32.const 280
          i32.add
          i32.const 32
          local.get 1
          i32.const 28
          i32.add
          i32.const 248
          call $emit
          i64.const -1
          i64.gt_s
          br_if 2 (;@1;)
          i32.const 1048576
          i32.const 5
          i64.const 489
          call $rollback
          drop
          unreachable
        end
        i32.const 1048576
        i32.const 5
        i64.const 489
        call $rollback
        drop
        unreachable
      end
      i32.const 1048576
      i32.const 5
      i64.const 489
      call $rollback
      drop
      unreachable
    end
    i32.const 1048576
    i32.const 0
    i64.const 0
    call $accept
    drop
    unreachable)
  (func $_ZN17compiler_builtins3mem6memcpy17hd25e15df6c990a04E (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.set 5
      block  ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        local.get 1
        local.set 6
        loop  ;; label = @3
          local.get 3
          local.get 6
          i32.load8_u
          i32.store8
          local.get 6
          i32.const 1
          i32.add
          local.set 6
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      local.tee 7
      i32.const -4
      i32.and
      local.tee 8
      i32.add
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 4
          i32.add
          local.tee 9
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const 1
          i32.lt_s
          br_if 1 (;@2;)
          local.get 9
          i32.const 3
          i32.shl
          local.tee 6
          i32.const 24
          i32.and
          local.set 2
          local.get 9
          i32.const -4
          i32.and
          local.tee 10
          i32.const 4
          i32.add
          local.set 1
          i32.const 0
          local.get 6
          i32.sub
          i32.const 24
          i32.and
          local.set 4
          local.get 10
          i32.load
          local.set 6
          loop  ;; label = @4
            local.get 5
            local.get 6
            local.get 2
            i32.shr_u
            local.get 1
            i32.load
            local.tee 6
            local.get 4
            i32.shl
            i32.or
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 3
            i32.lt_u
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 8
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 9
        local.set 1
        loop  ;; label = @3
          local.get 5
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 7
      i32.const 3
      i32.and
      local.set 2
      local.get 9
      local.get 8
      i32.add
      local.set 1
    end
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      i32.add
      local.set 5
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 5
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memcpy (type 7) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN17compiler_builtins3mem6memcpy17hd25e15df6c990a04E)
  (memory (;0;) 17)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048588))
  (global (;2;) i32 (i32.const 1048592))
  (export "memory" (memory 0))
  (export "cbak" (func $cbak))
  (export "hook" (func $hook))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data $.rodata (i32.const 1048576) "error"))
