use proc_macro::TokenStream;
use quote::quote;
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
    let mut field_types = Vec::new();
    let mut field_idents = Vec::new();

    for f in fields.iter() {
        let field_type = match &f.ty {
            Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.clone(),
            _ => panic!("Unsupported field type"),
        };
        field_types.push(field_type.clone());

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

    let values = field_types
        .iter()
        .zip(field_idents.iter())
        .map(|(ty, field)| {
            if ty == "String" || ty == "&str" || ty == "NaiveDateTime" || ty == "NaiveDate" {
                quote! { format!("'{}'", self.#field) }
            } else {
                quote! { format!("{}", self.#field) }
            }
        });

    let sets = column_names
        .iter()
        .zip(field_types.iter())
        .zip(field_idents.iter())
        .map(|((col, ty), field)| {
            if ty == "String" || ty == "&str" || ty == "NaiveDateTime" || ty == "NaiveDate" {
                quote! { format!("{}='{}'", #col, self.#field) }
            } else {
                quote! { format!("{}={}", #col, self.#field) }
            }
        });

    let columns_list = column_names.clone();
    let fields_list = field_idents.clone();

    let expanded = quote! {
        impl #struct_name {
            pub fn table_name() -> &'static str {
                #table_name_str
            }

            pub fn columns() -> Vec<&'static str> {
                vec![ #( #columns_list ),* ]
            }

            pub fn select_sql(&self, where_clause: Option<&str>) -> String {
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
                let sets_vec = vec![ #( #sets ),* ].join(", ");
                match where_clause {
                    Some(cond) => format!("UPDATE {} SET {} WHERE {}", Self::table_name(), sets_vec, cond),
                    None => format!("UPDATE {} SET {} WHERE 1=1 ", Self::table_name(), sets_vec),
                }
            }

            pub fn delete_sql(&self, where_clause: Option<&str>) -> String {
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
