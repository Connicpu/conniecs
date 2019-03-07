use crate::{improper_attr_format, quote_path, read_path_item};

use syn::{Ident, Lit, Meta, MetaNameValue, NestedMeta};

pub fn quote_aspect(
    ty: &Ident,
    cty: &proc_macro2::TokenStream,
    all_filters: &[Ident],
    none_filters: &[Ident],
) -> proc_macro2::TokenStream {
    quote! {
        impl ::conniecs::aspect::AspectFilter<#cty> for #ty {
            fn check<'a>(&self, entity: ::conniecs::EntityData<'a, #cty >, components: & #cty ) -> bool {
                #(
                    if !components.#all_filters.has(entity) {
                        return false;
                    }
                )*
                #(
                    if components.#none_filters.has(entity) {
                        return false;
                    }
                )*
                true
            }
        }
    }
}

pub fn impl_aspect(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let ty = &ast.ident;
    let mut all_filters = vec![];
    let mut none_filters = vec![];
    let mut components_ty = None;

    for attr in &ast.attrs {
        let meta = attr.parse_meta().unwrap();

        match (attr.path.segments[0].ident.to_string().as_str(), &meta) {
            ("components", meta) => {
                let word = read_path_item(meta, || improper_comp_format());
                components_ty = Some(word);
            }
            ("aspect", Meta::List(list)) => {
                read_aspect(list.nested.iter(), &mut all_filters, &mut none_filters);
            }
            _ => continue,
        }
    }

    let cty = match components_ty {
        Some(ty) => quote_path(&ty),
        None => quote_path("crate::Components"),
    };

    quote_aspect(ty, &cty, &all_filters, &none_filters)
}

pub fn read_aspect_meta<'a>(
    attr: &'a Meta,
    all: &mut Vec<Ident>,
    none: &mut Vec<Ident>,
) -> Option<proc_macro2::TokenStream> {
    match attr {
        Meta::List(list) => {
            read_aspect(list.nested.iter(), all, none);
            None
        }
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(path),
            ..
        }) => Some(quote_path(&path.value())),
        _ => improper_format(),
    }
}

pub fn read_aspect<'a>(
    items: impl IntoIterator<Item = &'a NestedMeta>,
    all: &mut Vec<Ident>,
    none: &mut Vec<Ident>,
) {
    for item in items {
        let item = unwrap_meta(item);
        let items = unwrap_list(item);
        match item.name().to_string().as_str() {
            "all" => {
                for item in items {
                    let item = unwrap_meta(item);
                    let component = unwrap_word(item);
                    all.push(component.clone());
                }
            }
            "none" => {
                for item in items {
                    let item = unwrap_meta(item);
                    let component = unwrap_word(item);
                    none.push(component.clone());
                }
            }
            _ => improper_format(),
        }
    }
}

fn unwrap_list<'a>(item: &'a Meta) -> impl Iterator<Item = &'a NestedMeta> {
    match item {
        Meta::List(list) => list.nested.iter(),
        _ => improper_format(),
    }
}

fn unwrap_word(item: &Meta) -> &Ident {
    match item {
        Meta::Word(ident) => ident,
        _ => improper_format(),
    }
}

fn unwrap_meta(item: &NestedMeta) -> &Meta {
    match item {
        NestedMeta::Meta(item) => item,
        NestedMeta::Literal(_) => improper_format(),
    }
}

fn improper_format() -> ! {
    improper_attr_format("#[aspect(...)]", "conniecs::aspect")
}

fn improper_comp_format() -> ! {
    improper_attr_format("#[components(...)]", "conniecs::aspect")
}
