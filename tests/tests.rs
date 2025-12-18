#[cfg(test)]
mod tests {
    use pratt_parser::expression::Expression;

    // Helper function per ridurre duplicazione
    fn assert_parse_eq(input: &str, expected: &str) {
        let expr = Expression::from_str(input);
        println!("Input: {} -> Output: {:?}", input, expr);
        assert_eq!(format!("{}", expr), expected);
    }

    mod single_digit {
        use super::*;

        #[test]
        fn parse_single_number() {
            assert_parse_eq("1", "1");
        }

        #[test]
        fn parse_addition_chain() {
            assert_parse_eq("1+2+3", "(+ (+ 1 2) 3)");
        }

        #[test]
        fn parse_subtraction_chain() {
            assert_parse_eq("1-2-3", "(- (- 1 2) 3)");
        }

        #[test]
        fn parse_multiplication_chain() {
            assert_parse_eq("1*2*3", "(* (* 1 2) 3)");
        }

        #[test]
        fn parse_division_chain() {
            assert_parse_eq("8/4/2", "(/ (/ 8 4) 2)");
        }

        #[test]
        fn parse_exponentiation_chain() {
            assert_parse_eq("2^3^2", "(^ 2 (^ 3 2))");
        }

        #[test]
        fn parse_addition_then_subtraction() {
            assert_parse_eq("1+2-3", "(- (+ 1 2) 3)");
        }

        #[test]
        fn parse_subtraction_then_addition() {
            assert_parse_eq("1-2+3", "(+ (- 1 2) 3)");
        }

        #[test]
        fn parse_multiplication_then_division() {
            assert_parse_eq("6*2/3", "(/ (* 6 2) 3)");
        }

        #[test]
        fn parse_division_then_multiplication() {
            assert_parse_eq("6/2*3", "(* (/ 6 2) 3)");
        }

        #[test]
        fn parse_addition_with_multiplication() {
            assert_parse_eq("1+2*3", "(+ 1 (* 2 3))");
        }

        #[test]
        fn parse_multiplication_with_addition() {
            assert_parse_eq("1*2+3", "(+ (* 1 2) 3)");
        }

        #[test]
        fn parse_addition_with_division() {
            assert_parse_eq("1+8/2", "(+ 1 (/ 8 2))");
        }

        #[test]
        fn parse_division_with_addition() {
            assert_parse_eq("8/2+3", "(+ (/ 8 2) 3)");
        }

        #[test]
        fn parse_multiplication_with_exponentiation() {
            assert_parse_eq("2*3^2", "(* 2 (^ 3 2))");
        }

        #[test]
        fn parse_exponentiation_with_multiplication() {
            assert_parse_eq("2^3*4", "(* (^ 2 3) 4)");
        }

        #[test]
        fn parse_addition_with_exponentiation() {
            assert_parse_eq("1+2^3", "(+ 1 (^ 2 3))");
        }

        #[test]
        fn parse_exponentiation_with_addition() {
            assert_parse_eq("2^3+4", "(+ (^ 2 3) 4)");
        }
    }

    mod multiple_digits {
        use super::*;

        #[test]
        fn parse_large_number() {
            assert_parse_eq("123456", "123456");
        }

        #[test]
        fn parse_addition_chain() {
            assert_parse_eq("11+21+31", "(+ (+ 11 21) 31)");
        }

        #[test]
        fn parse_subtraction_chain() {
            assert_parse_eq("11-21-31", "(- (- 11 21) 31)");
        }

        #[test]
        fn parse_multiplication_chain() {
            assert_parse_eq("11*21*31", "(* (* 11 21) 31)");
        }

        #[test]
        fn parse_division_chain() {
            assert_parse_eq("100/10/2", "(/ (/ 100 10) 2)");
        }

        #[test]
        fn parse_exponentiation_chain() {
            assert_parse_eq("2^10^2", "(^ 2 (^ 10 2))");
        }

        #[test]
        fn parse_addition_then_subtraction() {
            assert_parse_eq("11+21-31", "(- (+ 11 21) 31)");
        }

        #[test]
        fn parse_subtraction_then_addition() {
            assert_parse_eq("11-21+31", "(+ (- 11 21) 31)");
        }

        #[test]
        fn parse_multiplication_then_division() {
            assert_parse_eq("12*10/20", "(/ (* 12 10) 20)");
        }

        #[test]
        fn parse_division_then_multiplication() {
            assert_parse_eq("100/10*5", "(* (/ 100 10) 5)");
        }

        #[test]
        fn parse_addition_with_multiplication() {
            assert_parse_eq("11+21*31", "(+ 11 (* 21 31))");
        }

        #[test]
        fn parse_multiplication_with_addition() {
            assert_parse_eq("11*21+31", "(+ (* 11 21) 31)");
        }

        #[test]
        fn parse_addition_with_division() {
            assert_parse_eq("10+100/20", "(+ 10 (/ 100 20))");
        }

        #[test]
        fn parse_division_with_addition() {
            assert_parse_eq("100/20+15", "(+ (/ 100 20) 15)");
        }

        #[test]
        fn parse_multiplication_with_exponentiation() {
            assert_parse_eq("10*2^5", "(* 10 (^ 2 5))");
        }

        #[test]
        fn parse_exponentiation_with_multiplication() {
            assert_parse_eq("2^10*3", "(* (^ 2 10) 3)");
        }
    }

    mod operator_precedence {
        use super::*;

        #[test]
        fn multiplication_before_addition() {
            assert_parse_eq("2+3*4", "(+ 2 (* 3 4))");
        }

        #[test]
        fn multiplication_before_subtraction() {
            assert_parse_eq("10-2*3", "(- 10 (* 2 3))");
        }

        #[test]
        fn division_before_addition() {
            assert_parse_eq("2+12/3", "(+ 2 (/ 12 3))");
        }

        #[test]
        fn division_before_subtraction() {
            assert_parse_eq("10-12/3", "(- 10 (/ 12 3))");
        }

        #[test]
        fn exponentiation_before_multiplication() {
            assert_parse_eq("2*3^2", "(* 2 (^ 3 2))");
        }

        #[test]
        fn exponentiation_before_division() {
            assert_parse_eq("16/2^2", "(/ 16 (^ 2 2))");
        }

        #[test]
        fn exponentiation_before_addition() {
            assert_parse_eq("1+2^3", "(+ 1 (^ 2 3))");
        }

        #[test]
        fn exponentiation_before_subtraction() {
            assert_parse_eq("10-2^3", "(- 10 (^ 2 3))");
        }

        #[test]
        fn left_to_right_same_precedence_addition_subtraction() {
            assert_parse_eq("10-5+2", "(+ (- 10 5) 2)");
        }

        #[test]
        fn left_to_right_same_precedence_subtraction() {
            assert_parse_eq("10-5-2", "(- (- 10 5) 2)");
        }

        #[test]
        fn left_to_right_same_precedence_multiplication_division() {
            assert_parse_eq("12/2*3", "(* (/ 12 2) 3)");
        }

        #[test]
        fn left_to_right_same_precedence_division() {
            assert_parse_eq("12/2/3", "(/ (/ 12 2) 3)");
        }

        #[test]
        fn right_to_left_exponentiation() {
            assert_parse_eq("2^3^2", "(^ 2 (^ 3 2))");
        }

        #[test]
        fn complex_mixed_operators() {
            assert_parse_eq("1+2*3-4*5", "(- (+ 1 (* 2 3)) (* 4 5))");
        }

        #[test]
        fn complex_all_operators() {
            assert_parse_eq("2+3*4^2-10/5", "(- (+ 2 (* 3 (^ 4 2))) (/ 10 5))");
        }

        #[test]
        fn complex_exponentiation_chains() {
            assert_parse_eq("2^3^2+1", "(+ (^ 2 (^ 3 2)) 1)");
        }
    }

    mod parentheses {
        use super::*;

        #[test]
        fn simple_parentheses() {
            assert_parse_eq("(5)", "5");
        }

        #[test]
        fn parentheses_override_precedence() {
            assert_parse_eq("(1+2)*3", "(* (+ 1 2) 3)");
        }

        #[test]
        fn parentheses_with_division() {
            assert_parse_eq("(10+2)/3", "(/ (+ 10 2) 3)");
        }

        #[test]
        fn parentheses_with_exponentiation() {
            assert_parse_eq("(1+2)^3", "(^ (+ 1 2) 3)");
        }

        #[test]
        fn parentheses_in_exponent() {
            assert_parse_eq("2^(3+1)", "(^ 2 (+ 3 1))");
        }

        #[test]
        fn parentheses_chain() {
            assert_parse_eq("((3*(1+2)))", "(* 3 (+ 1 2))");
        }

        #[test]
        fn nested_parentheses() {
            assert_parse_eq("((1+2)*3)+4", "(+ (* (+ 1 2) 3) 4)");
        }

        #[test]
        fn multiple_parentheses_groups() {
            assert_parse_eq("(1+2)*(3+4)", "(* (+ 1 2) (+ 3 4))");
        }

        #[test]
        fn deeply_nested_parentheses() {
            assert_parse_eq("(((1+2)*3)+4)", "(+ (* (+ 1 2) 3) 4)");
        }

        #[test]
        fn complex_parentheses_expression() {
            assert_parse_eq("(2+3)*(4-1)/(5+2)", "(/ (* (+ 2 3) (- 4 1)) (+ 5 2))");
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn parse_empty_string_panics() {
            assert_parse_eq("", "");
        }

        #[test]
        fn parse_with_whitespace() {
            assert_parse_eq("1 + 2 * 3", "(+ 1 (* 2 3))");
        }

        #[test]
        fn parse_with_extra_whitespace() {
            assert_parse_eq("  1  +  2  *  3  ", "(+ 1 (* 2 3))");
        }

        #[test]
        fn parse_with_tabs() {
            assert_parse_eq("1\t+\t2\t*\t3", "(+ 1 (* 2 3))");
        }

        #[test]
        fn zero_in_expression() {
            assert_parse_eq("0+1*2", "(+ 0 (* 1 2))");
        }

        #[test]
        fn multiple_zeros() {
            assert_parse_eq("100+200", "(+ 100 200)");
        }

        #[test]
        fn division_by_one() {
            assert_parse_eq("5/1", "(/ 5 1)");
        }

        #[test]
        fn exponent_of_zero() {
            assert_parse_eq("5^0", "(^ 5 0)");
        }

        #[test]
        fn exponent_of_one() {
            assert_parse_eq("5^1", "(^ 5 1)");
        }

        #[test]
        fn large_numbers() {
            assert_parse_eq("999999+888888", "(+ 999999 888888)");
        }
        #[test]
        fn division_multiplication_left_associative_matters() {
            // Espressione: 12/2*3

            // CORRETTO (left-to-right):
            // (12/2)*3 = 6*3 = 18 ✓
            assert_parse_eq("2*12/2*3", "(* (/ (* 2 12) 2) 3)");

            // SBAGLIATO (se fosse right-associative):
            // 12/(2*3) = 12/6 = 2 ✗
            // Il tuo parser probabilmente fa: "(/ 12 (* 2 3))"
        }
        #[test]
        fn division_division_order_matters() {
            // 100/10/2
            // Corretto: (100/10)/2 = 10/2 = 5 ✓
            // Sbagliato: 100/(10/2) = 100/5 = 20 ✗
            assert_parse_eq("100/10/2", "(/ (/ 100 10) 2)");
        }

        #[test]
        fn subtraction_subtraction_order_matters() {
            // 10-5-2
            // Corretto: (10-5)-2 = 5-2 = 3 ✓
            // Sbagliato: 10-(5-2) = 10-3 = 7 ✗
            assert_parse_eq("10-5-2", "(- (- 10 5) 2)");
        }

        #[test]
        fn mixed_mult_div_concrete() {
            // 20/4*5
            // Corretto: (20/4)*5 = 5*5 = 25 ✓
            // Sbagliato: 20/(4*5) = 20/20 = 1 ✗
            assert_parse_eq("20/4*5", "(* (/ 20 4) 5)");
        }
    }

    mod associativity {
        use super::*;

        #[test]
        fn addition_left_associative() {
            assert_parse_eq("1+2+3+4", "(+ (+ (+ 1 2) 3) 4)");
        }

        #[test]
        fn subtraction_left_associative() {
            assert_parse_eq("10-2-3-1", "(- (- (- 10 2) 3) 1)");
        }

        #[test]
        fn multiplication_left_associative() {
            assert_parse_eq("2*3*4*5", "(* (* (* 2 3) 4) 5)");
        }

        #[test]
        fn division_left_associative() {
            assert_parse_eq("100/2/5/2", "(/ (/ (/ 100 2) 5) 2)");
        }

        #[test]
        fn exponentiation_right_associative() {
            assert_parse_eq("2^2^3", "(^ 2 (^ 2 3))");
        }

        #[test]
        fn mixed_left_associative() {
            assert_parse_eq("10-3+2-1", "(- (+ (- 10 3) 2) 1)");
        }

        #[test]
        fn mixed_multiplication_division() {
            assert_parse_eq("20*2/4*3", "(* (/ (* 20 2) 4) 3)");
        }
    }

    mod comprehensive {
        use super::*;

        #[test]
        fn realistic_calculation_1() {
            assert_parse_eq("100-20*3+15/5", "(+ (- 100 (* 20 3)) (/ 15 5))");
        }

        #[test]
        fn realistic_calculation_2() {
            assert_parse_eq("2^8/4+3*5-10", "(- (+ (/ (^ 2 8) 4) (* 3 5)) 10)");
        }

        #[test]
        fn realistic_calculation_3() {
            assert_parse_eq("(10+5)*2^3/4", "(/ (* (+ 10 5) (^ 2 3)) 4)");
        }

        #[test]
        fn all_operators_once() {
            assert_parse_eq("1+2-3*4/5^6", "(- (+ 1 2) (/ (* 3 4) (^ 5 6)))");
        }

        #[test]
        fn nested_operations() {
            assert_parse_eq("((2+3)*(4+5))^2", "(^ (* (+ 2 3) (+ 4 5)) 2)");
        }

        #[test]
        fn long_expression() {
            assert_parse_eq(
                "1+2*3-4/2+5^2-6*7+8",
                "(+ (- (+ (- (+ 1 (* 2 3)) (/ 4 2)) (^ 5 2)) (* 6 7)) 8)",
            );
        }
    }
}
