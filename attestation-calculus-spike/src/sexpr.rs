//! sexpr.rs — homoiconic mirror of `serde_json::Value` for the Attestation Calculus.
//!
//! `Sexpr` is a faithful, lossless relabeling of a JSON `Value`. `Num` wraps
//! `serde_json::Number` *verbatim* (not `f64`), which is the load-bearing choice:
//! it preserves the i64/u64/f64 tag and the exact digits, so `value_to_sexpr` /
//! `sexpr_to_value` form a structural isomorphism and the JCS round-trip is a
//! relabeling rather than a numeric re-encode.
//!
//! INVARIANT (proven by the test battery below):
//!   jcs_bytes(v) == jcs_bytes(&sexpr_to_value(&value_to_sexpr(v)))
//! for arbitrary JSON envelopes (ints, large u64, floats, negatives, nested,
//! unicode, arrays, empties).
//!
//! Crate deps: serde_json, serde_jcs, sha2, hex. NOTE: the `real-persist`
//! feature transitively unifies `serde_json/arbitrary_precision` into the build
//! graph (via ciris-persist), under which a `Number` may be string-backed and
//! hold a value with no finite IEEE-754 double image (e.g. `1e1000`, a 1000-digit
//! integer). [`jcs_bytes`] therefore normalizes every number to the RFC-8785
//! double model *before* canonicalizing and returns an honest `Err` on any
//! number that has no finite double — so a content address is always either the
//! spec-canonical hash or a hard error, never a silently non-canonical one, in
//! EITHER feature config.
//!
//! `print_sexpr` is a human-facing *display* rendering and is intentionally
//! non-injective (it sugars the `attestation_type` member into a bare operator
//! head). The canonical, reversible homoiconic representation is the pair
//! [`value_to_sexpr`]/[`sexpr_to_value`] together with [`jcs_bytes`]; a string
//! *reader* (parse → `Sexpr`) is deferred future work.

use serde_json::{Map, Number, Value};
use sha2::{Digest, Sha256};

/// Homoiconic mirror of `serde_json::Value`.
///
/// `Num` wraps `serde_json::Number` verbatim so the i64/u64/f64 tag and the exact
/// digits survive the round-trip. `Assoc` preserves member order as-read; key
/// ordering is irrelevant to canonical (JCS) bytes since RFC 8785 re-sorts keys
/// by UTF-16 code units.
#[derive(Clone, Debug, PartialEq)]
pub enum Sexpr {
    Null,
    Bool(bool),
    Num(Number),
    Str(String),
    List(Vec<Sexpr>),
    Assoc(Vec<(String, Sexpr)>),
}

/// Faithful mirror of a JSON value into an `Sexpr`.
///
/// Preserves `serde_json::Number` losslessly (clone, not re-encode) and the key
/// order as-read from the underlying `Map`.
pub fn value_to_sexpr(v: &Value) -> Sexpr {
    match v {
        Value::Null => Sexpr::Null,
        Value::Bool(b) => Sexpr::Bool(*b),
        Value::Number(n) => Sexpr::Num(n.clone()),
        Value::String(s) => Sexpr::Str(s.clone()),
        Value::Array(a) => Sexpr::List(a.iter().map(value_to_sexpr).collect()),
        Value::Object(o) => {
            Sexpr::Assoc(o.iter().map(|(k, val)| (k.clone(), value_to_sexpr(val))).collect())
        }
    }
}

/// Exact inverse of [`value_to_sexpr`].
///
/// NOTE: `serde_json::Map` dedupes keys (last write wins). This is safe for the
/// value->sexpr->value direction because a `Value` can never carry duplicate
/// keys; a homoiconic *reader* that builds `Assoc` by hand must reject duplicate
/// keys to stay injective (RFC 8785 likewise forbids duplicate keys).
pub fn sexpr_to_value(s: &Sexpr) -> Value {
    match s {
        Sexpr::Null => Value::Null,
        Sexpr::Bool(b) => Value::Bool(*b),
        Sexpr::Num(n) => Value::Number(n.clone()),
        Sexpr::Str(s) => Value::String(s.clone()),
        Sexpr::List(l) => Value::Array(l.iter().map(sexpr_to_value).collect()),
        Sexpr::Assoc(a) => {
            let mut m = Map::new();
            for (k, val) in a {
                m.insert(k.clone(), sexpr_to_value(val));
            }
            Value::Object(m)
        }
    }
}

/// Canonicalize a JSON value to RFC 8785 (JCS) bytes.
///
/// RFC 8785 models every number as an IEEE-754 double (the ECMAScript
/// `Number::toString` form). We coerce every `Number` to that model *before*
/// canonicalizing, which keeps `jcs_bytes` total-and-canonical regardless of
/// whether `serde_json/arbitrary_precision` is unified into the build graph
/// (it is, transitively, under the `real-persist` feature). A number with no
/// finite double image (e.g. a string-backed `1e1000` admitted under
/// arbitrary_precision) is rejected with an honest `Err` rather than emitted as
/// non-canonical bytes — so a content address is always either the spec-canonical
/// hash or a hard error, never a wrong-but-plausible one. On the default build
/// the coercion is a no-op on the resulting bytes (serde_jcs already models
/// numbers as doubles), so every existing canonical form is unchanged.
pub fn jcs_bytes(v: &Value) -> Result<Vec<u8>, String> {
    let normalized = to_double_model(v)?;
    serde_jcs::to_vec(&normalized).map_err(|e| e.to_string())
}

/// Deep-copy `v`, coercing every `Number` to its finite IEEE-754 double image
/// (the RFC 8785 / ECMAScript number model). Returns `Err` if any number has no
/// finite double — the guard that makes [`jcs_bytes`] total under
/// `arbitrary_precision`.
fn to_double_model(v: &Value) -> Result<Value, String> {
    match v {
        Value::Number(n) => {
            let f = n
                .as_f64()
                .filter(|x| x.is_finite())
                .ok_or_else(|| format!("number `{n}` has no finite IEEE-754 double image; not RFC-8785 representable"))?;
            let num = Number::from_f64(f)
                .ok_or_else(|| format!("number `{n}` is not representable as a JSON number after double coercion"))?;
            Ok(Value::Number(num))
        }
        Value::Array(a) => Ok(Value::Array(
            a.iter().map(to_double_model).collect::<Result<_, _>>()?,
        )),
        Value::Object(o) => {
            let mut m = Map::new();
            for (k, val) in o {
                m.insert(k.clone(), to_double_model(val)?);
            }
            Ok(Value::Object(m))
        }
        other => Ok(other.clone()),
    }
}

/// Pretty homoiconic **display** (non-injective — see module docs; the
/// canonical reversible form is [`value_to_sexpr`] + [`jcs_bytes`]).
///
/// * `List`  -> `(v1 v2 ...)`
/// * `Assoc` -> `(:k1 v1 :k2 v2 ...)`, UNLESS a member named `attestation_type`
///   exists, in which case its value leads as the bare operator head:
///   `(scores :dimension ... )`.
/// * Atoms: `Str` in double quotes (JSON-escaped); `Num`/`Bool`/`Null` literal.
/// * Keys are emitted bare (`:key`) only when they are symbol-safe; otherwise
///   they are quoted (`:"odd key"`) so the rendering is unambiguous.
pub fn print_sexpr(s: &Sexpr) -> String {
    match s {
        Sexpr::Null => "null".to_string(),
        Sexpr::Bool(b) => b.to_string(),
        Sexpr::Num(n) => n.to_string(),
        Sexpr::Str(s) => quote_str(s),
        Sexpr::List(items) => {
            let parts: Vec<String> = items.iter().map(print_sexpr).collect();
            format!("({})", parts.join(" "))
        }
        Sexpr::Assoc(members) => print_assoc(members),
    }
}

/// Render an `Assoc` body, leading with the `attestation_type` value as a bare
/// head when present.
fn print_assoc(members: &[(String, Sexpr)]) -> String {
    let head_idx = members.iter().position(|(k, _)| k == "attestation_type");

    let mut out = String::from("(");
    let mut first = true;

    if let Some(idx) = head_idx {
        out.push_str(&head_atom(&members[idx].1));
        first = false;
    }

    for (i, (k, v)) in members.iter().enumerate() {
        if Some(i) == head_idx {
            continue; // consumed into the head
        }
        if !first {
            out.push(' ');
        }
        first = false;
        out.push_str(&render_key(k));
        out.push(' ');
        out.push_str(&print_sexpr(v));
    }

    out.push(')');
    out
}

/// Render an `Sexpr` used as the head of an `Assoc`: a string head appears bare
/// (e.g. `scores`), anything else falls back to its normal printed form.
fn head_atom(s: &Sexpr) -> String {
    match s {
        Sexpr::Str(x) => x.clone(),
        other => print_sexpr(other),
    }
}

/// Render an `Assoc` key: bare `:key` when symbol-safe, else `:"quoted key"`.
/// Keeps the display unambiguous for keys containing spaces, delimiters, or that
/// are empty (JSON permits any string key).
fn render_key(k: &str) -> String {
    let bare_safe = !k.is_empty()
        && k.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | ':'));
    if bare_safe {
        format!(":{k}")
    } else {
        format!(":{}", quote_str(k))
    }
}

/// Double-quote a string with JSON-correct escaping.
fn quote_str(s: &str) -> String {
    // serde_json applies RFC-8259/8785-compatible minimal escaping.
    serde_json::to_string(s).unwrap_or_else(|_| format!("\"{}\"", s))
}

/// Content address = lowercase hex of SHA-256 over the canonical (JCS) bytes.
pub fn content_address(jcs: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(jcs);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Number, Value};

    /// The exact call ciris-verify-core makes; total on every constructible Value.
    fn jcs(v: &Value) -> Vec<u8> {
        jcs_bytes(v).expect("finite JSON canonicalizes")
    }

    fn roundtrip(v: &Value) -> Value {
        sexpr_to_value(&value_to_sexpr(v))
    }

    #[test]
    fn jcs_round_trip_bijection_holds_for_all_shapes() {
        // CEG-shaped scores envelope (the running example from the design memo §3.1).
        let ceg = json!({
            "schema": "ceg.scores.v1",
            "op": "scores",
            "dimension": "evaluation:quality",
            "score": 0.85,
            "confidence": 0.92,
            "evidence_refs": [],
            "tier": "federation",
            "weights": {"b": 1, "a": 2}
        });

        let battery: Vec<Value> = vec![
            json!(null),
            json!(true),
            json!(false),
            json!(0),
            json!(-1),
            json!(42),
            json!(i64::MIN),
            json!(u64::MAX),            // 18446744073709551615  (> 2^53)
            json!(9007199254740993u64), // 2^53 + 1
            json!(0.85),
            json!(1.0),  // ECMAScript form is "1"
            json!(100.0), // "100"
            json!(-0.0), // "0"
            json!(1e21), // "1e+21"
            json!(1e-7), // "1e-7"
            json!(3.141592653589793),
            json!(""),
            json!("hello"),
            json!("tab\tnewline\n\"quote\"\\backslash"),
            json!("unicode: \u{00e9}\u{4e2d}\u{1f600}"),
            json!([]),
            json!({}),
            json!([1, 2, [3, [4, [5]]]]),
            json!({"z": 1, "a": 2, "m": {"y": 3, "b": 4}}), // unsorted keys
            json!({"\u{00e9}": "e-acute", "z": "zed", "10": "ten", "1": "one"}),
            json!({"nested": {"a": [true, null, {"deep": 0.123}], "b": []}}),
            ceg,
        ];

        for (i, v) in battery.iter().enumerate() {
            assert_eq!(
                jcs(v),
                jcs(&roundtrip(v)),
                "case {i}: jcs_bytes(v) must equal jcs_bytes(sexpr_to_value(value_to_sexpr(v)))"
            );
            // Stronger: the mirror is a structural isomorphism, so Value itself round-trips.
            assert_eq!(*v, roundtrip(v), "case {i}: value round-trip must be deep-equal");
        }
    }

    #[test]
    fn jcs_canonical_forms_are_rfc8785() {
        let s = |v: &Value| String::from_utf8(jcs(v)).unwrap();
        // ECMAScript number canonicalization (ryu-js):
        assert_eq!(s(&json!(0.85)), "0.85");
        assert_eq!(s(&json!(1.0)), "1");
        assert_eq!(s(&json!(100.0)), "100");
        assert_eq!(s(&json!(-0.0)), "0");
        assert_eq!(s(&json!(1e21)), "1e+21");
        assert_eq!(s(&json!(1e-7)), "1e-7");
        // Integers > 2^53 are coerced to the nearest double (by-spec, both sides agree):
        assert_eq!(s(&json!(u64::MAX)), "18446744073709552000");
        assert_eq!(s(&json!(9007199254740993u64)), "9007199254740992");
        // Keys sorted by UTF-16 code units; input order is irrelevant:
        assert_eq!(s(&json!({"z":1,"a":2,"10":3,"1":4})), r#"{"1":4,"10":3,"a":2,"z":1}"#);
        // No Unicode normalization, raw UTF-8 strings, minimal escaping:
        assert_eq!(s(&json!("\u{4e2d}\u{1f600}")), "\"\u{4e2d}\u{1f600}\"");
        // Non-finite floats cannot even construct a Number, so they never reach JCS:
        assert!(Number::from_f64(f64::NAN).is_none());
        assert!(Number::from_f64(f64::INFINITY).is_none());
    }

    #[test]
    fn print_sexpr_renders_homoiconically() {
        // Atoms.
        assert_eq!(print_sexpr(&Sexpr::Null), "null");
        assert_eq!(print_sexpr(&Sexpr::Bool(true)), "true");
        assert_eq!(print_sexpr(&value_to_sexpr(&json!(42))), "42");
        assert_eq!(print_sexpr(&value_to_sexpr(&json!(0.85))), "0.85");
        assert_eq!(print_sexpr(&Sexpr::Str("hi".into())), "\"hi\"");

        // List.
        assert_eq!(print_sexpr(&value_to_sexpr(&json!([1, 2, 3]))), "(1 2 3)");

        // Assoc WITHOUT attestation_type -> generic ":k v" form (built directly
        // for deterministic order independent of Map iteration).
        let generic = Sexpr::Assoc(vec![
            ("a".into(), Sexpr::Num(Number::from(1))),
            ("b".into(), Sexpr::Str("x".into())),
        ]);
        assert_eq!(print_sexpr(&generic), "(:a 1 :b \"x\")");

        // Assoc WITH attestation_type -> value leads as bare head.
        let scores = Sexpr::Assoc(vec![
            ("attestation_type".into(), Sexpr::Str("scores".into())),
            ("dimension".into(), Sexpr::Str("evaluation:quality".into())),
            ("score".into(), Sexpr::Num(Number::from_f64(0.85).unwrap())),
        ]);
        assert_eq!(
            print_sexpr(&scores),
            "(scores :dimension \"evaluation:quality\" :score 0.85)"
        );

        // Empty Assoc and empty List.
        assert_eq!(print_sexpr(&Sexpr::Assoc(vec![])), "()");
        assert_eq!(print_sexpr(&Sexpr::List(vec![])), "()");
    }

    #[test]
    fn content_address_is_deterministic_hex_sha256() {
        let a = jcs(&json!({"attestation_type": "scores", "dimension": "q"}));
        let h1 = content_address(&a);
        let h2 = content_address(&a);
        assert_eq!(h1, h2, "content address must be deterministic");
        assert_eq!(h1.len(), 64, "sha256 hex is 64 chars");
        assert!(h1.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));

        // Distinct canonical bytes -> distinct address.
        let b = jcs(&json!({"attestation_type": "scores", "dimension": "r"}));
        assert_ne!(content_address(&a), content_address(&b));

        // Known vector: SHA-256 over the JCS of `{}` (the bytes `{}`).
        let empty = jcs(&json!({}));
        assert_eq!(empty, b"{}");
        assert_eq!(
            content_address(&empty),
            "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a"
        );
    }

    #[test]
    fn jcs_bytes_is_total_or_honest_error_under_arbitrary_precision() {
        // Default build: serde_json rejects out-of-range numeric literals at parse
        // time, so each `from_str` below is `Err` and the body is skipped (no-op).
        // `real-persist` build (arbitrary_precision unified on): the literal parses
        // to a string-backed Number with no finite IEEE-754 double image, and
        // `jcs_bytes` MUST reject it with `Err` rather than emit non-canonical bytes.
        let hazards = ["1e1000", "-1e1000", "1e400", &"9".repeat(400)];
        for lit in hazards {
            if let Ok(v) = serde_json::from_str::<Value>(lit) {
                assert!(
                    jcs_bytes(&v).is_err(),
                    "jcs_bytes must reject non-finite-double number `{lit}` under arbitrary_precision"
                );
            }
        }
        // Finite numbers are unaffected in either config.
        assert!(jcs_bytes(&json!({"score": 0.85, "n": u64::MAX})).is_ok());
    }

    #[test]
    fn print_sexpr_quotes_unsafe_keys() {
        // Keys with spaces / empty keys are quoted so the display is unambiguous;
        // symbol-safe keys stay bare.
        let odd = Sexpr::Assoc(vec![
            ("a b".into(), Sexpr::Num(Number::from(1))),
            ("".into(), Sexpr::Bool(true)),
            ("ok_key".into(), Sexpr::Null),
        ]);
        assert_eq!(print_sexpr(&odd), "(:\"a b\" 1 :\"\" true :ok_key null)");
    }
}
