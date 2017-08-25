use syn::{self, MetaItem, NestedMetaItem, Ident};
use quote;
use {read_path_item, quote_path, improper_attr_format};

pub fn impl_aspect(ast: syn::DeriveInput) -> quote::Tokens {
    let ty = &ast.ident;
    let mut all_filters = vec![];
    let mut none_filters = vec![];
    let mut components_ty = None;

    for attr in &ast.attrs {
        if attr.is_sugared_doc {
            continue;
        }

        match (attr.value.name(), &attr.value) {
            ("components", meta) => {
                let word = read_path_item(meta, || improper_comp_format());
                components_ty = Some(word);
            }
            ("aspect", &MetaItem::List(_, ref items)) => {
                read_aspect(items, &mut all_filters, &mut none_filters);
            }
            _ => continue,
        }
    }

    let cty = match components_ty {
        Some(ty) => quote_path(&ty),
        None => quote_path("::Components"),
    };

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

pub fn read_aspect<'a>(items: &'a [NestedMetaItem], all: &mut Vec<&'a Ident>, none: &mut Vec<&'a Ident>) {
    for item in items {
        let item = unwrap_meta(item);
        let items = unwrap_list(item);
        match item.name() {
            "all" => {
                for item in items {
                    let item = unwrap_meta(item);
                    let component = unwrap_word(item);
                    all.push(component);
                }
            }
            "none" => {
                for item in items {
                    let item = unwrap_meta(item);
                    let component = unwrap_word(item);
                    none.push(component);
                }
            },
            _ => improper_format(),
        }
    }
}

fn unwrap_list(item: &MetaItem) -> &[NestedMetaItem] {
    match item {
        &MetaItem::List(_, ref items) => items,
        _ => improper_format(),
    }
}

fn unwrap_word(item: &MetaItem) -> &Ident {
    match item {
        &MetaItem::Word(ref ident) => ident,
        _ => improper_format(),
    }
}

fn unwrap_meta(item: &NestedMetaItem) -> &MetaItem {
    match item {
        &NestedMetaItem::MetaItem(ref item) => item,
        &NestedMetaItem::Literal(_) => improper_format(),
    }
}

fn improper_format() -> ! {
    improper_attr_format("#[aspect(...)]", "conniecs::aspect")
}

fn improper_comp_format() -> ! {
    improper_attr_format("#[components(...)]", "conniecs::aspect")
}
