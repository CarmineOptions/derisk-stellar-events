diesel::table! {
  events (id) {
      id -> Text,
      contract_id -> Text,
      index -> Int4,
      sequence -> Int8,
      from_address -> Text,
      topics -> Jsonb,
      data -> Jsonb,
  }
}
