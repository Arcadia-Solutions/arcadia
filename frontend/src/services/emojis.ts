import { config } from '@/config'

/**
 * URL of a custom emoji image. The version is the emoji's last edit time, which busts the
 * browser cache when staff replaces the image, since the response is cached for a year.
 */
export const emojiImageUrl = (emojiId: number, imageVersion: number): string => `${config.api_base_url}/api/emojis/${emojiId}/image?v=${imageVersion}`
