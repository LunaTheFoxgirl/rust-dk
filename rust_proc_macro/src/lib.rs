use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Prob" => "Err",
        "Ok" => "Ok",
        "Streng" => "String",
        "Ordbog" => "HashMap",
        "Standard" => "Default",
        "Fejl" => "Error",
        "Valg" => "Option",
        "Nogle" => "Some",
        "Ingen" => "None",
        "Resultat" => "Result",
        "Selv" => "Self",
        "skrivlinje" => "println",
        "knæk" => "break",
        "asynkront" => "async",
        "afvent" => "await",
        "løkke" => "loop",
        "flyt" => "move",
        "kasse" => "crate",
        "utilgængelig_kode" => "unreachable_code",
        "som" => "as",
        "constant" => "const",
        "kedetegn" => "trait",
        "usikker" => "unsafe",
        "i" => "in",
        "fra" => "from",
        "dynamisk" => "dyn",
        "udpak" => "unwrap",
        "standard" => "default",
        "som_ref" => "as_ref",
        "io" => "io",
        "eksternt" => "extern",
        "falsk" => "false",
        "funktion" => "fn",
        "prima" => "super",
        "indsæt" => "insert",
        "hent" => "get",
        "tillad" => "allow",
        "lort" | "møg" | "oops" => "panic",
        "modul" => "mod",
        "forandrelig" => "mut",
        "ny" => "new",
        "hvor" => "where",
        "for" => "for",
        "hent_eller_indsæt_med" => "get_or_insert_with",
        "hoved" => "main",
        "offentlig" => "pub",
        "ingen" => None?,
        "returner" => "return",
        "implementer" => "impl",
        "ref" => "ref",
        "sammenlign" => "match",
        "hvis" => "if",
        "ellers" => "else",
        "selv" => "self",
        "lad" => "let",
        "statisk" => "static",
        "struktur" => "struct",
        "forvent" => "expect",
        "imens" => "while",
        "brug" => "use",
        "ind" => "into",
        "sandt" => "true",
        "enumeration" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rust(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
