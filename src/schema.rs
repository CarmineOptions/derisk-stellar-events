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

diesel::table! {
  stellarx_pools (id) {
      id -> Text,
      type_field -> Text,
      fee_bp -> Int8,
      paging_token -> Text,
      total_shares -> Text,
      total_trustlines -> Text,
      last_modified_time -> Text,
      last_modified_ledger -> Int8,
      liquidity -> Nullable<Float8>,
      volume_24h -> Nullable<Float8>,
      volume_7d -> Nullable<Float8>,
      fee_24h -> Nullable<Float8>,
      fee_1y -> Nullable<Float8>,
      fee_aqua -> Nullable<Float8>,
      assets_order -> Text,
      asset_1_domain -> Text,
      asset_2_domain -> Text,
      asset_1 -> Nullable<Jsonb>,
      asset_2 -> Nullable<Jsonb>,
  }
}
