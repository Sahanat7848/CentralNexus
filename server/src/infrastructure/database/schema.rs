// @generated automatically by Diesel CLI.

diesel::table! {
    Cen (id) {
        id -> Int8,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    brawlers (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 50]
        display_name -> Varchar,
        #[max_length = 512]
        avatar_url -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_public_id -> Nullable<Varchar>,
        name_updated_at -> Timestamp,
        #[max_length = 4]
        tag -> Varchar,
    }
}

diesel::table! {
    crew_memberships (mission_id, brawler_id) {
        mission_id -> Int4,
        brawler_id -> Int4,
        joined_at -> Timestamp,
    }
}

diesel::table! {
    friendships (brawler_id, friend_id) {
        brawler_id -> Int4,
        friend_id -> Int4,
        #[max_length = 20]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        sender_id -> Int4,
        receiver_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        read_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    missions (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        status -> Varchar,
        chief_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(crew_memberships -> brawlers (brawler_id));
diesel::joinable!(crew_memberships -> missions (mission_id));
diesel::joinable!(missions -> brawlers (chief_id));

diesel::allow_tables_to_appear_in_same_query!(
    Cen,
    brawlers,
    crew_memberships,
    friendships,
    messages,
    missions,
);
