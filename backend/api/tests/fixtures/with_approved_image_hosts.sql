-- Set approved image hosts for testing (just hostnames, no protocol)
UPDATE arcadia_settings SET approved_image_hosts = ARRAY['i.imgur.com', 'upload.wikimedia.org'];
