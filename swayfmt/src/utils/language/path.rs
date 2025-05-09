use crate::{
    formatter::*,
    utils::{
        map::byte_span::{ByteSpan, LeafSpans},
        {close_angle_bracket, open_angle_bracket},
    },
};
use std::{fmt::Write, vec};
use sway_ast::{
    keywords::{AsToken, Keyword, Token},
    DoubleColonToken, PathExpr, PathExprSegment, PathType, PathTypeSegment, QualifiedPathRoot,
};
use sway_types::Spanned;

impl Format for PathExpr {
    fn format(
        &self,
        formatted_code: &mut FormattedCode,
        formatter: &mut Formatter,
    ) -> Result<(), FormatterError> {
        if let Some((qualified_path_root, _double_colon_token)) = &self.root_opt {
            if let Some(root) = &qualified_path_root {
                open_angle_bracket(formatted_code)?;
                root.clone()
                    .into_inner()
                    .format(formatted_code, formatter)?;
                close_angle_bracket(formatted_code)?;
            }
            write!(formatted_code, "{}", DoubleColonToken::AS_STR)?;
        }
        self.prefix.format(formatted_code, formatter)?;
        for (_double_colon_token, path_expr_segment) in self.suffix.iter() {
            write!(formatted_code, "{}", DoubleColonToken::AS_STR)?;
            path_expr_segment.format(formatted_code, formatter)?;
        }

        Ok(())
    }
}

impl Format for PathExprSegment {
    fn format(
        &self,
        formatted_code: &mut FormattedCode,
        formatter: &mut Formatter,
    ) -> Result<(), FormatterError> {
        // name
        self.name.format(formatted_code, formatter)?;
        // generics `::<args>`
        if let Some((_double_colon_token, generic_args)) = &self.generics_opt {
            write!(formatted_code, "{}", DoubleColonToken::AS_STR)?;
            generic_args.format(formatted_code, formatter)?;
        }

        Ok(())
    }
}

impl Format for QualifiedPathRoot {
    fn format(
        &self,
        formatted_code: &mut FormattedCode,
        formatter: &mut Formatter,
    ) -> Result<(), FormatterError> {
        self.ty.format(formatted_code, formatter)?;
        let (_as_token, path_type) = &self.as_trait;
        write!(formatted_code, " {} ", AsToken::AS_STR)?;
        path_type.format(formatted_code, formatter)?;

        Ok(())
    }
}

impl Format for PathType {
    fn format(
        &self,
        formatted_code: &mut FormattedCode,
        formatter: &mut Formatter,
    ) -> Result<(), FormatterError> {
        if let Some((root_opt, _double_colon_token)) = &self.root_opt {
            if let Some(qualified_path_root) = &root_opt {
                open_angle_bracket(formatted_code)?;
                qualified_path_root
                    .clone()
                    .into_inner()
                    .format(formatted_code, formatter)?;
                close_angle_bracket(formatted_code)?;
            }
            write!(formatted_code, "{}", DoubleColonToken::AS_STR)?;
        }
        self.prefix.format(formatted_code, formatter)?;
        for (_double_colon_token, path_type_segment) in self.suffix.iter() {
            write!(formatted_code, "{}", DoubleColonToken::AS_STR)?;
            path_type_segment.format(formatted_code, formatter)?;
        }

        Ok(())
    }
}

impl Format for PathTypeSegment {
    fn format(
        &self,
        formatted_code: &mut FormattedCode,
        formatter: &mut Formatter,
    ) -> Result<(), FormatterError> {
        // name
        write!(formatted_code, "{}", self.name.as_str())?;
        // generics `::<args>`
        if let Some((double_colon_opt, generic_args)) = &self.generics_opt {
            if double_colon_opt.is_some() {
                write!(formatted_code, "{}", DoubleColonToken::AS_STR)?;
            }
            generic_args.format(formatted_code, formatter)?;
        }

        Ok(())
    }
}

impl LeafSpans for PathExpr {
    fn leaf_spans(&self) -> Vec<ByteSpan> {
        vec![ByteSpan::from(self.span())]
    }
}

impl LeafSpans for PathType {
    fn leaf_spans(&self) -> Vec<ByteSpan> {
        vec![ByteSpan::from(self.span())]
    }
}
