use crate::aspect::{quote_aspect, read_aspect_meta};
use crate::{improper_attr_format, quote_path, read_path_item};

use proc_macro2::Span;
use syn::{self, Attribute, Ident, Lit, Meta, MetaNameValue, NestedMeta};

enum SystemType {
    Basic,
    Entity,
    Lazy,
    Interval,
    Interact,
}

pub fn impl_system(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let mut kind = SystemType::Basic;

    for attr in &ast.attrs {
        let meta = attr.parse_meta().unwrap();
        match meta.name().to_string().as_str() {
            "system_type" => kind = read_systy(&meta),
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

fn impl_basic_system(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let mut cs_data = None;
    let mut init_func = None;
    let mut process_func = None;

    for attr in &ast.attrs {
        let meta = attr.parse_meta().unwrap();
        match meta.name().to_string().as_str() {
            "data" => cs_data = Some(read_data(&meta)),
            "init" => init_func = Some(read_path_item(&meta, || improper_init_fmt())),
            "process" => process_func = Some(read_path_item(&meta, || improper_process_fmt())),
            _ => (),
        }
    }

    let (components, services) = match cs_data {
        Some((c, s)) => (c, s),
        None => (quote_path("crate::Components"), quote_path("crate::Services")),
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
        quote! {}
    };

    let activations = read_activations(&ast.attrs);

    quote! {
        impl ::conniecs::system::System for #name {
            type Components = #components;
            type Services = #services;

            fn build_system() -> Self {
                #init
            }

            #activations
        }

        #process
    }
}

fn impl_entity_system(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let mut cs_data = None;
    let mut init_func = None;
    let mut process_func = None;
    let mut aspect_all = vec![];
    let mut aspect_none = vec![];

    let aspect_id = Ident::new(&format!("{}EntityAspect", name), Span::call_site());
    let mut aspect_path = None;

    for attr in &ast.attrs {
        let meta = attr.parse_meta().unwrap();
        match meta.name().to_string().as_str() {
            "data" => cs_data = Some(read_data(&meta)),
            "init" => init_func = Some(read_path_item(&meta, || improper_init_fmt())),
            "process" => process_func = Some(read_path_item(&meta, || improper_process_fmt())),
            "aspect" => aspect_path = read_aspect_meta(&meta, &mut aspect_all, &mut aspect_none),
            _ => (),
        }
    }

    let (components, services) = match cs_data {
        Some((c, s)) => (c, s),
        None => (quote_path("crate::Components"), quote_path("crate::Services")),
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
        quote! {}
    };

    let (aspect, aspect_id) = if let Some(aspect_path) = aspect_path {
        (quote! {}, aspect_path)
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

    let activations = read_activations(&ast.attrs);

    quote! {
        impl ::conniecs::system::System for #name {
            type Components = #components;
            type Services = #services;

            fn build_system() -> Self {
                #init
            }

            #activations
        }

        #process
        #aspect
        #filterdef
    }
}

fn impl_lazy_system(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let mut cs_data = None;
    let mut process_func = None;

    for attr in &ast.attrs {
        let meta = attr.parse_meta().unwrap();
        match meta.name().to_string().as_str() {
            "data" => cs_data = Some(read_data(&meta)),
            "process" => process_func = Some(read_path_item(&meta, || improper_process_fmt())),
            _ => (),
        }
    }

    let (components, services) = match cs_data {
        Some((c, s)) => (c, s),
        None => (quote_path("crate::Components"), quote_path("crate::Services")),
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
        quote! {}
    };

    let activations = read_activations(&ast.attrs);

    quote! {
        impl ::conniecs::system::System for #name {
            type Components = #components;
            type Services = #services;

            fn build_system() -> Self {
                unimplemented!()
            }

            #activations
        }

        #process
    }
}

fn impl_interval_system(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let mut cs_data = None;
    let mut init_func = None;
    let mut process_func = None;
    let mut interval = None;

    for attr in &ast.attrs {
        let meta = attr.parse_meta().unwrap();
        match meta.name().to_string().as_str() {
            "data" => cs_data = Some(read_data(&meta)),
            "init" => init_func = Some(read_path_item(&meta, || improper_init_fmt())),
            "process" => process_func = Some(read_path_item(&meta, || improper_process_fmt())),
            "interval" => interval = Some(parse_interval(&meta)),
            _ => (),
        }
    }

    let (components, services) = match cs_data {
        Some((c, s)) => (c, s),
        None => (quote_path("crate::Components"), quote_path("crate::Services")),
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
        quote! {}
    };

    let iv = interval.unwrap_or_else(|| panic!("#[interval = ...] attribute must be specified"));

    let create_interval = quote! {
        impl ::conniecs::system::interval::SystemInterval for #name {
            fn create_interval() -> ::conniecs::system::interval::TickerState {
                #iv
            }
        }
    };

    let activations = read_activations(&ast.attrs);

    quote! {
        impl ::conniecs::system::System for #name {
            type Components = #components;
            type Services = #services;

            fn build_system() -> Self {
                #init
            }

            #activations
        }

        #process
        #create_interval
    }
}

fn parse_interval(attr: &Meta) -> proc_macro2::TokenStream {
    match attr {
        Meta::NameValue(mnv) => match &mnv.lit {
            Lit::Str(time) => parse_iv_time(&time.value()),
            Lit::Int(time) => frame_iv(time.value()),
            _ => improper_interval_fmt(),
        },
        Meta::List(list) => parse_explicit_iv(list.nested.iter()),
        _ => improper_interval_fmt(),
    }
}

fn parse_u64(time: &str) -> u64 {
    time.parse().map_err(|_| improper_interval_fmt()).unwrap()
}

fn parse_iv_time(time: &str) -> proc_macro2::TokenStream {
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

fn parse_explicit_iv<'a>(
    mut items: impl Iterator<Item = &'a NestedMeta>,
) -> proc_macro2::TokenStream {
    let item = items.next();
    if item.is_none() || items.next().is_some() {
        improper_interval_fmt();
    }

    match item.unwrap() {
        NestedMeta::Literal(Lit::Str(time)) => parse_iv_time(&time.value()),
        NestedMeta::Literal(Lit::Int(time)) => frame_iv(time.value()),
        NestedMeta::Meta(Meta::NameValue(mnv)) => {
            let iv = match &mnv.lit {
                Lit::Str(time) => parse_u64(&time.value()),
                Lit::Int(iv) => iv.value(),
                _ => improper_interval_fmt(),
            };
            let ns = match mnv.ident.to_string().as_str() {
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

fn frame_iv(iv: u64) -> proc_macro2::TokenStream {
    quote! {
        ::conniecs::system::interval::TickerState::Frames {
            interval: #iv,
            ticks: 0,
        }
    }
}

fn frame_ns(ns: u64) -> proc_macro2::TokenStream {
    quote! {
        ::conniecs::system::interval::TickerState::Timed {
            interval: #ns,
            next_tick: None,
        }
    }
}

fn impl_interact_system(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let mut cs_data = None;
    let mut init_func = None;
    let mut process_func = None;
    let mut aspect_all_a = vec![];
    let mut aspect_none_a = vec![];
    let mut aspect_all_b = vec![];
    let mut aspect_none_b = vec![];

    let aspect_id_a = Ident::new(&format!("{}EntityAspectA", name), Span::call_site());
    let mut aspect_path_a = None;
    let aspect_id_b = Ident::new(&format!("{}EntityAspectB", name), Span::call_site());
    let mut aspect_path_b = None;

    for attr in &ast.attrs {
        let meta = attr.parse_meta().unwrap();
        match meta.name().to_string().as_str() {
            "data" => cs_data = Some(read_data(&meta)),
            "init" => init_func = Some(read_path_item(&meta, || improper_init_fmt())),
            "process" => process_func = Some(read_path_item(&meta, || improper_process_fmt())),
            "aspect_a" => {
                aspect_path_a = read_aspect_meta(&meta, &mut aspect_all_a, &mut aspect_none_a)
            }
            "aspect_b" => {
                aspect_path_b = read_aspect_meta(&meta, &mut aspect_all_b, &mut aspect_none_b)
            }
            _ => (),
        }
    }

    let (components, services) = match cs_data {
        Some((c, s)) => (c, s),
        None => (quote_path("crate::Components"), quote_path("crate::Services")),
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
        quote! {}
    };

    let (aspect_a, aspect_id_a) = if let Some(aspect_path_a) = aspect_path_a {
        (quote! {}, aspect_path_a)
    } else {
        let aspect_a = quote_aspect(&aspect_id_a, &components, &aspect_all_a, &aspect_none_a);
        let aspect_a = quote! { #[derive(Copy, Clone, Debug)] pub struct #aspect_id_a; #aspect_a };
        (aspect_a, quote! { #aspect_id_a })
    };

    let (aspect_b, aspect_id_b) = if let Some(aspect_path_b) = aspect_path_b {
        (quote! {}, aspect_path_b)
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

    let activations = read_activations(&ast.attrs);

    quote! {
        impl ::conniecs::system::System for #name {
            type Components = #components;
            type Services = #services;

            fn build_system() -> Self {
                #init
            }

            #activations
        }

        #process
        #aspect_a
        #aspect_b
        #filterdef
    }
}

fn read_activations(attrs: &[Attribute]) -> proc_macro2::TokenStream {
    let mut activated = None;
    let mut reactivated = None;
    let mut deactivated = None;

    for attr in attrs {
        let meta = attr.parse_meta().unwrap();
        match meta.name().to_string().as_str() {
            "activated" => activated = Some(read_path_item(&meta, || improper_activated_fmt())),
            "reactivated" => {
                reactivated = Some(read_path_item(&meta, || improper_reactivated_fmt()))
            }
            "deactivated" => {
                deactivated = Some(read_path_item(&meta, || improper_deactivated_fmt()))
            }
            _ => (),
        }
    }

    let activated = activation_fn(
        Ident::new("activated", Span::call_site()),
        activated.map(|s| quote_path(&s)),
    );
    let reactivated = activation_fn(
        Ident::new("reactivated", Span::call_site()),
        reactivated.map(|s| quote_path(&s)),
    );
    let deactivated = activation_fn(
        Ident::new("deactivated", Span::call_site()),
        deactivated.map(|s| quote_path(&s)),
    );

    quote! {
        #activated
        #reactivated
        #deactivated
    }
}

fn activation_fn(name: Ident, item: Option<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    if let Some(item) = item {
        quote! {
            fn #name (
                &mut self,
                entity: ::conniecs::entity::EntityData<Self::Components>,
                components: &Self::Components,
                services: &mut Self::Services,
            ) {
                #item (self, entity, components, services)
            }
        }
    } else {
        quote! {}
    }
}

fn read_systy(attr: &Meta) -> SystemType {
    let systy = read_path_item(attr, || improper_systy_fmt());
    match &systy[..] {
        "Basic" | "basic" => SystemType::Basic,
        "Entity" | "entity" => SystemType::Entity,
        "Lazy" | "lazy" => SystemType::Lazy,
        "Interval" | "interval" => SystemType::Interval,
        "Interact" | "interact" => SystemType::Interact,
        _ => improper_systy_fmt(),
    }
}

pub fn read_data(item: &Meta) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match item {
        Meta::List(items) => read_data_items(items.nested.iter()),
        _ => improper_data_fmt(),
    }
}

pub fn read_data_items<'a>(
    mut items: impl Iterator<Item = &'a NestedMeta>,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let (item0, item1) = match (items.next(), items.next(), items.next()) {
        (Some(item0), Some(item1), None) => (item0, item1),
        _ => improper_data_fmt(),
    };

    let comps = match item0 {
        NestedMeta::Literal(Lit::Str(path)) => path.value(),
        NestedMeta::Meta(Meta::Word(word)) => word.to_string(),
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            ident,
            lit: Lit::Str(path),
            ..
        })) if ident == "components" => path.value(),
        _ => improper_data_fmt(),
    };

    let servs = match item1 {
        NestedMeta::Literal(Lit::Str(path)) => path.value(),
        NestedMeta::Meta(Meta::Word(word)) => word.to_string(),
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            ident,
            lit: Lit::Str(path),
            ..
        })) if ident == "services" => path.value(),
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

fn improper_activated_fmt() -> ! {
    improper_attr_format("#[activated = ...]", "conniecs::system");
}

fn improper_reactivated_fmt() -> ! {
    improper_attr_format("#[reactivated = ...]", "conniecs::system");
}

fn improper_deactivated_fmt() -> ! {
    improper_attr_format("#[deactivated = ...]", "conniecs::system");
}
