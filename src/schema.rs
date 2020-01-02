table! {
    shift_expansion (id) {
        id -> Int8,
        id_structure -> Int8,
        morning -> Bool,
        afternoon -> Bool,
        night -> Bool,
        rest -> Bool,
        prog -> Int2,
    }
}

table! {
    shift_structure (id) {
        id -> Int8,
        id_user -> Int8,
        day -> Date,
    }
}

table! {
    user (id) {
        id -> Int8,
        pwd -> Bpchar,
        email -> Varchar,
        last_login -> Nullable<Timestamp>,
        enable -> Bool,
        salt -> Bpchar,
        superuser -> Bool,
    }
}

joinable!(shift_expansion -> shift_structure (id_structure));
joinable!(shift_structure -> user (id_user));

allow_tables_to_appear_in_same_query!(
    shift_expansion,
    shift_structure,
    user,
);
