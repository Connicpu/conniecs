use syn::{self, MetaItem, NestedMetaItem, Lit};
use quote;
use {read_path_item, improper_attr_format, quote_path};

enum SystemType {
    Basic,
    Entity,
    Lazy,
    Interval,
    Interact,
}

pub fn impl_system(ast: syn::DeriveInput) -> quote::Tokens {
    let mut kind = SystemType::Basic;

    for attr in &ast.attrs {
        match attr.name() {
            "system_type" => kind = read_systy(&attr.value),
            _ => (),
        }
    }

    match kind {
        SystemType::Basic => impl_basic_system(&ast),
        SystemType::Entity => impl_entity_system(&ast),
        SystemType::Lazy => impl_lazy_system(&ast),
        SystemType::Interval => impl_interval_system(&ast),
        SystemType::Interact => impl_interact_system(&ast),
    }
}

fn impl_basic_system(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut cs_data = None;
    let mut init_func = None;
    let mut process_func = None;

    for attr in &ast.attrs {
        match attr.name() {
            "data" => cs_data = Some(read_data(&attr.value)),
            "init" => init_func = Some(read_path_item(&attr.value, || improper_init_fmt())),
            "process" => process_func = Some(read_path_item(&attr.value, || improper_process_fmt())),
            _ => (),
        }
    }

    let (components, services) = match cs_data {
        Some((c, s)) => (c, s),
        None => (quote_path("::Components"), quote_path("::Services")),
    };

    let init = if let Some(init_func) = init_func {
        quote! { #init_func() }
    } else {
        quote! { Default::default() }
    };

    let process = if let Some(proc_func) = process_func {
        let proc_func = quote_path(&proc_func);
        quote! {
            impl ::conniecs::system::Process for #name {
                fn process(&mut self, data: &mut ::conniecs::DataHelper<Self::Components, Self::Services>) {
                    #proc_func(self, data);
                }
            }
        }
    } else {
        quote!{}
    };

    quote! {
        impl ::conniecs::system::System for #name {
            type Components = #components;
            type Services = #services;

            fn build_system() -> Self {
                #init
            }
        }
        
        #process
    }
}

fn impl_entity_system(ast: &syn::DeriveInput) -> quote::Tokens {
    let _todo = ast;
    quote!{}
}

fn impl_lazy_system(ast: &syn::DeriveInput) -> quote::Tokens {
    let _todo = ast;
    quote!{}
}

fn impl_interval_system(ast: &syn::DeriveInput) -> quote::Tokens {
    let _todo = ast;
    quote!{}
}

fn impl_interact_system(ast: &syn::DeriveInput) -> quote::Tokens {
    let _todo = ast;
    quote!{}
}

fn read_systy(attr: &MetaItem) -> SystemType {
    let systy = read_path_item(attr, || improper_systy_fmt());
    match &systy[..] {
        "basic" => SystemType::Basic,
        "entity" => SystemType::Entity,
        "lazy" => SystemType::Lazy,
        "interval" => SystemType::Interval,
        "interact" => SystemType::Interact,
        _ => improper_systy_fmt(),
    }
}

pub fn read_data(item: &MetaItem) -> (quote::Tokens, quote::Tokens) {
    match item {
        &MetaItem::List(_, ref items) => read_data_items(items),
        _ => improper_data_fmt(),
    }
}

pub fn read_data_items(items: &[NestedMetaItem]) -> (quote::Tokens, quote::Tokens) {
    if items.len() != 2 {
        improper_data_fmt();
    }

    let comps = match items[0] {
        NestedMetaItem::Literal(Lit::Str(ref path, _)) => path.clone(),
        NestedMetaItem::MetaItem(MetaItem::Word(ref word)) => word.to_string(),
        NestedMetaItem::MetaItem(MetaItem::NameValue(ref comps, Lit::Str(ref path, _))) if comps == "components" => path.clone(),
        _ => improper_data_fmt(),
    };

    let servs = match items[1] {
        NestedMetaItem::Literal(Lit::Str(ref path, _)) => path.clone(),
        NestedMetaItem::MetaItem(MetaItem::Word(ref word)) => word.to_string(),
        NestedMetaItem::MetaItem(MetaItem::NameValue(ref comps, Lit::Str(ref path, _))) if comps == "services" => path.clone(),
        _ => improper_data_fmt(),
    };

    (quote_path(&comps), quote_path(&servs))
}

fn improper_systy_fmt() -> ! {
    improper_attr_format("#[system_type(...)]", "conniecs::system");
}

fn improper_data_fmt() -> ! {
    improper_attr_format("#[data(...)]", "conniecs::system");
}

fn improper_init_fmt() -> ! {
    improper_attr_format("#[init(...)]", "conniecs::system");
}

fn improper_process_fmt() -> ! {
    improper_attr_format("#[process(...)]", "conniecs::system");
}
