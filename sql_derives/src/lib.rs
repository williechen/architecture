use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

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
    let table_name_str = to_snake_case(&struct_name_str);

    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => panic!("Only structs are supported"),
    };

    let mut column_names = Vec::new();
    let mut field_idents = Vec::new();

    for f in fields.iter() {
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
    let sets = column_names
        .iter()
        .zip(field_idents.iter())
        .map(|(col, field)| {
            quote! { format!("{}='{}'", #col, self.#field) }
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

            pub fn select_sql() -> String {
                format!("SELECT {} FROM {}", #columns_literal, Self::table_name())
            }

            pub fn insert_sql(&self) -> String {
                let values = vec![ #( format!("'{}'", self.#fields_list) ),* ];
                format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    Self::table_name(),
                    #columns_literal,
                    values.join(", ")
                )
            }

            pub fn update_sql(&self, where_clause: Option<&str>) -> String {
                let sets_vec = vec![ #( #sets ),* ].join(", ");
                match where_clause {
                    Some(cond) => format!("UPDATE {} SET {} WHERE {}", Self::table_name(), sets_vec, cond),
                    None => format!("UPDATE {} SET {}", Self::table_name(), sets_vec),
                }
            }

            // 單條件
            pub fn where_eq(field: &str, value: &str) -> String {
                format!("{}='{}'", field, value)
            }

            // 多條件 AND
            pub fn where_and(conditions: Vec<String>) -> String {
                conditions.join(" AND ")
            }

            // 多條件 OR
            pub fn where_or(conditions: Vec<String>) -> String {
                conditions.join(" OR ")
            }
        }
    };

    TokenStream::from(expanded)
}
