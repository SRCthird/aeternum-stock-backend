// @generated automatically by Diesel CLI.

diesel::table! {
    inventory (id) {
        id -> Integer,
        #[max_length = 191]
        lot_number -> Varchar,
        #[max_length = 191]
        location -> Varchar,
        quantity -> Integer,
        created_at -> Datetime,
        #[max_length = 191]
        created_by -> Varchar,
        updated_at -> Nullable<Datetime>,
        #[max_length = 191]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    inventorybay (id) {
        id -> Integer,
        #[max_length = 191]
        name -> Varchar,
        #[max_length = 191]
        warehouse_name -> Varchar,
        max_unique_lots -> Integer,
    }
}

diesel::table! {
    log (id) {
        id -> Integer,
        #[max_length = 191]
        from_location -> Varchar,
        #[max_length = 191]
        to_location -> Varchar,
        date_time -> Datetime,
        #[max_length = 191]
        user -> Varchar,
        #[max_length = 191]
        lot_number -> Varchar,
        quantity_moved -> Integer,
        #[max_length = 191]
        comments -> Varchar,
    }
}

diesel::table! {
    product (id) {
        id -> Integer,
        #[max_length = 191]
        name -> Varchar,
        #[max_length = 191]
        description -> Varchar,
    }
}

diesel::table! {
    productlot (id) {
        id -> Integer,
        #[max_length = 191]
        lot_number -> Varchar,
        #[max_length = 191]
        internal_reference -> Varchar,
        #[max_length = 191]
        product_name -> Varchar,
        quantity -> Integer,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        #[max_length = 191]
        email -> Varchar,
        #[max_length = 64]
        password -> Varchar,
        #[max_length = 191]
        role -> Varchar,
        #[max_length = 191]
        position -> Nullable<Varchar>,
        #[max_length = 191]
        first_name -> Nullable<Varchar>,
        #[max_length = 191]
        last_name -> Nullable<Varchar>,
        #[max_length = 191]
        bio -> Nullable<Varchar>,
        #[max_length = 191]
        image -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    warehouse (id) {
        id -> Integer,
        #[max_length = 191]
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    inventory,
    inventorybay,
    log,
    product,
    productlot,
    user,
    warehouse,
);
