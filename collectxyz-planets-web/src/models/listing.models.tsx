export interface ListingResponse {
  listing_id: string
  lister_xyz_id: string
  price_rmi: string
  deposit_rmi_denom: string
  deposit_rmi_amount: string
  created_at: string
  active_at: string
  expired_at: string
  resources: Array<{
    id: string
    amount: string
  }>
}
