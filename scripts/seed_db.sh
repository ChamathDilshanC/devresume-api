#!/bin/bash
echo "Seeding DevResume AI Database..."
psql postgres://devresume_user:devresume_password@localhost:5432/devresume_db -f migrations/0001_initial_schema.sql
psql postgres://devresume_user:devresume_password@localhost:5432/devresume_db -f migrations/0002_enterprise_schema.sql
echo "Database Seeding Completed Successfully."
