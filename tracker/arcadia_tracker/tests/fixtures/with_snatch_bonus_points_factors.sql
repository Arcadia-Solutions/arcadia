-- Site-wide snatch bonus point factors: snatcher is charged 200% of the cost,
-- the receiver (uploader) gets 50% of the cost. Independent, so points are net burned.
UPDATE arcadia_settings SET
    global_snatch_bonus_points_cost_factor = 200,
    global_snatch_bonus_points_reward_factor = 50,
    snatched_torrent_bonus_points_transferred_to = 'uploader';
