use syn::{self, MetaItem, NestedMetaItem, Lit, Ident};
use quote;
use {read_path_item, improper_attr_format, quote_path};
use aspect::{read_aspect_meta, quote_aspect};

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
    let name = &ast.ident;
    let mut cs_data = None;
    let mut init_func = None;
    let mut process_func = None;
    let mut aspect_all = vec![];
    let mut aspect_none = vec![];

    let aspect_id = Ident::new(format!("{}EntityAspect", name));
    let mut aspect_path = None;

    for attr in &ast.attrs {
        match attr.name() {
            "data" => cs_data = Some(read_data(&attr.value)),
            "init" => init_func = Some(read_path_item(&attr.value, || improper_init_fmt())),
            "process" => process_func = Some(read_path_item(&attr.value, || improper_process_fmt())),
            "aspect" => aspect_path = read_aspect_meta(&attr.value, &mut aspect_all, &mut aspect_none),
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

    let datahelper = quote! { ::conniecs::world::DataHelper<Self::Components, Self::Services> };
    let entiter = quote! { ::conniecs::entity::EntityIter<Self::Components> };

    let process = if let Some(proc_func) = process_func {
        let proc_func = quote_path(&proc_func);
        quote! {
            impl ::conniecs::system::entity::EntityProcess for #name {
                fn process(&mut self, entities: #entiter, data: &mut #datahelper) {
                    #proc_func(self, entities, data);
                }
            }
        }
    } else {
        quote!{}
    };

    let (aspect, aspect_id) = if let Some(aspect_path) = aspect_path {
        (quote!{}, aspect_path)
    } else {
        let aspect = quote_aspect(&aspect_id, &components, &aspect_all, &aspect_none);
        let aspect = quote! { #[derive(Copy, Clone, Debug)] pub struct #aspect_id; #aspect };
        (aspect, quote! { #aspect_id })
    };

    let filterdef = quote! {
        impl ::conniecs::system::entity::FilteredEntitySystem for #name {
            fn create_aspect() -> ::conniecs::aspect::Aspect<Self::Components> {
                ::conniecs::aspect::Aspect::new( #aspect_id )
            }
        }
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
        #aspect
        #filterdef
    }
}

fn impl_lazy_system(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut cs_data = None;
    let mut process_func = None;

    for attr in &ast.attrs {
        match attr.name() {
            "data" => cs_data = Some(read_data(&attr.value)),
            "process" => process_func = Some(read_path_item(&attr.value, || improper_process_fmt())),
            _ => (),
        }
    }

    let (components, services) = match cs_data {
        Some((c, s)) => (c, s),
        None => (quote_path("::Components"), quote_path("::Services")),
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
                unimplemented!()
            }
        }
        
        #process
    }
}

fn impl_interval_system(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut cs_data = None;
    let mut init_func = None;
    let mut process_func = None;
    let mut interval = None;

    for attr in &ast.attrs {
        match attr.name() {
            "data" => cs_data = Some(read_data(&attr.value)),
            "init" => init_func = Some(read_path_item(&attr.value, || improper_init_fmt())),
            "process" => process_func = Some(read_path_item(&attr.value, || improper_process_fmt())),
            "interval" => interval = Some(parse_interval(&attr.value)),
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

    let iv = interval.unwrap_or_else(|| panic!("#[interval = ...] attribute must be specified"));

    let create_interval = quote! {
        impl ::conniecs::system::interval::SystemInterval for #name {
            fn create_interval() -> ::conniecs::system::interval::TickerState {
                #iv
            }
        }
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
        #create_interval
    }
}

fn parse_interval(attr: &MetaItem) -> quote::Tokens {
    match attr {
        &MetaItem::NameValue(_, Lit::Str(ref time, _)) => parse_iv_time(time),
        &MetaItem::NameValue(_, Lit::Int(time, _)) => frame_iv(time),
        &MetaItem::List(_, ref items) => parse_explicit_iv(items),
        _ => improper_interval_fmt(),
    }
}

fn parse_u64(time: &str) -> u64 {
    time.parse().map_err(|_| improper_interval_fmt()).unwrap()
}

fn parse_iv_time(time: &str) -> quote::Tokens {
    let len = time.len();

    let ns = if time.ends_with("ms") && len > 2 {
        let iv = parse_u64(&time[..len - 2]);
        iv * 1_000_000
    } else if (time.ends_with("us") || time.ends_with("μs")) && len > 2 {
        let iv = parse_u64(&time[..len - 2]);
        iv * 1_000
    } else if time.ends_with("ns") && len > 2 {
        let iv = parse_u64(&time[..len - 2]);
        iv
    } else if time.ends_with("s") && len > 1 {
        let iv = parse_u64(&time[..len - 1]);
        iv * 1_000_000_000
    } else {
        let iv = parse_u64(time);
        return frame_iv(iv);
    };

    frame_ns(ns)
}

fn parse_explicit_iv(items: &[NestedMetaItem]) -> quote::Tokens {
    if items.len() != 1 { improper_interval_fmt(); }

    match items[0] {
        NestedMetaItem::Literal(Lit::Str(ref time, _)) => parse_iv_time(time),
        NestedMetaItem::Literal(Lit::Int(time, _)) => frame_iv(time),
        NestedMetaItem::MetaItem(MetaItem::NameValue(ref kind, ref lit)) => {
            let iv = match lit {
                &Lit::Str(ref time, _) => parse_u64(time),
                &Lit::Int(iv, _) => iv,
                _ => improper_interval_fmt(),
            };
            let ns = match kind.as_ref() {
                "ticks" => return frame_iv(iv),

                "s" => iv * 1_000_000_000,
                "ms" => iv * 1_000_000,
                "us" | "μs" => iv * 1_000,
                "ns" => iv,

                _ => improper_interval_fmt(),
            };

            frame_ns(ns)
        }
        _ => improper_interval_fmt(),
    }
}

fn frame_iv(iv: u64) -> quote::Tokens {
    quote! {
        ::conniecs::system::interval::TickerState::Frames {
            interval: #iv,
            ticks: 0,
        }
    }
}

fn frame_ns(ns: u64) -> quote::Tokens {
    quote! {
        ::conniecs::system::interval::TickerState::Timed {
            interval: #ns,
            next_tick: None,
        }
    }
}

fn impl_interact_system(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut cs_data = None;
    let mut init_func = None;
    let mut process_func = None;
    let mut aspect_all_a = vec![];
    let mut aspect_none_a = vec![];
    let mut aspect_all_b = vec![];
    let mut aspect_none_b = vec![];

    let aspect_id_a = Ident::new(format!("{}EntityAspectA", name));
    let mut aspect_path_a = None;
    let aspect_id_b = Ident::new(format!("{}EntityAspectB", name));
    let mut aspect_path_b = None;

    for attr in &ast.attrs {
        match attr.name() {
            "data" => cs_data = Some(read_data(&attr.value)),
            "init" => init_func = Some(read_path_item(&attr.value, || improper_init_fmt())),
            "process" => process_func = Some(read_path_item(&attr.value, || improper_process_fmt())),
            "aspect_a" => aspect_path_a = read_aspect_meta(&attr.value, &mut aspect_all_a, &mut aspect_none_a),
            "aspect_b" => aspect_path_b = read_aspect_meta(&attr.value, &mut aspect_all_b, &mut aspect_none_b),
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

    let datahelper = quote! { ::conniecs::world::DataHelper<Self::Components, Self::Services> };
    let entiter = quote! { ::conniecs::entity::EntityIter<Self::Components> };

    let process = if let Some(proc_func) = process_func {
        let proc_func = quote_path(&proc_func);
        quote! {
            impl ::conniecs::system::interact::InteractProcess for #name {
                fn process(&mut self, ea: #entiter, eb: #entiter, data: &mut #datahelper) {
                    #proc_func(self, ea, eb, data);
                }
            }
        }
    } else {
        quote!{}
    };

    let (aspect_a, aspect_id_a) = if let Some(aspect_path_a) = aspect_path_a {
        (quote!{}, aspect_path_a)
    } else {
        let aspect_a = quote_aspect(&aspect_id_a, &components, &aspect_all_a, &aspect_none_a);
        let aspect_a = quote! { #[derive(Copy, Clone, Debug)] pub struct #aspect_id_a; #aspect_a };
        (aspect_a, quote! { #aspect_id_a })
    };

    let (aspect_b, aspect_id_b) = if let Some(aspect_path_b) = aspect_path_b {
        (quote!{}, aspect_path_b)
    } else {
        let aspect_b = quote_aspect(&aspect_id_b, &components, &aspect_all_b, &aspect_none_b);
        let aspect_b = quote! { #[derive(Copy, Clone, Debug)] pub struct #aspect_id_b; #aspect_b };
        (aspect_b, quote! { #aspect_id_b })
    };

    let filterdef = quote! {
        impl ::conniecs::system::interact::InteractSystemFilter for #name {
            fn create_filter_a() -> ::conniecs::aspect::Aspect<Self::Components> {
                ::conniecs::aspect::Aspect::new( #aspect_id_a )
            }
            fn create_filter_b() -> ::conniecs::aspect::Aspect<Self::Components> {
                ::conniecs::aspect::Aspect::new( #aspect_id_b )
            }
        }
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
        #aspect_a
        #aspect_b
        #filterdef
    }
}

fn read_systy(attr: &MetaItem) -> SystemType {
    let systy = read_path_item(attr, || improper_systy_fmt());
    match &systy[..] {
        "Basic"    | "basic"    => SystemType::Basic,
        "Entity"   | "entity"   => SystemType::Entity,
        "Lazy"     | "lazy"     => SystemType::Lazy,
        "Interval" | "interval" => SystemType::Interval,
        "Interact" | "interact" => SystemType::Interact,
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

fn improper_interval_fmt() -> ! {
    improper_attr_format("#[interval = ...]", "conniecs::system");
}
