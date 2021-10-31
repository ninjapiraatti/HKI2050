table! {
    characters (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
    }
}

table! {
	invitations (id) {
		id -> Uuid,
		email -> Varchar,
		username -> Varchar,
		password_plain -> Nullable<Varchar>,
		expires_at -> Timestamp,
		reset_request_id -> Nullable<Uuid>,
		updated_by -> Varchar,
	}
}

table! {
    reset_requests (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        expire_at -> Timestamp,
        updated_by -> Varchar,
    }
}

table! {
	activesessions (session_id) {
		session_id -> Uuid,
		user_id -> Uuid,
		email -> Varchar,
		expire_at -> Timestamp,
		isadmin -> Bool,
	}
}

table! {
    users (id) {
        id -> Uuid,
        isadmin -> Bool,
        email -> Varchar,
        username -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(invitations -> reset_requests (reset_request_id));
joinable!(sessions -> users (user_id));
joinable!(characters -> users (user_id));

allow_tables_to_appear_in_same_query!(
    invitations,
    reset_requests,
    sessions,
    users,
    characters,
);
