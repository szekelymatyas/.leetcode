/*
 * @lc app=leetcode id=10 lang=rust
 *
 * [10] Regular Expression Matching
 */

// @lc code=start

enum ExpressionType {
    Char(char),
    Dot,
}

enum Expression {
    Single(ExpressionType),
    ZeroOrAny(ExpressionType),
}
trait ExpressionConverter {
    fn get_expressions(&self) -> Vec<Expression>;
}

impl ExpressionConverter for String {
    fn get_expressions(&self) -> Vec<Expression> {
        let mut expressions: Vec<Expression> = Vec::new();
        let mut iter = self.chars().peekable();

        while let Some(ch1) = iter.next() {
            if let Some(ch2) = iter.peek() {
                match (ch1, ch2) {
                    ('.', '*') => {
                        expressions.push(Expression::ZeroOrAny(ExpressionType::Dot));
                        iter.next();
                    }
                    (c, '*') => {
                        expressions.push(Expression::ZeroOrAny(ExpressionType::Char(c)));
                        iter.next();
                    }
                    ('.', _) => expressions.push(Expression::Single(ExpressionType::Dot)),
                    (c, _) => expressions.push(Expression::Single(ExpressionType::Char(c))),
                };
            } else {
                match ch1 {
                    '.' => expressions.push(Expression::Single(ExpressionType::Dot)),
                    c => expressions.push(Expression::Single(ExpressionType::Char(c))),
                };
            }
        }
        expressions
    }
}

trait Compressor {
    fn compress(self) -> Vec<Expression>;
}

impl Compressor for Vec<Expression> {
    fn compress(mut self) -> Vec<Expression> {
        let mut index = 0;

        while index + 1 < self.len() {
            let exp1 = &self[index];
            let exp2 = &self[index + 1];
            match (exp1, exp2) {
                (
                    Expression::ZeroOrAny(ExpressionType::Dot),
                    Expression::ZeroOrAny(ExpressionType::Dot),
                ) => {
                    self.remove(index);
                }
                (
                    Expression::ZeroOrAny(ExpressionType::Char(_)),
                    Expression::ZeroOrAny(ExpressionType::Dot),
                ) => {
                    self.remove(index);
                }
                (
                    Expression::ZeroOrAny(ExpressionType::Dot),
                    Expression::ZeroOrAny(ExpressionType::Char(_)),
                ) => {
                    self.remove(index + 1);
                }
                (
                    Expression::ZeroOrAny(ExpressionType::Char(c1)),
                    Expression::ZeroOrAny(ExpressionType::Char(c2)),
                ) if c1 == c2 => {
                    self.remove(index);
                }
                (_, _) => {
                    index += 1;
                }
            }
        }
        return self;
    }
}

trait Regex {
    fn match_pattern(self, s: String) -> bool;
}

impl Regex for String {
    fn match_pattern(self, p: String) -> bool {
        let chars = self.chars().collect::<Vec<char>>();
        let chars = chars.as_slice();
        let exps = p.get_expressions().compress();
        let exps = exps.as_slice();
        return match_pattern(exps, chars);
    }
}

fn match_pattern(exps: &[Expression], chars: &[char]) -> bool {
    match (exps, chars) {
        ([], []) => {
            return true;
        }
        ([], [..]) => {
            return false;
        }
        ([_exps @ ..], []) => {
            if _exps.iter().any(|exp| match exp {
                Expression::Single(_) => { return true; },
                _ => { return false; },
            }) {
                return false;
            }
            return true;
        }
        ([Expression::Single(exp), exps @ ..], [ch, chars @ ..]) => {
            return match_single_expression(exp, ch, exps, chars);
        }
        ([exps @ .., Expression::Single(exp)], [chars @ .., ch]) => {
            return match_single_expression(exp, ch, exps, chars);
        }
        ([Expression::ZeroOrAny(ExpressionType::Char(ch)), exps @ ..], chars) => {
            let mut index = 0;
            while index < chars.len() && &chars[index] == ch
            {
                index += 1;
            }
            return match_pattern(exps, &chars[index..])
        },
        ([exps @ .., Expression::ZeroOrAny(ExpressionType::Char(ch))], chars) => {
            let mut index = chars.len();
            while index > 0  && &chars[index -1] == ch
            {
                index -= 1;
            }
            return match_pattern(exps, &chars[..index])
        },       
        ([Expression::ZeroOrAny(ExpressionType::Dot)], _) => {
            return true;
        },        
        ([Expression::ZeroOrAny(ExpressionType::Dot),  exps @ ..], chars) => {
            let mut index = 0;
            while index < chars.len()
            {
                if match_pattern(exps, &chars[index..]) {
                    return true;
                }
                index += 1;
            }
            return false;
        },
        _ => panic!("Not implemented")
    }
}

fn match_single_expression(exp: &ExpressionType, ch: &char, exps: &[Expression], chars: &[char]) -> bool {
    match (exp, ch) {
        (ExpressionType::Dot, _) => return match_pattern(exps, chars),
        (ExpressionType::Char(ch1), ch2) => {
            if ch1 != ch2 {
                return false;
            }
            return match_pattern(exps, chars);
        }
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        return s.match_pattern(p);
    }
}
// @lc code=end

