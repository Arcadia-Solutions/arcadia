-- Give test users some bonus points and freeleech tokens for gift testing
UPDATE users SET bonus_points = 1000, freeleech_tokens = 10 WHERE id = 100;
UPDATE users SET bonus_points = 500, freeleech_tokens = 5 WHERE id = 101;
