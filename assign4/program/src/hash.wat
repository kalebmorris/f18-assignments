(module
  (memory 1)
  (export "mem" (memory 0))

  ;; Stack-based Adler32 hash implementation.
  (func $adler32 (param $address i32) (param $len i32) (result i32)
    (local $a i32) (local $b i32) (local $i i32)

    ;; YOUR CODE GOES HERE
    (i32.const 1)
    (set_local $a)
    (i32.const 0)
    (set_local $b)
    (i32.const 0)
    (set_local $i)
    (block $loop_end
      (loop $loop_start
        (get_local $len)
        (get_local $i)
        (i32.eq)
        (br_if $loop_end)

        (get_local $i)
        (get_local $address)
        (i32.add)
        (i32.load8_u)
        (get_local $a)
        (i32.add)
        (i32.const 65521)
        (i32.rem_u)
        (set_local $a)

        (get_local $a)
        (get_local $b)
        (i32.add)
        (i32.const 65521)
        (i32.rem_u)
        (set_local $b)

        (get_local $i)
        (i32.const 1)
        (i32.add)
        (set_local $i)

        (br $loop_start)))

    (get_local $b)
    (i32.const 16)
    (i32.shl)
    (get_local $a)
    (i32.or))

  (export "adler32" (func $adler32))

  ;; Tree-based Adler32 hash implementation.
  (func $adler32v2 (param $address i32) (param $len i32) (result i32)
    (local $a i32) (local $b i32) (local $i i32)

    ;; YOUR CODE GOES HERE
    (set_local $a (i32.const 1))
    (set_local $b (i32.const 0))
    (set_local $i (i32.const 0))
    (block $loop_end
      (loop $loop_start
        (br_if $loop_end
          (i32.eq
            (get_local $i)
            (get_local $len)))

        (set_local $a 
          (i32.rem_u 
            (i32.add 
              (get_local $a) 
              (i32.load8_u 
                (i32.add 
                  (get_local $i) 
                  (get_local $address)))) 
            (i32.const 65521)))

        (set_local $b
          (i32.rem_u
            (i32.add
              (get_local $b)
              (get_local $a))
            (i32.const 65521)))

        (set_local $i
          (i32.add
            (i32.const 1)
            (get_local $i)))

        (br $loop_start)))

    (i32.or
      (i32.shl
        (get_local $b)
        (i32.const 16))
      (get_local $a)))

  (export "adler32v2" (func $adler32v2))

  ;; Initialize memory allocator. Creates the initial block assuming memory starts with
  ;; 1 page.
  (func $alloc_init
    (i32.store (i32.const 0) (i32.const 65528))
    (i32.store (i32.const 4) (i32.const 1)))
  (export "alloc_init" (func $alloc_init))

  ;; Frees a memory block by setting the free bit to 1.
  (func $free (param $address i32)
    (i32.store
      (i32.sub (get_local $address) (i32.const 4))
      (i32.const 1)))
  (export "free" (func $free))

  (func $alloc (param $len i32) (result i32)
    (local $addr i32) (local $cur_len i32)
    ;; YOUR CODE GOES HERE
    (set_local $addr 
      (i32.const 0))
    (block $loop_end
      (loop $loop_start
        (i32.ge_u
          (get_local $addr)
          (i32.const 65528))
        (if
          (then
            (unreachable)))

        (set_local $cur_len
          (i32.load
            (get_local $addr)))

        (i32.and
          (i32.eq
            (i32.load
              (i32.add
                (get_local $addr)
                (i32.const 4)))
            (i32.const 1))
          (i32.ge_u
            (get_local $cur_len)
            (get_local $len)))
        (if
          (then
            (i32.store
              (i32.add
                (get_local $addr)
                (i32.const 4))
              (i32.const 0))

            (i32.ge_u
              (i32.sub
                (get_local $cur_len)
                (get_local $len))
              (i32.const 8))
            (if
              (then
                (i32.store
                  (i32.add
                    (i32.add
                      (get_local $addr)
                      (get_local $len))
                    (i32.const 8))
                  (i32.sub
                    (get_local $cur_len)
                    (i32.add
                      (get_local $len)
                      (i32.const 8))))
                (i32.store
                  (i32.add
                    (i32.add
                      (get_local $addr)
                      (get_local $len))
                    (i32.const 12))
                  (i32.const 1))
                (i32.store
                  (get_local $addr)
                  (get_local $len))))

            (br $loop_end))
          (else
            (set_local $addr
              (i32.add
                (i32.add
                  (get_local $addr)
                  (get_local $cur_len))
                (i32.const 8)))
            (br $loop_start)))))
    (i32.add
      (get_local $addr)
      (i32.const 8))
  )

  (export "alloc" (func $alloc))
)