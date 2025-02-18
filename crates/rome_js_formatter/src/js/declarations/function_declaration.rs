use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{
    JsAnyBinding, JsFunctionBody, JsFunctionDeclaration, JsFunctionExportDefaultDeclaration,
    JsFunctionExpression, JsParameters, JsSyntaxToken, TsAnyReturnType,
    TsDeclareFunctionDeclaration, TsReturnTypeAnnotation, TsType, TsTypeParameters,
};
use rome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsFunctionDeclaration;

impl FormatNodeRule<JsFunctionDeclaration> for FormatJsFunctionDeclaration {
    fn fmt_fields(&self, node: &JsFunctionDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [FormatFunction::from(node.clone())]]
    }
}

declare_node_union! {
    pub(crate) FormatFunction =
        JsFunctionDeclaration |
        JsFunctionExpression |
        JsFunctionExportDefaultDeclaration |
        TsDeclareFunctionDeclaration
}

impl FormatFunction {
    fn async_token(&self) -> Option<JsSyntaxToken> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.async_token(),
            FormatFunction::JsFunctionExpression(expression) => expression.async_token(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.async_token()
            }
            FormatFunction::TsDeclareFunctionDeclaration(member) => member.async_token(),
        }
    }

    fn function_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.function_token(),
            FormatFunction::JsFunctionExpression(expression) => expression.function_token(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.function_token()
            }
            FormatFunction::TsDeclareFunctionDeclaration(member) => member.function_token(),
        }
    }

    fn star_token(&self) -> Option<JsSyntaxToken> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.star_token(),
            FormatFunction::JsFunctionExpression(expression) => expression.star_token(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.star_token()
            }
            FormatFunction::TsDeclareFunctionDeclaration(_) => None,
        }
    }

    fn id(&self) -> SyntaxResult<Option<JsAnyBinding>> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.id().map(Some),
            FormatFunction::JsFunctionExpression(expression) => Ok(expression.id()),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => Ok(declaration.id()),
            FormatFunction::TsDeclareFunctionDeclaration(member) => member.id().map(Some),
        }
    }

    fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.type_parameters(),
            FormatFunction::JsFunctionExpression(expression) => expression.type_parameters(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.type_parameters()
            }
            FormatFunction::TsDeclareFunctionDeclaration(member) => member.type_parameters(),
        }
    }

    fn parameters(&self) -> SyntaxResult<JsParameters> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.parameters(),
            FormatFunction::JsFunctionExpression(expression) => expression.parameters(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.parameters()
            }
            FormatFunction::TsDeclareFunctionDeclaration(member) => member.parameters(),
        }
    }

    fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
            FormatFunction::JsFunctionExpression(expression) => expression.return_type_annotation(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
            FormatFunction::TsDeclareFunctionDeclaration(member) => member.return_type_annotation(),
        }
    }

    fn body(&self) -> SyntaxResult<Option<JsFunctionBody>> {
        Ok(match self {
            FormatFunction::JsFunctionDeclaration(declaration) => Some(declaration.body()?),
            FormatFunction::JsFunctionExpression(expression) => Some(expression.body()?),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                Some(declaration.body()?)
            }
            FormatFunction::TsDeclareFunctionDeclaration(_) => None,
        })
    }
}

impl Format<JsFormatContext> for FormatFunction {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        if let Some(async_token) = self.async_token() {
            write!(f, [async_token.format(), space()])?;
        }

        write!(
            f,
            [self.function_token().format(), self.star_token().format()]
        )?;

        match self.id()? {
            Some(id) => {
                write!(f, [space(), id.format()])?;
            }
            None => {
                write!(f, [space()])?;
            }
        }

        let type_parameters = self.type_parameters();
        let parameters = self.parameters()?;
        let return_type_annotation = self.return_type_annotation();

        write!(f, [type_parameters.format()])?;

        write!(
            f,
            [group(&format_with(|f| {
                let mut format_return_type_annotation = return_type_annotation.format().memoized();
                let group_parameters = should_group_function_parameters(
                    type_parameters.as_ref(),
                    parameters.items().len(),
                    return_type_annotation
                        .as_ref()
                        .map(|annotation| annotation.ty()),
                    &mut format_return_type_annotation,
                    f,
                )?;

                if group_parameters {
                    write!(f, [group(&parameters.format())])?;
                } else {
                    write!(f, [parameters.format()])?;
                }

                write![f, [format_return_type_annotation]]
            }))]
        )?;

        if let Some(body) = self.body()? {
            write!(f, [space(), body.format()])?;
        }

        Ok(())
    }
}

/// Returns `true` if the function parameters should be grouped.
/// Grouping the parameters has the effect that the return type will break first.
pub(crate) fn should_group_function_parameters(
    type_parameters: Option<&TsTypeParameters>,
    parameter_count: usize,
    return_type: Option<SyntaxResult<TsAnyReturnType>>,
    formatted_return_type: &mut Memoized<impl Format<JsFormatContext>, JsFormatContext>,
    f: &mut JsFormatter,
) -> FormatResult<bool> {
    let return_type = match return_type {
        Some(return_type) => return_type?,
        None => return Ok(false),
    };

    if let Some(type_parameters) = type_parameters {
        match type_parameters.items().len() {
            0 => {
                // fall through
            }
            1 => {
                // SAFETY: Safe because the length is 1
                let first = type_parameters.items().iter().next().unwrap()?;

                if first.constraint().is_none() || first.default().is_some() {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }
    }

    let result = if parameter_count != 1 {
        false
    } else {
        matches!(
            return_type,
            TsAnyReturnType::TsType(TsType::TsObjectType(_) | TsType::TsMappedType(_))
        ) || formatted_return_type.inspect(f)?.will_break()
    };

    Ok(result)
}
