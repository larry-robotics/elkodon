#[macro_export(local_inner_macros)]
macro_rules! assert_that {
    ($lhs:expr, eq $rhs:expr) => {
        {
            let lval = &$lhs;
            let rval = &$rhs;

            if !(lval == rval) {
                assert_that!(message $lhs, $rhs, lval, rval, "==");
            }
        }
   };
    ($lhs:expr, ne $rhs:expr) => {
        {
            let lval = &$lhs;
            let rval = &$rhs;

            if !(lval != rval) {
                assert_that!(message $lhs, $rhs, lval, rval, "!=");
            }
        }
    };
    ($lhs:expr, lt $rhs:expr) => {
        {
            let lval = &$lhs;
            let rval = &$rhs;

            if !(lval < rval) {
                assert_that!(message $lhs, $rhs, lval, rval, "<");
            }
        }
    };
    ($lhs:expr, le $rhs:expr) => {
        {
            let lval = &$lhs;
            let rval = &$rhs;

            if !(lval <= rval) {
                assert_that!(message $lhs, $rhs, lval, rval, "<=");
            }
        }
    };
    ($lhs:expr, gt $rhs:expr) => {
        {
            let lval = &$lhs;
            let rval = &$rhs;

            if !(lval > rval) {
                assert_that!(message $lhs, $rhs, lval, rval, ">");
            }
        }
    };
    ($lhs:expr, ge $rhs:expr) => {
        {
            let lval = &$lhs;
            let rval = &$rhs;

            if !(lval >= rval) {
                assert_that!(message $lhs, $rhs, lval, rval, ">=");
            }
        }
    };
    ($lhs:expr, mod $rhs:expr, is $result:expr) => {
        {
            let lval = &$lhs;
            let rval = &$rhs;
            let act_result = lval % rval;

            if !(act_result == $result) {
                assert_that!(message $lhs, $rhs, lval, rval, "%", $result, act_result);
            }
        }
    };
    ($lhs:expr, is_ok) => {
        {
            let lval = $lhs.is_ok();

            if !lval {
                assert_that!(message_result $lhs, "is_ok()");
            }
        }
    };
    ($lhs:expr, is_err) => {
        {
            let lval = $lhs.is_err();

            if !lval {
                assert_that!(message_result $lhs, "is_err()");
            }
        }
    };
    ($lhs:expr, is_some) => {
        {
            let lval = $lhs.is_some();

            if !lval {
                assert_that!(message_result $lhs, "is_some()");
            }
        }
    };
    ($lhs:expr, is_none) => {
        {
            let lval = $lhs.is_none();

            if !lval {
                assert_that!(message_result $lhs, "is_none()");
            }
        }
    };
    ($lhs:expr, is_empty) => {
        {
            let lval = $lhs.is_empty();

            if !lval {
                assert_that!(message_result $lhs, "is_empty()");
            }
        }
    };
    ($lhs:expr, is_not_empty) => {
        {
            let lval = !$lhs.is_empty();

            if !lval {
                assert_that!(message_result $lhs, "is_empty() (not)");
            }
        }
    };
    ($lhs:expr, len $rhs:expr) => {
        {
            let lval = $lhs.len();
            if !(lval == $rhs) {
                assert_that!(message_property $lhs, lval, "len()", $rhs);
            }
        }
    };
    ($lhs:expr, contains $rhs:expr) => {
        {
            let mut does_contain = false;
            for value in &$lhs {
                if *value == $rhs {
                    does_contain = true;
                    break;
                }
            }
            if !does_contain {
                assert_that!(message_contains $lhs, $rhs);
            }
        }
    };
    [color_start] => {
        "\x1b[1;4;33m"
    };
    [color_end] => {
        "\x1b[0m"
    };
    [message_contains $lhs:expr, $rhs:expr] => {
        core::panic!(
            "assertion failed: {}expr: {} contains {} ({});  contents: {:?}{}",
            assert_that![color_start],
            core::stringify!($lhs),
            core::stringify!($rhs),
            $rhs,
            $lhs,
            assert_that![color_end]
        );
    };
    [message_property $lhs:expr, $lval:expr, $property:expr, $rhs:expr] => {
        core::panic!(
            "assertion failed: {}expr: {}.{} == {};  value: {} == {}{}",
            assert_that![color_start],
            core::stringify!($lhs),
            $property,
            $rhs,
            $lval,
            $rhs,
            assert_that![color_end]
        );
    };
    [message_result $lhs:expr, $state:expr] => {
        core::panic!(
            "assertion failed: {}{}.{}{}",
            assert_that![color_start],
            core::stringify!($lhs),
            $state,
            assert_that![color_end]
        );
    };
    [message $lhs:expr, $rhs:expr, $lval:expr, $rval:expr, $symbol:expr] => {
        core::panic!(
            "assertion failed: {}expr: {} {} {};  value: {:?} {} {:?}{}",
            assert_that![color_start],
            core::stringify!($lhs),
            $symbol,
            core::stringify!($rhs),
            $lval,
            $symbol,
            $rval,
            assert_that![color_end]
        );
    };
    [message $lhs:expr, $rhs:expr, $lval:expr, $rval:expr, $symbol:expr, $exp_result:expr, $act_result:expr] => {
        core::panic!(
            "assertion failed: {}expr: {} {} {} == {:?};  value: {:?} {} {:?} == {:?}{}",
            assert_that![color_start],
            core::stringify!($lhs),
            $symbol,
            core::stringify!($rhs),
            $exp_result,
            $lval,
            $symbol,
            $rval,
            $act_result,
            assert_that![color_end]
        );
    }
}
