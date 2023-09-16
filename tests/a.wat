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
    (local i32 i32 i64 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 512
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
          i32.const 12
          i32.add
          i32.const 20
          i32.const 524289
          call $otxn_field
          i64.const 0
          i64.lt_s
          br_if 0 (;@3;)
          call $ledger_seq
          local.set 3
          local.get 1
          i32.const 12
          i32.add
          i32.const 20
          call $hook_account
          i64.const -1
          i64.le_s
          br_if 1 (;@2;)
          local.get 1
          i32.const 18
          i32.store8 offset=12
          local.get 1
          i32.const 0
          i32.store8 offset=262
          local.get 1
          i32.const 0
          i32.store8 offset=510
          local.get 1
          i32.const 34
          i32.store8 offset=509
          local.get 1
          i32.const 128
          i32.store8 offset=508
          local.get 1
          i32.const 0
          i32.store8 offset=507
          local.get 1
          i32.const 0
          i32.store8 offset=506
          local.get 1
          i32.const 0
          i32.store8 offset=505
          local.get 1
          i32.const 35
          i32.store8 offset=504
          local.get 1
          i32.const 0
          i32.store8 offset=503
          local.get 1
          i32.const 0
          i32.store8 offset=502
          local.get 1
          i32.const 0
          i32.store8 offset=501
          local.get 1
          i32.const 0
          i32.store8 offset=500
          local.get 1
          i32.const 36
          i32.store8 offset=499
          local.get 1
          i32.const 0
          i32.store8 offset=498
          local.get 1
          i32.const 0
          i32.store8 offset=497
          local.get 1
          i32.const 0
          i32.store8 offset=496
          local.get 1
          i32.const 0
          i32.store8 offset=495
          local.get 1
          i32.const 46
          i32.store8 offset=494
          local.get 1
          i32.const 0
          i32.store8 offset=493
          local.get 1
          i32.const 0
          i32.store8 offset=492
          local.get 1
          i32.const 0
          i32.store8 offset=491
          local.get 1
          i32.const 0
          i32.store8 offset=490
          local.get 1
          i32.const 32
          i32.store8 offset=489
          local.get 1
          local.get 3
          i32.wrap_i64
          i32.const 1
          i32.add
          local.tee 2
          i32.const 16
          i32.shr_u
          local.tee 4
          i32.store8 offset=486
          local.get 1
          i32.const 26
          i32.store8 offset=488
          local.get 1
          local.get 2
          i32.const 8
          i32.shr_u
          local.tee 5
          i32.store8 offset=485
          local.get 1
          local.get 2
          i32.const 24
          i32.shr_u
          local.tee 6
          i32.store8 offset=487
          local.get 1
          local.get 2
          i32.store8 offset=484
          local.get 1
          local.get 2
          i32.store8 offset=40
          local.get 1
          local.get 5
          i32.store8 offset=39
          local.get 1
          local.get 4
          i32.store8 offset=38
          local.get 1
          local.get 6
          i32.store8 offset=37
          local.get 1
          i32.const 6688
          i32.store16 offset=35 align=1
          local.get 1
          i64.const 771751936
          i64.store offset=27 align=1
          local.get 1
          i64.const 10133099161592576
          i64.store offset=19 align=1
          local.get 1
          i32.const 32802
          i32.store offset=15 align=1
          local.get 1
          i32.const 0
          i32.store8 offset=14
          local.get 1
          i32.const 18
          i32.store16 offset=12 align=1
          local.get 1
          i32.const 41
          i32.add
          local.get 1
          i32.const 262
          i32.add
          i32.const 219
          call $memcpy
          drop
          local.get 1
          i32.const 262
          i32.add
          i32.const 32
          local.get 1
          i32.const 12
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
