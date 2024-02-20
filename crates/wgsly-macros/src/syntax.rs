//! Syntax definitions and ops.
use naga::Module;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

pub fn error<T>(t: &T, msg: &str) -> syn::Error
where
    T: syn::spanned::Spanned,
{
    syn::Error::new(t.span(), format!("wgsly: {msg}"))
}

pub fn validate_source(module: &Module) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut validator =
        naga::valid::Validator::new(Default::default(), naga::valid::Capabilities::empty());
    match validator.validate(&module) {
        Err(e) => Err(error(&proc_macro2::Span::call_site(), &e.to_string())),
        Ok(info) => {
            match naga::back::wgsl::write_string(
                &module,
                &info,
                naga::back::wgsl::WriterFlags::empty(),
            ) {
                Err(e) => Err(error(&proc_macro2::Span::call_site(), &e.to_string())),
                Ok(source) => Ok(quote! { #source }),
            }
        }
    }
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
        t => Err(syn::Error::new(
            t.span(),
            format!("wgsly: Unsupported type '{displayed}'",),
        )),
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

pub enum WgslItem {
    Const(WgslConst),
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
