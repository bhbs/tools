use crate::js::expressions::arrow_function_expression::is_multiline_template_starting_on_same_line;
use crate::prelude::*;
use crate::utils::{is_call_like_expression, write_arguments_multi_line};
use rome_formatter::{format_args, write, CstFormatContext};
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsAnyFunctionBody, JsAnyLiteralExpression, JsAnyName,
    JsAnyStatement, JsArrayExpression, JsArrowFunctionExpression, JsCallArgumentList,
    JsCallArguments, JsCallArgumentsFields, JsCallExpression, JsExpressionStatement,
    TsReferenceType,
};
use rome_rowan::{AstSeparatedList, SyntaxResult, SyntaxTokenText};

#[derive(Debug, Clone, Default)]
pub struct FormatJsCallArguments;

impl FormatNodeRule<JsCallArguments> for FormatJsCallArguments {
    fn fmt_fields(&self, node: &JsCallArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCallArgumentsFields {
            l_paren_token,
            args,
            r_paren_token,
        } = node.as_fields();

        let l_paren_token = l_paren_token?;
        let r_paren_token = r_paren_token?;
        let arguments_len = args.len();
        if arguments_len == 0 {
            return write!(
                f,
                [
                    l_paren_token.format(),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    r_paren_token.format()
                ]
            );
        }

        let call_expression = node.parent::<JsCallExpression>();

        if is_commonjs_or_amd_call(node, call_expression.as_ref())?
            || is_multiline_template_only_args(node)
        {
            return write!(
                f,
                [
                    l_paren_token.format(),
                    format_with(|f| {
                        f.join_with(space())
                            .entries(
                                args.format_separated(",")
                                    .with_trailing_separator(TrailingSeparator::Omit),
                            )
                            .finish()
                    }),
                    r_paren_token.format()
                ]
            );
        }

        let mut iter = args.iter();
        let first_argument = iter.next();
        let second_argument = iter.next();
        let third_argument = iter.next();

        if let (Some(first_argument), Some(second_argument)) = (first_argument, second_argument) {
            let first_argument = first_argument?;
            let second_argument = second_argument?;

            let is_framework_test_call =
                if let Some(call_expression) = node.parent::<JsCallExpression>() {
                    let callee = call_expression.callee()?;

                    is_framework_test_call(IsTestFrameworkCallPayload {
                        first_argument: &first_argument,
                        second_argument: &second_argument,
                        third_argument: &third_argument,
                        arguments_len,
                        callee: &callee,
                    })?
                } else {
                    false
                };

            let is_react_hook_with_deps_array = is_react_hook_with_deps_array(
                &first_argument,
                &second_argument,
                f.context().comments(),
            )?;

            if is_framework_test_call || is_react_hook_with_deps_array {
                write!(f, [l_paren_token.format(),])?;
                let separated = args
                    .format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit);

                f.join_with(space()).entries(separated).finish()?;
                return write!(f, [r_paren_token.format()]);
            }
        };

        // we now extracts the formatted version of trivias and tokens of the delimiters
        // tokens on the left
        let l_paren = l_paren_token.format();

        // tokens on the right
        let r_paren = r_paren_token.format();

        let comments = f.context().comments();
        let should_group_first_argument = should_group_first_argument(&args, comments)?;
        let should_group_last_argument = should_group_last_argument(&args, comments)?;

        // if the first or last groups needs grouping, then we prepare some special formatting
        if should_group_first_argument || should_group_last_argument {
            // We finished the "simple cases", we now need to use `best_fitting`.
            // We now need to allocate a new vector with cached nodes, this is needed because
            // we can't attempt to print the same node twice without incur in "printed token twice" errors.
            // We also disallow the trailing separator, we are interested in doing it manually.
            let mut separated: Vec<_> = args
                .format_separated(",")
                .with_trailing_separator(TrailingSeparator::Omit)
                .map(|e| e.memoized())
                .collect();

            let mut any_argument_breaks = false;
            let mut first_last_breaks = false;

            for (index, argument) in separated.iter_mut().enumerate() {
                let breaks = argument.inspect(f)?.will_break();

                any_argument_breaks = any_argument_breaks || breaks;

                if (should_group_first_argument && index > 0)
                    || (should_group_last_argument && index < args.len() - 1)
                {
                    first_last_breaks = first_last_breaks || breaks;
                    if breaks {
                        break;
                    }
                }
            }

            let format_flat_arguments = format_with(|f| {
                f.join_with(soft_line_break_or_space())
                    .entries(separated.iter())
                    .finish()
            });

            // We now cache them the delimiters tokens. This is needed because `[rome_formatter::best_fitting]` will try to
            // print each version first
            // tokens on the left
            let l_paren = l_paren.memoized();

            // tokens on the right
            let r_paren = r_paren.memoized();

            // This is the version of where all the arguments are broken out
            let all_arguments_expanded = format_with(|f| {
                // this formatting structure replicates what we have inside the `format_delimited`
                // function, but here we use a different way to print the trailing separator
                write!(
                    f,
                    [group(&format_args![
                        l_paren,
                        soft_block_indent(&format_with(|f| {
                            write_arguments_multi_line(separated.iter(), f)
                        })),
                        r_paren
                    ])
                    .should_expand(true)]
                )
            });

            if first_last_breaks {
                return write!(f, [all_arguments_expanded]);
            }

            let edge_arguments_do_not_break = format_with(|f| {
                // `should_group_first_argument` and `should_group_last_argument` are mutually exclusive
                // which means that if one is `false`, then the other is `true`.
                // This means that in this branch we format the case where `should_group_first_argument`,
                // in the else branch we format the case where `should_group_last_argument` is `true`.
                write!(f, [l_paren])?;
                if should_group_first_argument {
                    // special formatting of the first element
                    let mut iter = separated.iter();
                    // SAFETY: check on the existence of at least one argument are done before
                    let first = iter.next().unwrap();
                    f.join_with(&space()).entry(&first).entries(iter).finish()?;
                } else {
                    // special formatting of the last element
                    let mut iter = separated.iter();
                    // SAFETY: check on the existence of at least one argument are done before
                    let last = iter.next_back().unwrap();
                    f.join_with(&space()).entries(iter).entry(&last).finish()?;
                }
                write!(f, [r_paren])
            });

            if any_argument_breaks {
                write!(f, [expand_parent()])?;
            }

            write!(
                f,
                [best_fitting![
                    format_args![l_paren, format_flat_arguments, r_paren],
                    group(&edge_arguments_do_not_break).should_expand(true),
                    all_arguments_expanded
                ]]
            )
        } else {
            write!(
                f,
                [group(&format_args![
                    l_paren,
                    soft_block_indent(&format_with(|f| {
                        let separated = args
                            .format_separated(",")
                            .with_trailing_separator(TrailingSeparator::Omit)
                            .nodes_grouped();
                        write_arguments_multi_line(separated, f)
                    })),
                    r_paren,
                ])]
            )
        }
    }

    fn fmt_dangling_comments(&self, _: &JsCallArguments, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}

/// Checks if the the first argument requires grouping
fn should_group_first_argument(
    list: &JsCallArgumentList,
    comments: &JsComments,
) -> SyntaxResult<bool> {
    if list.len() != 2 {
        return Ok(false);
    }
    let mut iter = list.iter();
    // SAFETY: checked at the beginning of the function
    let first = iter.next().unwrap()?;
    let second = iter.next().unwrap()?;

    let is_function_like = match first.as_js_any_expression() {
        Some(JsAnyExpression::JsFunctionExpression(_)) => true,
        Some(JsAnyExpression::JsArrowFunctionExpression(arrow)) => {
            matches!(arrow.body()?, JsAnyFunctionBody::JsFunctionBody(_))
        }
        _ => false,
    };

    let (second_arg_is_function_like, can_group) = match second.as_js_any_expression() {
        Some(second_expression) => {
            let second_arg_is_function_like = matches!(
                &second_expression,
                JsAnyExpression::JsFunctionExpression(_)
                    | JsAnyExpression::JsArrowFunctionExpression(_)
                    | JsAnyExpression::JsConditionalExpression(_)
            );
            (
                second_arg_is_function_like,
                could_group_expression_argument(second_expression, false, comments)?,
            )
        }
        None => (false, false),
    };

    Ok(!comments.has_comments(first.syntax())
        && is_function_like
        && !second_arg_is_function_like
        && !can_group)
}

/// Checks if the last group requires grouping
fn should_group_last_argument(
    list: &JsCallArgumentList,
    comments: &JsComments,
) -> SyntaxResult<bool> {
    let list_len = list.len();
    let mut iter = list.iter();
    let last = iter.next_back();
    let penultimate = iter.next_back();

    if let Some(last) = last {
        let last = last?;
        let check_with_penultimate = if let Some(penultimate) = penultimate {
            let penultimate = penultimate?;
            let different_kind = last.syntax().kind() != penultimate.syntax().kind();

            let no_array_and_arrow_function = list_len != 2
                || !JsArrayExpression::can_cast(penultimate.syntax().kind())
                || !JsArrowFunctionExpression::can_cast(last.syntax().kind());

            // TODO implement no poor printed array
            let _no_poor_printed_array =
                !list_len > 1 && JsArrayExpression::can_cast(last.syntax().kind());
            different_kind && no_array_and_arrow_function
        } else {
            true
        };

        let can_group = match &last {
            JsAnyCallArgument::JsAnyExpression(expression) => {
                could_group_expression_argument(expression, false, comments)?
            }
            _ => false,
        };

        Ok(!comments.has_leading_comments(last.syntax())
            && !comments.has_trailing_comments(last.syntax())
            && can_group
            && check_with_penultimate)
    } else {
        Ok(false)
    }
}

/// Checks if the current argument could be grouped
fn could_group_expression_argument(
    argument: &JsAnyExpression,
    is_arrow_recursion: bool,
    comments: &JsComments,
) -> SyntaxResult<bool> {
    let result = match argument {
        JsAnyExpression::JsObjectExpression(object_expression) => {
            object_expression.members().len() > 0
                || comments.has_comments(object_expression.syntax())
        }

        JsAnyExpression::JsArrayExpression(array_expression) => {
            array_expression.elements().len() > 0
                || comments.has_comments(array_expression.syntax())
        }
        JsAnyExpression::TsTypeAssertionExpression(assertion_expression) => {
            could_group_expression_argument(&assertion_expression.expression()?, false, comments)?
        }

        JsAnyExpression::TsAsExpression(as_expression) => {
            could_group_expression_argument(&as_expression.expression()?, false, comments)?
        }
        JsAnyExpression::JsArrowFunctionExpression(arrow_function) => {
            let body = arrow_function.body()?;
            let return_type_annotation = arrow_function.return_type_annotation();

            // Handles cases like:
            //
            // app.get("/", (req, res): void => {
            //     res.send("Hello World!");
            // });
            //
            // export class Thing implements OtherThing {
            //   do: (type: Type) => Provider<Prop> = memoize(
            //     (type: ObjectType): Provider<Opts> => {}
            //   );
            // }
            let can_group_type =
                !return_type_annotation
                    .and_then(|rty| rty.ty().ok())
                    .map_or(false, |any_type| {
                        TsReferenceType::can_cast(any_type.syntax().kind())
                            || if let JsAnyFunctionBody::JsFunctionBody(function_body) = &body {
                                function_body
                                    .statements()
                                    .iter()
                                    .any(|st| matches!(st, JsAnyStatement::JsEmptyStatement(_)))
                            } else {
                                true
                            }
                    });

            let expression_body = match &body {
                JsAnyFunctionBody::JsFunctionBody(_) => None,
                JsAnyFunctionBody::JsAnyExpression(expression) => Some(expression),
            };

            let body_is_delimited = matches!(body, JsAnyFunctionBody::JsFunctionBody(_))
                || matches!(
                    expression_body,
                    Some(
                        JsAnyExpression::JsObjectExpression(_)
                            | JsAnyExpression::JsArrayExpression(_)
                    )
                );

            if let Some(any_expression) = expression_body {
                let is_nested_arrow_function =
                    if let JsAnyExpression::JsArrowFunctionExpression(arrow_function_expression) =
                        &any_expression
                    {
                        arrow_function_expression
                            .body()
                            .ok()
                            .and_then(|body| body.as_js_any_expression().cloned())
                            .and_then(|body| {
                                could_group_expression_argument(&body, true, comments).ok()
                            })
                            .unwrap_or(false)
                    } else {
                        false
                    };

                body_is_delimited
                    && is_nested_arrow_function
                    && can_group_type
                    && (!is_arrow_recursion
                        && (is_call_like_expression(any_expression)
                            || matches!(
                                any_expression,
                                JsAnyExpression::JsConditionalExpression(_)
                            )))
            } else {
                body_is_delimited && can_group_type
            }
        }

        JsAnyExpression::JsFunctionExpression(_) => true,
        _ => false,
    };

    Ok(result)
}

/// Tests if this is a call to commonjs [`require`](https://nodejs.org/api/modules.html#requireid)
/// or amd's [`define`](https://github.com/amdjs/amdjs-api/wiki/AMD#define-function-) function.
fn is_commonjs_or_amd_call(
    arguments: &JsCallArguments,
    call: Option<&JsCallExpression>,
) -> SyntaxResult<bool> {
    let call = match call {
        Some(call) => call,
        None => return Ok(false),
    };

    let callee = call.callee()?;

    Ok(match callee {
        JsAnyExpression::JsIdentifierExpression(identifier) => {
            let reference = identifier.name()?;

            if reference.has_name("require") {
                true
            } else if reference.has_name("define") {
                let in_statement = call.parent::<JsExpressionStatement>().is_some();

                if in_statement {
                    let args = arguments.args();
                    match args.len() {
                        1 => true,
                        2 => matches!(
                            args.first(),
                            Some(Ok(JsAnyCallArgument::JsAnyExpression(
                                JsAnyExpression::JsArrayExpression(_)
                            )))
                        ),
                        3 => {
                            let mut iter = args.iter();
                            let first = iter.next();
                            let second = iter.next();
                            matches!(
                                (first, second),
                                (
                                    Some(Ok(JsAnyCallArgument::JsAnyExpression(
                                        JsAnyExpression::JsAnyLiteralExpression(
                                            JsAnyLiteralExpression::JsStringLiteralExpression(_)
                                        )
                                    ))),
                                    Some(Ok(JsAnyCallArgument::JsAnyExpression(
                                        JsAnyExpression::JsArrayExpression(_)
                                    )))
                                )
                            )
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }
        _ => false,
    })
}

/// Returns `true` if `arguments` contains a single [multiline template literal argument that starts on its own ](is_multiline_template_starting_on_same_line).
fn is_multiline_template_only_args(arguments: &JsCallArguments) -> bool {
    let args = arguments.args();

    match args.first() {
        Some(Ok(JsAnyCallArgument::JsAnyExpression(JsAnyExpression::JsTemplate(template))))
            if args.len() == 1 =>
        {
            is_multiline_template_starting_on_same_line(&template)
        }
        _ => false,
    }
}

/// This function is used to check if the code is a hook-like code:
///
/// ```js
/// useMemo(() => {}, [])
/// ```
fn is_react_hook_with_deps_array(
    first_argument: &JsAnyCallArgument,
    second_argument: &JsAnyCallArgument,
    comments: &JsComments,
) -> SyntaxResult<bool> {
    if comments.has_comments(first_argument.syntax())
        || comments.has_comments(second_argument.syntax())
    {
        return Ok(false);
    }

    let first_expression = match first_argument {
        JsAnyCallArgument::JsAnyExpression(expression) => Some(expression),
        _ => None,
    };

    let first_node_matches = if let Some(JsAnyExpression::JsArrowFunctionExpression(
        arrow_function,
    )) = first_expression
    {
        let no_parameters = arrow_function.parameters()?.is_empty();
        let body = arrow_function.body()?;
        let is_block = matches!(body, JsAnyFunctionBody::JsFunctionBody(_));

        no_parameters && is_block
    } else {
        false
    };

    let second_node_matches = matches!(second_argument, JsAnyCallArgument::JsAnyExpression(_));
    if first_node_matches && second_node_matches {
        Ok(true)
    } else {
        Ok(false)
    }
}

struct IsTestFrameworkCallPayload<'a> {
    first_argument: &'a JsAnyCallArgument,
    second_argument: &'a JsAnyCallArgument,
    third_argument: &'a Option<SyntaxResult<JsAnyCallArgument>>,
    arguments_len: usize,
    callee: &'a JsAnyExpression,
}

pub(crate) fn is_test_call_expression(expression: &JsCallExpression) -> SyntaxResult<bool> {
    let arguments = expression.arguments()?.args();
    let mut arguments_iter = arguments.iter();

    let result = match (
        arguments_iter.next(),
        arguments_iter.next(),
        arguments_iter.next(),
    ) {
        (Some(first_argument), Some(second_argument), third_argument) => {
            is_framework_test_call(IsTestFrameworkCallPayload {
                first_argument: &first_argument?,
                second_argument: &second_argument?,
                third_argument: &third_argument,
                arguments_len: arguments.len(),
                callee: &expression.callee()?,
            })?
        }
        (_, _, _) => false,
    };

    Ok(result)
}

/// This is a specialised function that checks if the current [call expression]
/// resembles a call expression usually used by a testing frameworks.
///
/// If the [call expression] matches the criteria, a different formatting is applied.
///
/// To evaluable the eligibility of a  [call expression] to be a test framework like,
/// we need to check its [callee] and its [arguments].
///
/// 1. The [callee] must contain a name or a chain of names that belongs to the
/// test frameworks, for example: `test()`, `test.only()`, etc.
/// 2. The [arguments] should be at the least 2
/// 3. The first argument has to be a string literal
/// 4. The third argument, if present, has to be a number literal
/// 5. The second argument has to be an [arrow function expression] or [function expression]
/// 6. Both function must have zero or one parameters
///
/// [call expression]: crate::rome_js_syntax::JsCallExpression
/// [callee]: crate::rome_js_syntax::JsAnyExpression
/// [arguments]: crate::rome_js_syntax::JsCallArgumentList
/// [arrow function expression]: crate::rome_js_syntax::JsArrowFunctionExpression
/// [function expression]: crate::rome_js_syntax::JsCallArgumentList
fn is_framework_test_call(payload: IsTestFrameworkCallPayload) -> SyntaxResult<bool> {
    let IsTestFrameworkCallPayload {
        first_argument,
        second_argument,
        third_argument,
        arguments_len,
        callee,
    } = payload;
    let first_argument_expression = first_argument.as_js_any_expression();
    let second_argument_expression = second_argument.as_js_any_expression();
    let third_argument_expression =
        third_argument
            .as_ref()
            .and_then(|third_argument| match third_argument {
                Ok(argument) => argument.as_js_any_expression(),
                _ => None,
            });

    let first_argument_is_literal_like = matches!(
        first_argument_expression,
        Some(
            JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsStringLiteralExpression(_)
            ) | JsAnyExpression::JsTemplate(_)
        )
    );

    if first_argument_is_literal_like && contains_a_test_pattern(callee)? {
        if arguments_len == 2 {
            Ok(matches!(
                second_argument_expression,
                Some(
                    JsAnyExpression::JsArrowFunctionExpression(_)
                        | JsAnyExpression::JsFunctionExpression(_)
                )
            ))
        } else {
            // if the third argument is not a numeric literal, we bail
            // example: `it("name", () => { ... }, 2500)`
            if !matches!(
                third_argument_expression,
                Some(JsAnyExpression::JsAnyLiteralExpression(
                    JsAnyLiteralExpression::JsNumberLiteralExpression(_)
                ))
            ) {
                return Ok(false);
            }

            let result = match second_argument_expression {
                Some(JsAnyExpression::JsFunctionExpression(node)) => {
                    node.parameters()?.items().len() <= 1
                }
                Some(JsAnyExpression::JsArrowFunctionExpression(node)) => {
                    let body = node.body()?;
                    let has_enough_parameters = node.parameters()?.len() <= 1;
                    matches!(body, JsAnyFunctionBody::JsFunctionBody(_)) && has_enough_parameters
                }
                _ => false,
            };
            Ok(result)
        }
    } else {
        Ok(false)
    }
}

/// This function checks if a call expressions has one of the following members:
/// - `it`
/// - `it.only`
/// - `it.skip`
/// - `describe`
/// - `describe.only`
/// - `describe.skip`
/// - `test`
/// - `test.only`
/// - `test.skip`
/// - `test.step`
/// - `test.describe`
/// - `test.describe.only`
/// - `test.describe.parallel`
/// - `test.describe.parallel.only`
/// - `test.describe.serial`
/// - `test.describe.serial.only`
/// - `skip`
/// - `xit`
/// - `xdescribe`
/// - `xtest`
/// - `fit`
/// - `fdescribe`
/// - `ftest`
///
/// Based on this [article]
///
/// [article]: https://craftinginterpreters.com/scanning-on-demand.html#tries-and-state-machines
fn contains_a_test_pattern(callee: &JsAnyExpression) -> SyntaxResult<bool> {
    let members: Vec<_> = matches_test_call(callee)?;

    let first = members.get(0).map(|t| t.text());
    let second = members.get(1).map(|t| t.text());
    let third = members.get(2).map(|t| t.text());
    let fourth = members.get(3).map(|t| t.text());
    let fifth = members.get(4).map(|t| t.text());

    Ok(match first {
        Some("it" | "describe") => match second {
            None => true,
            Some("only" | "skip") => third.is_none(),
            _ => false,
        },
        Some("test") => match second {
            None => true,
            Some("only" | "skip" | "step") => third.is_none(),
            Some("describe") => match third {
                None => true,
                Some("only") => true,
                Some("parallel" | "serial") => match fourth {
                    None => true,
                    Some("only") => fifth.is_none(),
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        },
        Some("skip" | "xit" | "xdescribe" | "xtest" | "fit" | "fdescribe" | "ftest") => true,
        _ => false,
    })
}

/// This is particular used to identify if a [JsCallExpression] has the shape
/// of a call argument coming from a test framework.
///
/// An example are call arguments coming from Mocha, Jest, etc.
///
/// ```js
/// describe("My component", () => {
///     it("should render", () => {
///
///     });
/// })
///
/// test.only("", testSomething);
/// ```
///
/// This function should accept the `callee` of [JsCallExpression] and the
/// string pattern to test against. For example "test", "test.only"
fn matches_test_call(callee: &JsAnyExpression) -> SyntaxResult<Vec<SyntaxTokenText>> {
    // this the max depth plus one, because we want to catch cases where we have test.only.WRONG
    const MAX_DEPTH: u8 = 5;
    let mut test_call = Vec::with_capacity(MAX_DEPTH as usize);
    let mut current_node = callee.clone();
    let mut i = 0;

    while i < MAX_DEPTH {
        i += 1;
        current_node = match current_node {
            JsAnyExpression::JsIdentifierExpression(identifier) => {
                let value_token = identifier.name()?.value_token()?;
                let value = value_token.token_text_trimmed();
                test_call.push(value);
                break;
            }
            JsAnyExpression::JsStaticMemberExpression(member_expression) => {
                match member_expression.member()? {
                    JsAnyName::JsName(name) => {
                        let value = name.value_token()?;
                        test_call.push(value.token_text_trimmed());
                        member_expression.object()?
                    }
                    _ => break,
                }
            }
            _ => break,
        };
    }
    test_call.reverse();
    Ok(test_call)
}

#[cfg(test)]
mod test {
    use super::contains_a_test_pattern;
    use rome_diagnostics::file::FileId;
    use rome_js_parser::parse;
    use rome_js_syntax::{JsCallExpression, SourceType};
    use rome_rowan::AstNodeList;

    fn extract_call_expression(src: &str) -> JsCallExpression {
        let source_type = SourceType::js_module();
        let result = parse(src, FileId::zero(), source_type);
        let module = result
            .tree()
            .as_js_module()
            .unwrap()
            .items()
            .first()
            .unwrap();

        module
            .as_js_any_statement()
            .unwrap()
            .as_js_expression_statement()
            .unwrap()
            .expression()
            .unwrap()
            .as_js_call_expression()
            .unwrap()
            .clone()
    }

    #[test]
    fn matches_simple_call() {
        let call_expression = extract_call_expression("test();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );

        let call_expression = extract_call_expression("it();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn matches_static_member_expression() {
        let call_expression = extract_call_expression("test.only();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn matches_static_member_expression_deep() {
        let call_expression = extract_call_expression("test.describe.parallel.only();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn doesnt_static_member_expression_deep() {
        let call_expression = extract_call_expression("test.describe.parallel.only.AHAHA();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(false)
        );
    }
}
