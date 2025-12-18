-- Donations tracking table
CREATE TABLE donations (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    amount FLOAT NOT NULL,
    currency VARCHAR(10) NOT NULL DEFAULT 'EUR',
    donor_name VARCHAR(255),
    user_id INT REFERENCES users(id) ON DELETE SET NULL,
    note TEXT NOT NULL DEFAULT '',
    created_by_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

-- Donation settings table
CREATE TABLE donation_settings (
    id INT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    donation_goal FLOAT NOT NULL DEFAULT 0,
    donation_goal_period VARCHAR(20) NOT NULL DEFAULT 'monthly'
);

-- Insert default settings
INSERT INTO donation_settings (donation_goal, donation_goal_period) VALUES (0, 'monthly');
