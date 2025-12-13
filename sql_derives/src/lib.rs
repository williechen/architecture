use proc_macro::TokenStream;
use quote::quote;
use syn::GenericArgument;
use syn::PathArguments;
use syn::{Data, DeriveInput, Type, parse_macro_input};

// CamelCase -> snake_case
fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            for c in ch.to_lowercase() {
                result.push(c);
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// 輔助函數：解析型別，回傳 (型別名稱字串, 是否為 Option)
/// 例如:
///   String -> ("String", false)
///   Option<i32> -> ("i32", true)
fn get_type_info(ty: &Type) -> (String, bool) {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                // 如果是 Option，解析角括號內的型別 <T>
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        // 遞迴呼叫或是直接取 inner type 的名稱
                        // 這裡簡化處理，直接取 inner type 的最後一個 segment
                        if let Type::Path(inner_path) = inner_ty {
                            if let Some(inner_seg) = inner_path.path.segments.last() {
                                return (inner_seg.ident.to_string(), true);
                            }
                        }
                    }
                }
            } else {
                // 不是 Option
                return (segment.ident.to_string(), false);
            }
        }
    }
    panic!("Unsupported field type parsing");
}

#[proc_macro_derive(SqlTable, attributes(sql))]
pub fn sql_table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident.clone();
    let struct_name_str = struct_name.to_string();
    let mut table_name_str = to_snake_case(&struct_name_str);

    // 解析 struct-level #[sql(table = "...")]
    for attr in &input.attrs {
        if attr.path().is_ident("sql") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("table") {
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    table_name_str = lit.value();
                    Ok(())
                } else {
                    Err(meta.error("Unknown sql attribute on struct"))
                }
            })
            .expect("Failed to parse sql attribute on struct");
        }
    }

    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => panic!("Only structs are supported"),
    };

    let mut column_names = Vec::new();
    let mut field_infos = Vec::new(); // 儲存 (type_name, is_option)
    let mut field_idents = Vec::new();

    for f in fields.iter() {
        let (type_name, is_option) = get_type_info(&f.ty);
        field_infos.push((type_name, is_option));

        let field_ident = f.ident.as_ref().unwrap();
        field_idents.push(field_ident.clone());

        let mut column_name = field_ident.to_string();

        for attr in &f.attrs {
            if attr.path().is_ident("sql") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("column") {
                        let lit: syn::LitStr = meta.value()?.parse()?;
                        column_name = lit.value();

                        Ok(())
                    } else {
                        Err(meta.error("Unknown sql attribute"))
                    }
                })
                .expect("Failed to parse sql attribute");
            }
        }

        column_names.push(column_name);
    }

    let columns_literal = column_names.join(", ");

    let values = field_infos
        .iter()
        .zip(field_idents.iter())
        .map(|((ty, is_opt), field)| {
            let needs_quotes = matches!(
                ty.as_str(),
                "String" | "&str" | "NaiveDateTime" | "NaiveDate"
            );

            if *is_opt {
                // 如果是 Option
                if needs_quotes {
                    quote! {
                        match &self.#field {
                            Some(v) => format!("'{}'", v),
                            None => "NULL".to_string()
                        }
                    }
                } else {
                    quote! {
                        match &self.#field {
                            Some(v) => format!("{}", v),
                            None => "NULL".to_string()
                        }
                    }
                }
            } else {
                // 如果不是 Option (原本的邏輯)
                if needs_quotes {
                    quote! { format!("'{}'", self.#field) }
                } else {
                    quote! { format!("{}", self.#field) }
                }
            }
        });

    let sets = column_names
        .iter()
        .zip(field_infos.iter())
        .zip(field_idents.iter())
        .map(|((col, (ty, is_opt)), field)| {
            let needs_quotes = matches!(
                ty.as_str(),
                "String" | "&str" | "NaiveDateTime" | "NaiveDate"
            );

            if *is_opt {
                // 如果是 Option
                if needs_quotes {
                    quote! {
                        match &self.#field {
                            Some(v) => Some(format!("{}='{}'", #col, v)),
                            None => None
                        }
                    }
                } else {
                    quote! {
                        match &self.#field {
                            Some(v) => Some(format!("{}={}", #col, v)),
                            None => None
                        }
                    }
                }
            } else {
                // 如果不是 Option (原本的邏輯)
                if needs_quotes {
                    quote! { Some(format!("{}='{}'", #col, self.#field)) }
                } else {
                    quote! { Some(format!("{}={}", #col, self.#field)) }
                }
            }
        });

    let columns_list = column_names.clone();
    let _fields_list = field_idents.clone();

    let expanded = quote! {
        impl #struct_name {
            pub fn table_name() -> &'static str {
                #table_name_str
            }

            pub fn columns() -> Vec<&'static str> {
                vec![ #( #columns_list ),* ]
            }

            pub fn select_sql(where_clause: Option<&str>) -> String {
                match where_clause {
                    Some(cond) => format!("SELECT {} FROM {} WHERE {}", #columns_literal, Self::table_name(), cond),
                    None => format!("SELECT {} FROM {} WHERE 1=1 ", #columns_literal, Self::table_name()),
                }
            }

            pub fn insert_sql(&self) -> String {
                let values_vec = vec![ #( #values ),* ].join(", ");
                format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    Self::table_name(),
                    #columns_literal,
                    values_vec
                )
            }

            pub fn update_sql(&self, where_clause: Option<&str>) -> String {
                // 1. 收集所有欄位的 Option<String>
                let sets_options: Vec<Option<String>> = vec![ #( #sets ),* ];

                // 2. 過濾掉 None，只留下要更新的欄位
                let sets_vec: Vec<String> = sets_options.into_iter().filter_map(|x| x).collect();

                // 3. 如果沒有任何欄位需要更新 (例如全都是 None)，這裡回傳空字串或者可根據需求噴錯
                if sets_vec.is_empty() {
                    return String::new();
                }

                let sets_str = sets_vec.join(", ");

                match where_clause {
                    Some(cond) => format!("UPDATE {} SET {} WHERE {}", Self::table_name(), sets_str, cond),
                    None => format!("UPDATE {} SET {} WHERE 1=1 ", Self::table_name(), sets_str),
                }
            }

            pub fn delete_sql(where_clause: Option<&str>) -> String {
                match where_clause {
                    Some(cond) => format!("DELETE FROM {} WHERE {}", Self::table_name(), cond),
                    None => format!("DELETE FROM {} WHERE 1=1 ", Self::table_name()),
                }
            }

            // 單條件
            pub fn where_eq(field: &str, value: &str) -> String {
                format!("{}={}", field, value)
            }

            // 多條件 AND
            pub fn where_and(conditions: Vec<String>) -> String {
                format!("({})", conditions.join(" AND "))
            }

            // 多條件 OR
            pub fn where_or(conditions: Vec<String>) -> String {
                format!("({})", conditions.join(" OR "))
            }
        }
    };

    TokenStream::from(expanded)
}
