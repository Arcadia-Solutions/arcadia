import api from './api.ts'

export const removeArtistAffiliations = async (affiliationIds: number[]) => {
  const params = new URLSearchParams()
  affiliationIds.forEach((id) => {
    params.append('affiliation_ids', id.toString())
  })
  return (await api.delete(`/affiliated-artists?${params.toString()}`)).data
}
