use super::check_miden;
use expect_test::expect;

#[test]
fn test_smoke() {
    let input = vec![];
    let secret_input = vec![];
    let expected_output = vec![2];
    check_miden(
        r#"
(module 
    (start $main)
    (func $main 
        i32.const 1
        i32.const 1
        i32.add
        return)
)"#
        .to_string(),
        input,
        secret_input,
        expected_output,
        expect![[r#"
            proc.main.0
            push.1
            push.1
            add
            end

            proc.globals_get.0
            push.18446744069414584317
            mul
            push.2147467263
            add
            mem_load
            end

            proc.globals_set.0
            push.18446744069414584317
            mul
            push.2147467263
            add
            swap.1
            swap.1
            mem_store
            end

            proc.save_pub_inputs.1
            push.2147483647
            loc_store.0
            sdepth
            push.16
            neq
            while.true
            loc_load.0
            swap.1
            swap.1
            mem_store
            loc_load.0
            push.8
            sub
            push.0
            exec.globals_set
            sdepth
            push.16
            neq
            end

            end

            proc.init_pub_outputs.0
            push.2147483647
            push.1
            exec.globals_set
            end

            proc.load_pub_outputs_on_stack.1
            push.1
            exec.globals_get
            dup.0
            loc_store.0
            push.2147483647
            sub
            neq.0
            while.true
            loc_load.0
            dup.0
            mem_load
            push.8
            add
            dup.0
            loc_store.0
            push.2147483647
            sub
            dup.0
            neq.0
            end

            end

            proc.start_with_miden_io_persistent.0
            exec.save_pub_inputs
            exec.init_pub_outputs
            exec.main
            exec.load_pub_outputs_on_stack
            end

            begin
            exec.start_with_miden_io_persistent
            end
        "#]],
    );
}
