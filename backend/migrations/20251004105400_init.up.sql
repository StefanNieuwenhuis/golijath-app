-- Add up migration script here
-- Table: places
CREATE TABLE IF NOT EXISTS places (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION
);

-- Table: archives
CREATE TABLE IF NOT EXISTS archives (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- Table: institutes
CREATE TABLE IF NOT EXISTS institutes (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- Table: documents
CREATE TABLE IF NOT EXISTS documents (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    inventory_number TEXT NOT NULL UNIQUE,
    scan_number TEXT,
    page_number TEXT,
    notes TEXT,
    archive_id INT NOT NULL REFERENCES archives(id) ON DELETE RESTRICT,
    institute_id INT NOT NULL REFERENCES institutes(id) ON DELETE RESTRICT,
    place_id INT NOT NULL REFERENCES places(id) ON DELETE RESTRICT
);

-- Table: debug_log
CREATE TABLE IF NOT EXISTS debug_log (
    id SERIAL PRIMARY KEY,
    message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);