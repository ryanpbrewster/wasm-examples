# Giving WebAssembly hooks to the outside world

We can tell the WebAssembly runtime about foreign functions like so:
```
  (import "./time.js" "__wbg_now_1c3b9952292fb29e" (func $__wbg_now_1c3b9952292fb29e (type 2)))
```

In this case we're telling it that there's a function named `__wbg_now_1c3b9952292fb29e` that it can call,
and the previously broken code now looks like
```
    block  ;; label = @1
      block  ;; label = @2
        call $__wbg_now_1c3b9952292fb29e
        local.tee 3
        f64.abs
        f64.const 0x1p+63 (;=9.22337e+18;)
        f64.lt
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i64.trunc_f64_s
        local.set 2
        br 1 (;@1;)
      end
      i64.const -9223372036854775808
      local.set 2
    end
```

No `unreachable` to be found!


Of course, now running this code is more complex. We can't just invoke the runtime, we need to bind all
the functions that the runtime expects. The auto-generated `time.js` looks like
```
module.exports.__wbg_now_1c3b9952292fb29e = typeof Date.now == 'function' ? Date.now : notDefined('Date.now');
```
