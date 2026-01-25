-- Give test user bonus points for shop testing
UPDATE users SET bonus_points = 10000, uploaded = 0, freeleech_tokens = 0 WHERE id = 100;
