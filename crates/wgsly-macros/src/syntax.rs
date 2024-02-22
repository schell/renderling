//! Syntax definitions and ops.
use std::collections::VecDeque;

use naga::Module;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, visit::Visit};

/// Inner portion of an attribute like `0` in `@group(0)`.
enum AttrField {
    Index(syn::LitInt),
    Named(syn::Ident),
}

/// Attribute like `@group(0)` or `@builtin(position)`.
struct Attr {
    attr_token: syn::Token![@],
    ident: syn::Ident,
    parent_token: syn::token::Paren,
    inner: AttrField,
}

struct SpanVisitor {
    target_byte_range: std::ops::Range<usize>,
    closest_span: proc_macro2::Span,
}

impl SpanVisitor {
    pub fn new(source_span_start: u32, source_span_length: u32) -> Self {
        let closest_span = proc_macro2::Span::call_site();
        let closest_byte_range = closest_span.byte_range();
        let target_start = closest_byte_range.start + source_span_start as usize;
        let target_end = target_start + source_span_length as usize;
        let target_byte_range = target_start..target_end;
        Self {
            target_byte_range,
            closest_span,
        }
    }

    pub fn distance_to_span(&self, span: &proc_macro2::Span) -> usize {
        let range = span.byte_range();
        let distance_to_start = self.target_byte_range.start.max(range.start)
            - self.target_byte_range.start.min(range.start);
        let distance_to_end =
            self.target_byte_range.end.max(range.end) - self.target_byte_range.start.min(range.end);
        distance_to_start + distance_to_end
    }

    pub fn distance_to_closest_span(&self) -> usize {
        self.distance_to_span(&self.closest_span)
    }
}

impl<'ast> Visit<'ast> for SpanVisitor {
    fn visit_span(&mut self, i: &proc_macro2::Span) {
        if self.distance_to_span(i) < self.distance_to_closest_span() {
            self.closest_span = i.clone();
        }
        syn::visit::visit_span(self, i);
    }
}

pub fn error<T>(t: &T, msg: &str) -> syn::Error
where
    T: syn::spanned::Spanned,
{
    syn::Error::new(t.span(), format!("wgsly: {msg}"))
}

pub fn parse_source(input: proc_macro2::TokenStream) -> Result<naga::Module, syn::Error> {
    let source = input.clone().to_string();
    match naga::front::wgsl::parse_str(&source) {
        Ok(m) => Ok(m),
        Err(e) => {
            let file = syn::parse2::<syn::File>(input)
                .map_err(|e| syn::Error::new(e.span(), format!("rust: {}", e.to_string())))?;
            let mut errors = e
                .labels()
                .map(|(naga_span, msg)| {
                    let location = naga_span.location(&source);
                    let mut visitor = SpanVisitor::new(location.offset, location.length);
                    visitor.visit_file(&file);
                    let span = visitor.closest_span;
                    syn::Error::new(span, msg)
                })
                .collect::<VecDeque<_>>();
            let error = errors.pop_front().unwrap_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("parse error: {}", e.message()),
                )
            });
            let error = errors.into_iter().fold(error, |mut acc_err, err| {
                acc_err.combine(err);
                acc_err
            });
            Err(error)
        }
    }
}

pub fn validate_source(module: &Module) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut validator =
        naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
    let info = validator
        .validate(&module)
        .map_err(|e| error(&proc_macro2::Span::call_site(), &e.to_string()))?;
    let source =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .map_err(|e| error(&proc_macro2::Span::call_site(), &e.to_string()))?;
    Ok(quote! { #source })
}

pub fn src(input: proc_macro::TokenStream) -> Result<proc_macro2::TokenStream, syn::Error> {
    let m = parse_source(input.into())?;
    let tokens = validate_source(&m)?;
    Ok(tokens)
}

pub fn module_fn(
    ident: &syn::Ident,
    ty: &syn::Type,
    m: &Module,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let fragment_ident = syn::Ident::new(&format!("{ident}__wgsl"), ident.span());
    let source = validate_source(&m)?;
    Ok(quote! {
        #[allow(non_snake_case)]
        pub fn #fragment_ident() -> wgsly::Wgsl<#ty> {
            wgsly::Wgsl::new(#source)
        }
    }
    .into())
}

pub fn rust_span_to_naga_span(span: proc_macro2::Span) -> naga::Span {
    let range = span.byte_range();
    let start = range.start;
    let end = range.end;
    naga::Span::new(start as u32, end as u32)
}

pub fn rust_ty_to_naga_ty(rust_ty: syn::Type) -> Result<naga::Type, syn::Error> {
    let displayed = rust_ty.clone().into_token_stream().to_string();
    match displayed.as_str() {
        "u32" => Ok(naga::Type {
            name: None,
            inner: naga::TypeInner::Scalar(naga::Scalar {
                kind: naga::ScalarKind::Uint,
                width: 4,
            }),
        }),
        t => Err(error(&t, &format!("Unsupported type '{displayed}'"))),
    }
}

pub fn rust_expr_to_naga_expr(
    ty: &naga::Type,
    rust_expr: syn::Expr,
) -> Result<naga::Expression, syn::Error> {
    match &rust_expr {
        syn::Expr::Lit(syn::ExprLit {
            attrs: _,
            lit: syn::Lit::Int(i),
        }) => {
            let scalar_kind = ty
                .inner
                .scalar_kind()
                .ok_or_else(|| error(&rust_expr, "Expected an integer"))?;
            let width = ty
                .inner
                .scalar_width()
                .ok_or_else(|| error(&rust_expr, "Missing scalar width"))?;

            Ok(naga::Expression::Literal(match (scalar_kind, width) {
                (naga::ScalarKind::Sint, 32) => naga::Literal::I32(i.base10_parse()?),
                (naga::ScalarKind::Sint, 64) => naga::Literal::I64(i.base10_parse()?),
                (naga::ScalarKind::Uint, 32) => naga::Literal::U32(i.base10_parse()?),
                (k, w) => {
                    return Err(error(
                        &rust_expr,
                        &format!("Unsupported scalar kind '{k:?}' or width '{w}'"),
                    ))
                }
            }))
        }
        e => Err(syn::Error::new(
            e.span(),
            format!(
                "Unsupported expression '{}': {e:#?}",
                e.clone().into_token_stream()
            ),
        )),
    }
}

pub struct WgslConst {
    pub rust: syn::ItemConst,
}

impl syn::parse::Parse for WgslConst {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let rust = syn::ItemConst::parse(input)?;
        WgslConst::try_from(rust)
    }
}

impl TryFrom<syn::ItemConst> for WgslConst {
    type Error = syn::Error;

    fn try_from(rust: syn::ItemConst) -> Result<Self, Self::Error> {
        let constant = WgslConst { rust };
        let _module = constant.to_naga()?;
        Ok(constant)
    }
}

impl WgslConst {
    pub fn name(&self) -> String {
        self.rust.ident.to_string()
    }

    pub fn to_naga(&self) -> Result<naga::Module, syn::Error> {
        let mut m = Module::default();
        let name = Some(self.name());
        let ty = rust_ty_to_naga_ty(*(self.rust.ty).clone())?;
        let init = rust_expr_to_naga_expr(&ty, *(self.rust.expr).clone())?;
        let ty = m
            .types
            .insert(ty, rust_span_to_naga_span(self.rust.ty.span()));
        let init = m
            .const_expressions
            .fetch_or_append(init, rust_span_to_naga_span(self.rust.expr.span()));
        let constant = naga::Constant {
            name,
            r#override: naga::Override::None,
            ty,
            init,
        };
        m.constants
            .fetch_or_append(constant, rust_span_to_naga_span(self.rust.span()));
        Ok(m)
    }
}

pub fn wgsl_const(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let c = syn::parse::<WgslConst>(input.clone())?;
    let m = c.to_naga()?;
    let ident = c.rust.ident;
    let ty = c.rust.ty;
    let f = module_fn(&ident, &ty, &m)?;
    let input = proc_macro2::TokenStream::from(input);
    Ok(quote! {
        #input
        #f
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn naga_parse() {
        // sandbox for parsing things and seeing what it is
        let module = naga::front::wgsl::parse_str("let the_iterations: u32 = 16;").unwrap();
        panic!("module: {module:#?}");
    }

    fn err_nice<T, E: std::fmt::Display>(r: Result<T, E>) -> T {
        match r {
            Ok(t) => t,
            Err(e) => panic!("error: {e}"),
        }
    }

    #[test]
    fn const_roundtrip() {
        let decl = "const MAX_ITERATIONS: u32 = 16;";
        let c = syn::parse_str::<crate::WgslConst>(decl).unwrap();
        let seen_m = c.to_naga().unwrap();
        let expected_m = naga::front::wgsl::parse_str(decl).unwrap();
        pretty_assertions::assert_str_eq!(format!("{expected_m:#?}"), format!("{seen_m:#?}"));
        let mut validator =
            naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
        let info = err_nice(validator.validate(&seen_m));
        // let result_src =
        //     naga::back::wgsl::write_string(&m, &info, naga::back::wgsl::WriterFlags::empty());
        // assert_eq!(decl, result_src.unwrap());
    }
}
