-- FieldForge Demo Seed Data
-- Run with: psql $DATABASE_URL -f apps/api/src/db/seed.sql

BEGIN;

-- Team
INSERT INTO teams (id, name, slug, industry, phone, email, timezone)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'Apex HVAC & Plumbing',
    'apex-hvac',
    'hvac',
    '(555) 123-4567',
    'office@apexhvac.com',
    'America/New_York'
) ON CONFLICT (id) DO NOTHING;

-- Users (password: "password123" hashed with argon2)
INSERT INTO users (id, team_id, email, first_name, last_name, role, password_hash)
VALUES
    ('00000000-0000-0000-0000-000000000010', '00000000-0000-0000-0000-000000000001', 'mike@apexhvac.com', 'Mike', 'Johnson', 'owner', '$argon2id$v=19$m=19456,t=2,p=1$placeholder_hash_for_demo'),
    ('00000000-0000-0000-0000-000000000011', '00000000-0000-0000-0000-000000000001', 'sarah@apexhvac.com', 'Sarah', 'Chen', 'admin', '$argon2id$v=19$m=19456,t=2,p=1$placeholder_hash_for_demo'),
    ('00000000-0000-0000-0000-000000000012', '00000000-0000-0000-0000-000000000001', 'james@apexhvac.com', 'James', 'Williams', 'technician', '$argon2id$v=19$m=19456,t=2,p=1$placeholder_hash_for_demo'),
    ('00000000-0000-0000-0000-000000000013', '00000000-0000-0000-0000-000000000001', 'lisa@apexhvac.com', 'Lisa', 'Rodriguez', 'technician', '$argon2id$v=19$m=19456,t=2,p=1$placeholder_hash_for_demo')
ON CONFLICT (id) DO NOTHING;

-- Customers
INSERT INTO customers (id, team_id, first_name, last_name, email, phone, company_name, customer_type, source)
VALUES
    ('00000000-0000-0000-0000-000000000020', '00000000-0000-0000-0000-000000000001', 'Tom', 'Williams', 'tom@example.com', '(555) 234-5678', NULL, 'residential', 'referral'),
    ('00000000-0000-0000-0000-000000000021', '00000000-0000-0000-0000-000000000001', 'Sarah', 'Johnson', 'sarah.j@example.com', '(555) 345-6789', NULL, 'residential', 'website'),
    ('00000000-0000-0000-0000-000000000022', '00000000-0000-0000-0000-000000000001', 'David', 'Park', 'david@parkproperties.com', '(555) 456-7890', 'Park Properties LLC', 'commercial', 'google'),
    ('00000000-0000-0000-0000-000000000023', '00000000-0000-0000-0000-000000000001', 'Maria', 'Garcia', 'maria@example.com', '(555) 567-8901', NULL, 'residential', 'referral'),
    ('00000000-0000-0000-0000-000000000024', '00000000-0000-0000-0000-000000000001', 'Robert', 'Kim', 'robert@kimrestaurants.com', '(555) 678-9012', 'Kim Restaurants Inc', 'commercial', 'website')
ON CONFLICT (id) DO NOTHING;

-- Properties
INSERT INTO properties (id, team_id, customer_id, name, property_type, address_line1, city, state, zip_code)
VALUES
    ('00000000-0000-0000-0000-000000000030', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000020', 'Home', 'residential', '123 Oak Street', 'Springfield', 'IL', '62701'),
    ('00000000-0000-0000-0000-000000000031', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000021', 'Home', 'residential', '456 Maple Ave', 'Springfield', 'IL', '62702'),
    ('00000000-0000-0000-0000-000000000032', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000022', 'Office Building A', 'commercial', '789 Business Blvd', 'Springfield', 'IL', '62703'),
    ('00000000-0000-0000-0000-000000000033', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000024', 'Downtown Restaurant', 'commercial', '321 Main St', 'Springfield', 'IL', '62704')
ON CONFLICT (id) DO NOTHING;

-- Jobs
INSERT INTO jobs (id, team_id, customer_id, property_id, title, description, status, priority, trade, scheduled_start, scheduled_end, assigned_to)
VALUES
    ('00000000-0000-0000-0000-000000000040', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000020', '00000000-0000-0000-0000-000000000030', 'AC Unit Replacement', 'Replace aging 3-ton AC unit with new high-efficiency model', 'scheduled', 'high', 'hvac', NOW() + INTERVAL '1 day', NOW() + INTERVAL '1 day 4 hours', '00000000-0000-0000-0000-000000000012'),
    ('00000000-0000-0000-0000-000000000041', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000021', '00000000-0000-0000-0000-000000000031', 'Furnace Maintenance', 'Annual furnace inspection and tune-up', 'in_progress', 'medium', 'hvac', NOW(), NOW() + INTERVAL '2 hours', '00000000-0000-0000-0000-000000000013'),
    ('00000000-0000-0000-0000-000000000042', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000022', '00000000-0000-0000-0000-000000000032', 'Commercial HVAC Overhaul', 'Full system inspection and repair for office building', 'estimated', 'high', 'hvac', NOW() + INTERVAL '3 days', NOW() + INTERVAL '3 days 8 hours', '00000000-0000-0000-0000-000000000012'),
    ('00000000-0000-0000-0000-000000000043', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000023', NULL, 'Water Heater Install', 'Install new tankless water heater', 'lead', 'medium', 'plumbing', NULL, NULL, NULL),
    ('00000000-0000-0000-0000-000000000044', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000024', '00000000-0000-0000-0000-000000000033', 'Kitchen Exhaust Repair', 'Repair commercial kitchen exhaust fan and ductwork', 'completed', 'high', 'hvac', NOW() - INTERVAL '2 days', NOW() - INTERVAL '2 days' + INTERVAL '6 hours', '00000000-0000-0000-0000-000000000013')
ON CONFLICT (id) DO NOTHING;

-- Estimates
INSERT INTO estimates (id, team_id, customer_id, job_id, estimate_number, title, status, subtotal, tax_rate, tax_amount, total, valid_until)
VALUES
    ('00000000-0000-0000-0000-000000000050', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000020', '00000000-0000-0000-0000-000000000040', 'EST-001', 'AC Unit Replacement', 'approved', 4500.00, 8.25, 371.25, 4871.25, NOW() + INTERVAL '30 days'),
    ('00000000-0000-0000-0000-000000000051', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000022', '00000000-0000-0000-0000-000000000042', 'EST-002', 'Commercial HVAC Overhaul', 'sent', 12800.00, 8.25, 1056.00, 13856.00, NOW() + INTERVAL '30 days'),
    ('00000000-0000-0000-0000-000000000052', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000023', '00000000-0000-0000-0000-000000000043', 'EST-003', 'Water Heater Install', 'draft', 2200.00, 8.25, 181.50, 2381.50, NOW() + INTERVAL '30 days')
ON CONFLICT (id) DO NOTHING;

-- Invoices
INSERT INTO invoices (id, team_id, customer_id, job_id, invoice_number, status, subtotal, tax_rate, tax_amount, total, due_date)
VALUES
    ('00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000024', '00000000-0000-0000-0000-000000000044', 'INV-001', 'paid', 3200.00, 8.25, 264.00, 3464.00, NOW() - INTERVAL '5 days'),
    ('00000000-0000-0000-0000-000000000061', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000021', '00000000-0000-0000-0000-000000000041', 'INV-002', 'sent', 275.00, 8.25, 22.69, 297.69, NOW() + INTERVAL '15 days'),
    ('00000000-0000-0000-0000-000000000062', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000020', '00000000-0000-0000-0000-000000000040', 'INV-003', 'draft', 4500.00, 8.25, 371.25, 4871.25, NOW() + INTERVAL '30 days')
ON CONFLICT (id) DO NOTHING;

-- Inventory Items
INSERT INTO inventory_items (id, team_id, name, sku, category, quantity, min_quantity, unit, unit_cost, unit_price)
VALUES
    ('00000000-0000-0000-0000-000000000070', '00000000-0000-0000-0000-000000000001', 'Copper Pipe 3/4"', 'COP-075', 'Piping', 150, 25, 'feet', 3.50, 7.00),
    ('00000000-0000-0000-0000-000000000071', '00000000-0000-0000-0000-000000000001', 'R-410A Refrigerant', 'REF-410', 'Refrigerant', 12, 5, 'cans', 45.00, 85.00),
    ('00000000-0000-0000-0000-000000000072', '00000000-0000-0000-0000-000000000001', 'Air Filter 20x25x1', 'FLT-2025', 'Filters', 48, 20, 'units', 4.50, 12.00),
    ('00000000-0000-0000-0000-000000000073', '00000000-0000-0000-0000-000000000001', 'Thermostat - Smart WiFi', 'THR-WIFI', 'Controls', 8, 3, 'units', 85.00, 175.00),
    ('00000000-0000-0000-0000-000000000074', '00000000-0000-0000-0000-000000000001', 'PVC Pipe 2"', 'PVC-200', 'Piping', 3, 10, 'feet', 2.25, 5.50)
ON CONFLICT (id) DO NOTHING;

-- Vehicles
INSERT INTO vehicles (id, team_id, name, make, model, year, license_plate, assigned_to, status, mileage)
VALUES
    ('00000000-0000-0000-0000-000000000080', '00000000-0000-0000-0000-000000000001', 'Service Van 1', 'Ford', 'Transit 250', 2022, 'IL-APEX01', '00000000-0000-0000-0000-000000000012', 'active', 34500),
    ('00000000-0000-0000-0000-000000000081', '00000000-0000-0000-0000-000000000001', 'Service Van 2', 'Chevrolet', 'Express 2500', 2021, 'IL-APEX02', '00000000-0000-0000-0000-000000000013', 'active', 52100),
    ('00000000-0000-0000-0000-000000000082', '00000000-0000-0000-0000-000000000001', 'Utility Truck', 'RAM', '3500', 2020, 'IL-APEX03', NULL, 'maintenance', 78200)
ON CONFLICT (id) DO NOTHING;

COMMIT;
